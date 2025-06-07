use std::collections::HashMap;

use web_compiler_html_ast::{Html, TagBuf};

use super::BinderValue;
use super::ScopeBindingEnv;
use super::PreProcessIO;
use super::PreProcessor;

pub fn bind_element_handler(
    _: &PreProcessor,
    _: TagBuf,
    attrs: HashMap<String, String>,
    children: Vec<Html>,
    scope_binding_env: &mut ScopeBindingEnv,
) -> PreProcessIO<Html> {
    assert!(children.is_empty());
    fn host_mode(attrs: &HashMap<String, String>, env: &ScopeBindingEnv) -> Option<(String, BinderValue)> {
        attrs
            .get("host")
            .and_then(|_| attrs.get("get-attribute"))
            .and_then(|key| {
                env .lookup_binding("host")
                    .and_then(|host| host.as_object())
                    .and_then(|host| host.get(key))
            })
            .map(|value| BinderValue::Literal(value.to_owned()))
            .and_then(|value| {
                attrs
                    .get("as")
                    .cloned()
                    .map(|key| (key, value))
            })
    }
    fn from_mode(attrs: &HashMap<String, String>, env: &ScopeBindingEnv) -> Option<(String, BinderValue)> {
        attrs
            .get("from")
            .and_then(|key| env.lookup_binding(key))
            .and_then(|value| {
                attrs
                    .get("get-attribute")
                    .and_then(|key| {
                        value.get_attribute(key)
                    })
            })
            .map(|value| BinderValue::Literal(value.to_owned()))
            .and_then(|value| {
                attrs
                    .get("as")
                    .cloned()
                    .map(|key| (key, value))
            })
    }
    let options = &[host_mode, from_mode];
    for option in options {
        if let Some((key, value)) = option(&attrs, &scope_binding_env) {
            scope_binding_env.define_binding(key, value);
            break;
        }
    }
    PreProcessIO::wrap(Html::empty())
}
