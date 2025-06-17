use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;
use macro_types::scope::{BinderValue, JsonBinderValue};
use xml_ast::Node;

use web_compiler_types::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct InjectMacroTag;

impl MacroTag for InjectMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "inject" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        _: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let injection = attributes
            .get("path")
            .and_then(|path_key| {
                let result = scope.binding_scope.lookup(path_key.as_str());
                if result.is_none() {
                    let source_file = runtime.source_context();
                    let source_file = source_file.file_input().source_file();
                    eprintln!("⚠️ {source_file:?} failed to resolve binding `{:?}`", path_key.as_str());
                }
                result
            })
            .and_then(|binder_value| {
                match binder_value {
                    BinderValue::Markup(node) => Some(node.0.to_owned()),
                    BinderValue::Json(JsonBinderValue::String(value)) => Some(Node::Text(value.to_string())),
                    _ => {
                        let source_file = runtime.source_context();
                        let source_file = source_file.file_input().source_file();
                        eprintln!("⚠️ {source_file:?} failed to resolve binding as markup: {:?}", binder_value);
                        None
                    },
                }
            });
        // - -
        if let Some(injection) = injection {
            return MacroIO::wrap(injection.to_owned())
        }
        MacroIO::wrap(Node::empty())
    }
}
