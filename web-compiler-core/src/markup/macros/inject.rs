use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;
use macro_types::scope::{BinderValue, JsonBinderValue};
use xml_ast::{Fragment, Node};

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
        scope: &mut macro_types::lexical_env::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        if attributes.contains_key("hoisted") {
            let nodes = scope
                .host_info()
                .hoisted()
                .into_iter()
                .filter_map(|x| x.as_node())
                .flat_map(|x| x.clone().flatten())
                .filter_map(|x| x.to_element())
                .map(|x| Node::Element(x))
                .collect::<Vec<_>>();
            return MacroIO::wrap(Node::Fragment(Fragment::from_nodes(nodes)))
        }
        if attributes.contains_key("self") {
            let result = scope.binding_scope
                .lookup("self")
                .and_then(|x| x.as_node())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| Node::empty());
            return MacroIO::wrap(result)
        }
        let injection = attributes
            .get("path")
            .and_then(|target| {
                let path_expr = macro_types::path_expr::PathExpression::parse(target.as_str()).unwrap();
                let path_value = path_expr.evaluate(&scope.binding_scope);
                if path_value.is_none() {
                    runtime.with_source_file_path(|file| {
                        eprintln!("⚠️ {file:?} `<inject>` failed to resolve binding `{:?}`", target.as_str());
                    });
                }
                path_value
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
