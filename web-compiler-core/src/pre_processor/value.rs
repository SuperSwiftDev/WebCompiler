use std::collections::HashMap;

use web_compiler_html_ast::{Html, TagBuf};

use super::BinderValue;
use super::ScopeBindingEnv;
use super::PreProcessIO;
use super::PreProcessor;

pub fn value_element_handler(
    _: &PreProcessor,
    _: TagBuf,
    attrs: HashMap<String, String>,
    _: Vec<Html>,
    scope_binding_env: &mut ScopeBindingEnv,
) -> PreProcessIO<Html> {
    let result = attrs
        .get("for")
        .and_then(|value| {
            scope_binding_env.lookup_binding(&value)
        })
        .cloned()
        .and_then(|value| {
            match value {
                BinderValue::Html(html) => Some(html),
                BinderValue::Literal(value) => Some(Html::Text(value)),
                BinderValue::Object(_) => None,
            }
        })
        .unwrap_or_else(|| Html::Fragment(vec![]));
    PreProcessIO::wrap(result)
}
