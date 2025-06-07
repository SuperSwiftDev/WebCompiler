use std::collections::HashMap;

use web_compiler_html_ast::{Html, TagBuf};

use super::BinderValue;
use super::ScopeBindingEnv;
use super::PreProcessIO;
use super::PreProcessor;

pub fn content_element_handler(
    _: &PreProcessor,
    _: TagBuf,
    _: HashMap<String, String>,
    children: Vec<Html>,
    scope_binding_env: &mut ScopeBindingEnv,
) -> PreProcessIO<Html> {
    assert!(children.is_empty());
    match scope_binding_env.lookup_binding("content") {
        Some(BinderValue::Html(html)) => PreProcessIO::wrap(html.to_owned()),
        _ => PreProcessIO::wrap(Html::empty()),
    }
}
