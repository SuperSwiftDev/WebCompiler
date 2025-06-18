use std::collections::{BTreeSet, VecDeque};
use std::path::Path;
use std::str::FromStr;
use url::Url;
use web_client_bot::WebClient;

use crate::crawler::db::{CanonicalUrlString, FailedUrlError, OriginalUrlString, PageEntry};
use crate::crawler::db::Database;
use crate::crawler::db::ProjectContext;

#[derive(Debug, Clone, Default)]
pub struct FilterSettings {
    pub domain_whitelist: BTreeSet<String>,
    pub protocol_whitelist: BTreeSet<String>,
    pub protocol_blacklist: BTreeSet<String>,
}

impl FilterSettings {
    pub fn with_whitelisted_domain(mut self, entry: impl Into<String>) -> Self {
        self.domain_whitelist.insert(entry.into());
        self
    }
    pub fn with_whitelisted_protocol(mut self, entry: impl Into<String>) -> Self {
        self.protocol_whitelist.insert(entry.into());
        self
    }
    pub fn with_blacklisted_protocol(mut self, entry: impl Into<String>) -> Self {
        self.protocol_blacklist.insert(entry.into());
        self
    }
    pub fn should_visit(&self, url: &Url) -> bool {
        if url.scheme().to_ascii_lowercase() == "tel" {
            return false
        }
        if url.scheme() != "http" && url.scheme() != "https" {
            eprintln!("ⓘ Skipping {:?}: NON-HTTP/S PROTOCOL", url.to_string());
            return false;
        }
        let url_domain = match url.domain() {
            None => {
                eprintln!("ⓘ Skipping: {:?}", url.to_string());
                return false
            }
            Some(x) => x,
        };
        let valid_domain = self.domain_whitelist.contains(url_domain);
        let keep = valid_domain;
        if !keep {
            eprintln!("ⓘ Skipping {:?}: Domain {url_domain:?}", url.to_string());
            return false
        }
        keep
    }
}

pub async fn process_project_spec(project: &crate::manifest::specification::ProjectSpec) {
    let mut filter_settings = FilterSettings::default();
    for domain in project.whitelist.domains.iter() {
        filter_settings = filter_settings.with_whitelisted_domain(domain);
    }
    for protocol in project.blacklist.protocols.iter() {
        filter_settings = filter_settings.with_blacklisted_protocol(protocol);
    }

    let input_url = &project.seed_url;
    let output_dir = &project.output_dir;

    crawl_site( input_url, output_dir, &filter_settings ).await
}

pub async fn crawl_site(
    seed_url: impl AsRef<str>,
    output_dir: impl AsRef<Path>,
    filter_settings: &FilterSettings,
) {
    let seed_url = Url::from_str(seed_url.as_ref()).unwrap();
    
    let output_dir = output_dir.as_ref();

    std::fs::create_dir_all(output_dir).unwrap();

    let project_context = ProjectContext {
        output_dir: output_dir.to_path_buf(),
    };
    
    // - -
    let mut queue = VecDeque::<CrawlTask>::from_iter(vec![
        CrawlTask::new(seed_url),
    ]);

    let mut database = Database::load(&output_dir).unwrap_or_default();

    // - -
    let mut client: WebClient = WebClient::start().await;

    let mut context = Context {
        project_context: &project_context,
        client: &mut client,
        database: &mut database,
    };

    // - -
    while let Some(task) = queue.pop_front() {
        context.process_task(task, &mut queue, filter_settings).await;
    }

    // - -
    client.close().await;
    database.save(&output_dir).unwrap();
}

struct Context<'a> {
    project_context: &'a ProjectContext,
    client: &'a mut WebClient,
    database: &'a mut Database,
}

impl<'a> Context<'a> {
    async fn process_task(
        &mut self,
        task: CrawlTask,
        queue: &mut VecDeque<CrawlTask>,
        filter_settings: &FilterSettings,
    ) {
        let local_shanpshot_path_rel = super::db::build_rel_html_snapshot_file_path(task.url.as_str()).unwrap();
        let canonical_url = task.canonicalize_url();
        let canonical_url_string = CanonicalUrlString(canonical_url.to_string());
        // - CHECK & SKIP EARLY -
        if let Some(entry) = self.database.lookup(&canonical_url_string) {
            let snapshot_path = entry.local_shanpshot_path_rel.resolve_full_snapshot_path(self.project_context);
            if snapshot_path.exists() {
                return
            }
        }
        // - CHECK OUTPUT FILE PATH -
        {
            let snapshot_file = local_shanpshot_path_rel.resolve_full_snapshot_path(self.project_context);
            if snapshot_file.exists() {
                eprintln!("ⓘ Skipping {:?} : SNAPSHOT FILE EXISTS", task.url.to_string());
                return
            }
        }
        // - CHECK IF ALREADY TRIED -
        if self.database.has_failed_url(&OriginalUrlString(task.url.to_string())) {
            return
        }

        // - PROCEED -

        // - LOG -
        eprintln!("🔎 Visiting: {}", canonical_url);

        // - -
        // let tab = self.client.open_new_tab_at_url(canonical_url.as_str()).await;
        let tab = self.client.open_new_tab_at_url_with_network_tracking(canonical_url.as_str()).await;
        let status_code = tab.status_code();

        if status_code != Some(200) {
            eprintln!("❌ Skipping {:?} : Request failed STATUS={status_code:?}", task.url.to_string());
            self.database.add_failed_url(OriginalUrlString(task.url.to_string()), FailedUrlError::HttpError {
                // url: OriginalUrlString(task.url.to_string()),
                status: status_code,
            });
            tab.close().await;
            return
        }


        tab.wait_for_navigation().await;
        
        match tab.is_text_html_document().await {
            Ok(true) => (),
            _ => {
                eprintln!("ⓘ Skipping {:?} : NOT HTML DOCUMENT", task.url.to_string());
                tab.close().await;
                return
            }
        };

        // - -
        if let Err(e) = tab.wait_until_fully_settled().await {
            eprintln!("❌ Failed to settle: {} | {e}", canonical_url.as_str());
            tab.close().await;
            return
        }

        // - DOM LOADED -
        let html = tab.html_content().await;
        let outgoing_anchors = tab.scrape_all_anchor_links().await;
        let outgoing_links = outgoing_anchors
            .iter()
            .map(|x| OriginalUrlString(x.href.clone()))
            .collect::<Vec<_>>();

        // - METADATA -
        let outgoing_internal_urls = outgoing_anchors
            .iter()
            .map(|link| link.href.as_str())
            .filter_map(|link| {
                match Url::from_str(link) {
                    Ok(x) => Some(x),
                    Err(error) => {
                        eprintln!("⚠️ Failed to parse link: {link} | {error}");
                        None
                    }
                }
            })
            .filter(|url| {
                filter_settings.should_visit(url)
            })
            .collect::<Vec<_>>();
        
        let enqueue_tasks = outgoing_internal_urls
            .clone()
            .into_iter()
            .map(|url| CrawlTask::new(url))
            .filter(|task| {
                !self.should_skip(task)
            })
            .collect::<Vec<_>>();

        // println!("📝 enqueue_tasks: {}", enqueue_tasks.len());

        // - FINALIZE -
        let page_entry = PageEntry::builder()
            .with_http_status(status_code)
            .with_original_url(task.url.clone().into())
            .with_canonical_url(canonical_url_string)
            .with_local_shanpshot_path_rel(local_shanpshot_path_rel.clone())
            .with_local_shanpshot_date(
                crate::crawler::db::ShanpshotDate::new_now()
            )
            .with_outgoing_links(outgoing_links)
            .with_incoming_links(Vec::default())
            .build()
            .unwrap();

        // - RAW -
        {
            let snapshot_path = local_shanpshot_path_rel.resolve_full_snapshot_path(self.project_context);
            if snapshot_path.exists() {
                eprintln!("❌ Skipping task {:?} : file exists {snapshot_path:?}", task.url.to_string());
                self.database.add_failed_url(
                    OriginalUrlString(task.url.to_string()),
                    // FailedUrlError::InternalFileExists(OriginalUrlString(task.url.to_string()))
                    FailedUrlError::InternalFileExists {
                        conflicting_file: local_shanpshot_path_rel.clone(),
                    }
                );
                tab.close().await;
                return
            }
            std::fs::create_dir_all(snapshot_path.parent().unwrap()).unwrap();
            std::fs::write(snapshot_path, &html).unwrap();
        }
        
        // - INGESTED -
        {
            let name = "normalized";
            let ingested_html = {
                let html = crate::processor::passes::parse(&html);
                let html = crate::processor::passes::to_normalized(html);
                let html = html.format_document_pretty();
                html
            };
            let ingested_path = local_shanpshot_path_rel.resolve_full_sub_snapshot_path_for(self.project_context, name);
            std::fs::create_dir_all(ingested_path.parent().unwrap()).unwrap();
            std::fs::write(ingested_path, ingested_html).unwrap();
        }
        {
            let name = "text-tree";
            let ingested_html = {
                let html = crate::processor::passes::parse(&html);
                let html = crate::processor::passes::to_text_tree(html);
                let html = html.format_document_pretty();
                html
            };
            let ingested_path = local_shanpshot_path_rel.resolve_full_sub_snapshot_path_for(self.project_context, name);
            std::fs::create_dir_all(ingested_path.parent().unwrap()).unwrap();
            std::fs::write(ingested_path, ingested_html).unwrap();
        }
        {
            let name = "plaintext";
            let ingested_html = {
                let html = crate::processor::passes::parse(&html);
                let html = crate::processor::passes::to_plain_text(html);
                // let html = html.format_document_pretty();
                html
            };
            let ingested_path = local_shanpshot_path_rel.resolve_full_sub_snapshot_path_for(self.project_context, name).with_extension("txt");
            std::fs::create_dir_all(ingested_path.parent().unwrap()).unwrap();
            std::fs::write(ingested_path, ingested_html).unwrap();
        }
        {
            let name = "metadata";
            let html = crate::processor::passes::parse(&html);
            if let Some(metadata) = crate::processor::metadata::compile_report(&html) {
                let metadata = serde_json::to_string_pretty(&metadata).unwrap();
                let ingested_path = page_entry.local_shanpshot_path_rel.resolve_full_sub_snapshot_path_for(self.project_context, name).with_extension("json");
                std::fs::create_dir_all(ingested_path.parent().unwrap()).unwrap();
                std::fs::write(ingested_path, metadata).unwrap();
            }
        }

        // - FINALIZE -
        let _ = self.database.insert(page_entry);

        // - DONE -
        tab.close().await;
        
        // queue.extend(enqueue_tasks);
        for next in enqueue_tasks {
            queue.push_back(next);
        }
    }
    
    pub fn should_skip(&self, crawl_task: &CrawlTask) -> bool {
        crawl_task.should_skip(self)
    }
}

#[derive(Debug, Clone)]
struct CrawlTask {
    pub url: Url,
}

impl CrawlTask {
    pub fn new(url: impl AsRef<str>) -> Self {
        let url = Url::from_str(url.as_ref()).unwrap();
        Self::from_url(url)
    }

    pub fn from_url(url: Url) -> Self {
        CrawlTask { url }
    }

    /// Normalize a URL (remove fragments, etc.)
    pub fn canonicalize_url(&self) -> Url {
        let mut new = self.url.clone();
        new.set_fragment(None);
        new
    }

    pub fn should_skip(&self, context: &Context) -> bool {
        let canonical_url = self.canonicalize_url();
        let canonical_url_string = CanonicalUrlString(canonical_url.to_string());
        if let Some(entry) = context.database.lookup(&canonical_url_string) {
            let snapshot_path = entry.local_shanpshot_path_rel.resolve_full_snapshot_path(context.project_context);
            if snapshot_path.exists() {
                return true
            }
        }
        false
    }
}
