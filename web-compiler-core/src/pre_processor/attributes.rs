use std::collections::HashMap;

use web_compiler_html_ast::Html;

use super::{BinderValue, ScopeBindingEnv};

pub fn resolve_attribute_bindings(
    attributes: &mut HashMap<String, String>,
    scope_binding_env: &ScopeBindingEnv,
) {
    for (_, value) in attributes.iter_mut() {
        let resolved_value = value
            .trim()
            .strip_prefix("{{")
            .and_then(|x| x.strip_suffix("}}"))
            .map(|x| x.trim())
            .and_then(|var| {
                scope_binding_env.lookup_binding(var)
            })
            .and_then(|value| {
                match value {
                    BinderValue::Literal(value) => Some(value),
                    BinderValue::Html(Html::Text(value)) => Some(value),
                    BinderValue::Object(_) => None,
                    BinderValue::Html(_) => None,
                }
            })
            .cloned();
        if let Some(resolved_value) = resolved_value {
            *value = resolved_value;
        }
    }
}
