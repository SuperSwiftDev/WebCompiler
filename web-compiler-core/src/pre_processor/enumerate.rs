use std::collections::HashMap;

use web_compiler_html_ast::transform::{HtmlTransformer, IO};
use web_compiler_html_ast::{Html, TagBuf};

use super::BinderValue;
use super::ScopeBindingEnv;
use super::PreProcessIO;
use super::PreProcessor;

pub fn enumerate_element_handler(
    processor: &PreProcessor,
    _: TagBuf,
    attrs: HashMap<String, String>,
    children: Vec<Html>,
    scope_binding_env: &mut ScopeBindingEnv,
) -> PreProcessIO<Html> {
    attrs
        .get("for")
        .and_then(|target| scope_binding_env.lookup_binding(target))
        .and_then(|value| value.as_html())
        .and_then(|node| {
            node.as_fragment() // TODO: CONSIDER OTHER VARIANTS
        })
        .cloned()
        .map(|nodes| {
            nodes
                .into_iter()
                .filter_map(|node| {
                    match node {
                        Html::Element(element) => Some(Html::Element(element)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        })
        // .map(ToOwned::to_owned)
        .and_then(|value| {
            attrs
                .get("as")
                .map(|key| {
                    (key, value)
                })
        })
        .map(|(key, items)| {
            items
                .to_owned()
                .into_iter()
                .map(BinderValue::Html)
                .map(|item| {
                    let mut sub_env = scope_binding_env.clone();
                    // println!("{item:?}");
                    sub_env.define_binding(key, item);
                    let children = children.clone();
                    processor.enter_node_sequence(children, &mut sub_env)
                })
                .collect::<Vec<_>>()
        })
        .map(|xs| {
            IO::flatten_deep(xs).map(Html::Fragment)
        })
        .unwrap_or_else(|| {
            PreProcessIO::wrap(Html::empty())
        })
}
