use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use xml_ast::TagBuf;
use crate::project::{FileInput, ProjectContext};

#[derive(Debug, Clone, Default)]
pub struct SiteTreeLayout {
    breadcrumb_path_map: BTreeMap::<PathBuf, SystemBreadcrumbPath>,
}

impl SiteTreeLayout {
    pub fn lookup_for(&self, input: &FileInput) -> Option<&SystemBreadcrumbPath> {
        self.breadcrumb_path_map.get(&input.source)
    }
    pub fn compute(input_rules: &[FileInput], project_context: &ProjectContext) -> Self {
        let mut mapping = BTreeMap::<PathBuf, FileInput>::default();
        let mut title_map = BTreeMap::<PathBuf, String>::new();
        let mut breadcrumb_path_map = BTreeMap::<PathBuf, SystemBreadcrumbPath>::default();
        // - -
        for input in input_rules {
            let mut public_path = input.resolved_public_path(project_context);
            let public_file_name = public_path.file_name().unwrap().to_str().unwrap();
            if public_file_name == "index.html" {
                public_path = public_path.parent().unwrap().to_path_buf();
            }
            mapping.insert(public_path, input.clone());
        }
        // - -
        for input in input_rules {
            if !title_map.contains_key(input.source_file()) {
                let title = get_title(input);
                let title = match title {
                    Some(x) => x,
                    None => continue
                };
                title_map.insert(input.source.clone(), title);
            }
        }
        // - -
        for input in input_rules {
            let public_path = input.resolved_public_path(project_context);
            let components = public_path
                .components()
                .map(|x| x.as_os_str().to_str().unwrap().to_string())
                .collect::<Vec<_>>();
            // - -
            let mut breadcrumbs = Vec::<SystemBreadcrumbComponent>::new();
            let mut leading = PathBuf::new();
            for component in components.clone() {
                let input = mapping.get(&leading).unwrap();
                let title = title_map.get(input.source_file()).unwrap().to_string();
                breadcrumbs.push(SystemBreadcrumbComponent {
                    source: input.to_owned(),
                    title,
                });
                leading.push(component);
            }
            // - -
            if let Some(finale) = mapping.get(&leading) {
                let title = title_map.get(finale.source_file()).unwrap().to_string();
                breadcrumbs.push(SystemBreadcrumbComponent {
                    source: finale.to_owned(),
                    title,
                });
            }
            // - -
            let breadcrumb_path = SystemBreadcrumbPath {
                file_input: input.to_owned(),
                components: breadcrumbs,
            };
            breadcrumb_path_map.insert(input.source.clone(), breadcrumb_path);
        }
        // - -
        Self {
            breadcrumb_path_map: breadcrumb_path_map,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemBreadcrumbPath {
    pub file_input: FileInput,
    pub components: Vec<SystemBreadcrumbComponent>,
}

#[derive(Debug, Clone)]
pub struct SystemBreadcrumbComponent {
    // pub title: String,
    pub source: FileInput,
    pub title: String,
}

fn get_title(file_input: &FileInput) -> Option<String> {
    let source = file_input.load_source_file().unwrap();
    let source_tree = {
        let output = xml_ast::parse_str_auto(&source);
        if !output.errors.is_empty() {
            let errors = output.errors.join("\n");
            eprintln!("TODO: {errors}");
            return None
        }
        output.output
    };
    let target_tag = TagBuf::from("define-title");
    let title = source_tree.find_first(&target_tag).expect("expecting define-title tag");
    let title = title.as_element().unwrap();
    let body_text = title.text_contents().join("");
    let body_text = body_text.trim().to_string();
    let override_value = title.attributes
        .get("page")
        .or_else(|| title.attributes.get("bind:title"))
        .map(|x| x.as_str().to_string());
    Some(override_value.unwrap_or(body_text))
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct BreadcrumbPathListValue(pub Vec<BreadcrumbComponentValue>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbComponentValue {
    /// Encoded virtual path.
    pub href: String,
    pub title: String,
}
