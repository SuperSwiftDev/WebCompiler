use std::collections::BTreeSet;
use std::{collections::BTreeMap, str::FromStr};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use url::Url;

// ————————————————————————————————————————————————————————————————————————————
// BASICS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct CanonicalUrlString(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct OriginalUrlString(pub String);

impl OriginalUrlString {
    /// Normalize a URL (remove fragments, etc.)
    pub fn canonical_url_string(&self) -> Result<CanonicalUrlString, url::ParseError> {
        let mut url = Url::from_str(&self.0)?;
        url.set_fragment(None);
        let url = url.to_string();
        Ok(CanonicalUrlString(url))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ShanpshotDate(pub String);

impl ShanpshotDate {
    pub fn new_now() -> Self {
        let date = chrono::Utc::now();
        Self(date.to_rfc3339())
    }
}

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub output_dir: PathBuf,
}

impl AsRef<str> for CanonicalUrlString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for OriginalUrlString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<OriginalUrlString> for Url {
    fn into(self) -> OriginalUrlString {
        OriginalUrlString(self.to_string())
    }
}
impl Into<CanonicalUrlString> for Url {
    fn into(mut self) -> CanonicalUrlString {
        self.set_fragment(None);
        CanonicalUrlString(self.to_string())
    }
}

use serde::{Deserializer, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeFileUrl(pub PathBuf);

impl RelativeFileUrl {
    /// Resolve the path relative to a given base directory.
    pub fn resolve_from(&self, base: &Path) -> PathBuf {
        base.join(&self.0)
    }

    /// Get a `file://./` prefixed string for JSON output.
    pub fn to_file_url(&self) -> String {
        format!("file://./{}", self.0.to_string_lossy())
    }

    pub fn resolve_full_snapshot_path(&self, project_context: &ProjectContext) -> PathBuf {
        project_context.output_dir.join(&self.0)
    }
    pub fn resolve_full_sub_snapshot_path_for(&self, project_context: &ProjectContext, name: &str) -> PathBuf {
        let snapshot = self.resolve_full_snapshot_path(project_context);
        let snapshot_parent = snapshot.parent().unwrap();
        let snapshot_ext = snapshot.extension().unwrap().to_str().unwrap();
        let snapshot_file_stem = snapshot.file_stem().unwrap().to_str().unwrap();
        let cleaned_file_name = format!("{snapshot_file_stem}.{name}.{snapshot_ext}");
        snapshot_parent.join(cleaned_file_name)
    }
    pub fn file_exists(&self, project_context: &ProjectContext) -> bool {
        self.resolve_full_snapshot_path(project_context).exists()
    }
}

impl Serialize for RelativeFileUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_file_url())
    }
}

impl<'de> Deserialize<'de> for RelativeFileUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let stripped = s
            .strip_prefix("file://./")
            .ok_or_else(|| serde::de::Error::custom("Expected 'file://./' prefix"))?;
        Ok(RelativeFileUrl(PathBuf::from(stripped)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailedUrlError {
    HttpError {
        status: Option<i64>,
    },
    InternalFileExists {
        conflicting_file: RelativeFileUrl,
    },
}

// ————————————————————————————————————————————————————————————————————————————
// DATABASE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Database {
    pub entries: BTreeMap<CanonicalUrlString, PageEntry>,
    pub failed_urls: BTreeMap<OriginalUrlString, FailedUrlError>,
}

impl Database {
    pub fn file_path(output_dir: impl AsRef<Path>) -> PathBuf {
        output_dir.as_ref().join("database.json")
    }
    pub fn load(output_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let database_path = Self::file_path(output_dir);
        let database = std::fs::read_to_string(&database_path)?;
        let database = serde_json::from_str::<Self>(&database)?;
        Ok(database)
    }
    pub fn save(&self, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let database_path = Self::file_path(output_dir);
        let database_json = serde_json::to_string_pretty(self)?;
        std::fs::write(&database_path, database_json)?;
        Ok(())
    }
    pub fn insert(&mut self, page_entry: PageEntry) -> Option<PageEntry> {
        let key = page_entry.canonical_url.clone();
        self.entries.insert(key, page_entry)
    }
    
    pub fn lookup(&self, canonical_url: &CanonicalUrlString) -> Option<&PageEntry> {
        self.entries.get(canonical_url)
    }
    pub fn add_failed_url(&mut self, key: OriginalUrlString, value: FailedUrlError) {
        let _ = self.failed_urls.insert(key, value);
    }
    pub fn has_failed_url(&mut self, key: &OriginalUrlString) -> bool {
        self.failed_urls.contains_key(key)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// PAGE ENTRY
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEntry {
    pub http_status: Option<i64>,
    pub original_url: OriginalUrlString,
    pub canonical_url: CanonicalUrlString,
    /// Relative to the output directory.
    pub local_shanpshot_path_rel: RelativeFileUrl,
    pub local_shanpshot_date: ShanpshotDate,
    pub outgoing_links: BTreeSet<OriginalUrlString>,
    pub incoming_links: BTreeSet<OriginalUrlString>,
}

impl PageEntry {
    pub fn builder() -> PageEntryBuilder {
        PageEntryBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PageEntryBuilder {
    pub http_status: Option<i64>,
    pub original_url: Option<OriginalUrlString>,
    pub canonical_url: Option<CanonicalUrlString>,
    /// Relative to the output directory.
    pub local_shanpshot_path_rel: Option<RelativeFileUrl>,
    pub local_shanpshot_date: Option<ShanpshotDate>,
    pub outgoing_links: Option<BTreeSet<OriginalUrlString>>,
    pub incoming_links: Option<BTreeSet<OriginalUrlString>>,
}

impl PageEntryBuilder {
    pub fn with_http_status(mut self, http_status: Option<i64>) -> Self {
        self.http_status = http_status;
        self
    }
    pub fn with_original_url(mut self, original_url: OriginalUrlString) -> Self {
        self.original_url = Some(original_url);
        self
    }
    pub fn with_canonical_url(mut self, canonical_url: CanonicalUrlString) -> Self {
        self.canonical_url = Some(canonical_url);
        self
    }
    pub fn with_local_shanpshot_path_rel(mut self, local_shanpshot_path_rel: RelativeFileUrl) -> Self {
        self.local_shanpshot_path_rel = Some(local_shanpshot_path_rel);
        self
    }
    pub fn with_local_shanpshot_date(mut self, local_shanpshot_date: ShanpshotDate) -> Self {
        self.local_shanpshot_date = Some(local_shanpshot_date);
        self
    }
    pub fn with_outgoing_links(mut self, outgoing_links: impl IntoIterator<Item=OriginalUrlString>) -> Self {
        self.outgoing_links = Some(outgoing_links.into_iter().collect());
        self
    }
    pub fn with_incoming_links(mut self, incoming_links: impl IntoIterator<Item=OriginalUrlString>) -> Self {
        self.incoming_links = Some(incoming_links.into_iter().collect());
        self
    }
    pub fn build(self) -> Option<PageEntry> {
        Some(PageEntry {
            http_status: self.http_status,
            original_url: self.original_url?,
            canonical_url: self.canonical_url?,
            local_shanpshot_path_rel: self.local_shanpshot_path_rel?,
            local_shanpshot_date: self.local_shanpshot_date?,
            outgoing_links: self.outgoing_links?,
            incoming_links: self.incoming_links?,
        })
    }
}

// ————————————————————————————————————————————————————————————————————————————
// HELPERS
// ————————————————————————————————————————————————————————————————————————————

const MAX_SEGMENT_LEN: usize = 100;
const MAX_PATH_LEN: usize = 240;

// const MAX_SEGMENT_LEN: usize = 64;
// const MAX_PATH_LEN: usize = 255;

/// Build relative file path for a given URL.
/// Regarding the parent directory — falls back to hashed folder if path becomes too long or unsafe.
pub fn build_rel_html_snapshot_file_path(url: &str) -> Option<RelativeFileUrl> {
    Some(RelativeFileUrl(build_rel_html_snapshot_dir(url).map(|base| {
        base.join("snapshot.html")
    })?))
}
/// Build directory path for a given URL, including query parameters.
/// Falls back to hashed folder if path becomes too long or unsafe.
pub fn build_rel_html_snapshot_dir(url: &str) -> Option<PathBuf> {
    use sha2::{Digest, Sha256};

    fn sanitize(s: &str) -> String {
        s.chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
            .collect()
    }

    fn short_hash(s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let hash = hasher.finalize();
        let short = hash[..4]
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        format!("h{short}")
    }

    let parsed = Url::parse(url).ok()?;
    let host = parsed.host_str().unwrap_or("unknown");

    let mut path = parsed.path().trim_matches('/').to_string();
    if path.is_empty() {
        path = "index".to_string();
    }

    let query = parsed.query().unwrap_or("");
    let query_suffix = if !query.is_empty() {
        let sanitized_query = sanitize(query);
        format!("~q~{sanitized_query}")
    } else {
        String::new()
    };

    let mut full_path = PathBuf::from(host);
    let mut total_len = host.len();

    let mut segments: Vec<String> = path
        .split('/')
        .map(sanitize)
        .collect();

    // Append sanitized query to the last segment
    if !query_suffix.is_empty() {
        if let Some(last) = segments.last_mut() {
            last.push_str(&query_suffix);
        }
    }

    for seg in &segments {
        total_len += seg.len() + 1;
        if seg.len() > MAX_SEGMENT_LEN || total_len > MAX_PATH_LEN {
            let hashed = short_hash(url);
            return Some(PathBuf::from(host).join("long").join(hashed));
        }
        full_path = full_path.join(seg);
    }

    Some(full_path)
}


// /// Build directory path for a given URL.
// /// Falls back to hashed folder if path becomes too long or unsafe.
// pub fn build_rel_html_snapshot_dir(url: &str) -> Option<PathBuf> {
//     fn sanitize(s: &str) -> String {
//         s.chars()
//             .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
//             .collect()
//     }
//     fn short_hash(s: &str) -> String {
//         use sha2::{Digest, Sha256};
//         let mut hasher = Sha256::new();
//         hasher.update(s.as_bytes());
//         let hash = hasher.finalize();

//         let short = hash[..4]
//             .iter()
//             .map(|b| format!("{:02x}", b))
//             .collect::<String>();

//         format!("h{short}")
//     }
//     let parsed = Url::parse(url).ok()?;
//     let host = parsed.host_str().unwrap_or("unknown");

//     let mut path = parsed.path().trim_matches('/').to_string();
//     if path.is_empty() {
//         path = "index".to_string();
//     }

//     let mut full_path = PathBuf::from(host);
//     let mut total_len = host.len();

//     let segments: Vec<String> = path
//         .split('/')
//         .map(sanitize)
//         .collect();

//     for seg in &segments {
//         total_len += seg.len() + 1;

//         if seg.len() > MAX_SEGMENT_LEN || total_len > MAX_PATH_LEN {
//             let hashed = short_hash(url);
//             return Some(PathBuf::from(host).join("long").join(hashed));
//         }

//         full_path = full_path.join(seg);
//     }

//     Some(full_path)
// }
