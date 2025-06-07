use std::collections::HashMap;

use web_compiler_html_ast::transform::HtmlTransformer;
use web_compiler_html_ast::{Html, ParserMode, TagBuf};


use super::BinderValue;
use super::ScopeBindingEnv;
use super::PreProcessIO;
use super::PreProcessor;

pub fn include_element_handler(
    processor: &PreProcessor,
    _: TagBuf,
    attrs: HashMap<String, String>,
    children: Vec<Html>,
    scope_binding_env: &mut ScopeBindingEnv,
) -> PreProcessIO<Html> {
    let mut parent_scope = scope_binding_env.to_owned();
    let mut embedded_scope = scope_binding_env.to_owned();
    let source_context = processor.source_context();
    processor
        .enter_node_sequence(children, &mut parent_scope)
        .and_then(|children| -> PreProcessIO<Html> {
            if let Some(src_value) = attrs.get("src").cloned() {
                let resolved_path = source_context.source_dir().join(&src_value);
                let resolved_path = path_clean::clean(resolved_path);
                // - DEPENDENCY -
                let dependency = source_context.new_relative_source_file_dependency(&src_value);
                // - SCOPE -
                let host_object = attrs
                    .into_iter()
                    .filter(|(key, _)| key != "src");
                embedded_scope.define_binding("content", BinderValue::fragment(children));
                embedded_scope.define_binding("host", BinderValue::object(host_object));
                // - LOAD -
                let parser_mode = ParserMode::fragment("div");
                let expanded_tree = processor
                    .subprocess_html_file(&src_value, &mut embedded_scope, parser_mode)
                    .unwrap_or_else(|error| {
                        let source_path = source_context.source_file();
                        let is_not_found = error
                            .downcast_ref::<std::io::Error>()
                            .filter(|error| error.kind() == std::io::ErrorKind::NotFound)
                            .is_some();
                        if is_not_found {
                            eprintln!("⚠️ {source_path:?} file not found: {resolved_path:?}");
                        } else {
                            eprintln!("⚠️ [{source_path:?} -> {resolved_path:?}]: {error}");
                        }
                        PreProcessIO::wrap(Html::empty())
                    })
                    .and_modify_context(|ctx| {
                        ctx.insert_file_dependency(dependency);
                    });
                return expanded_tree
            }
            PreProcessIO::wrap(Html::empty())
        })
}
