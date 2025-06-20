use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;
use macro_types::path_expr::PathExpression;
use xml_ast::Node;

use web_compiler_types::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct BindMacroTag;

impl MacroTag for BindMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "bind" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        _: xml_ast::Fragment,
        scope: &mut macro_types::environment::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let target = attributes
            .get("path")
            .and_then(|path_key| {
                let as_key = attributes.get("as")?;
                Some((path_key, as_key))
            })
            .and_then(|(path_key, as_key)| {
                let path_expr = PathExpression::parse(path_key.as_str());
                let path_expr = match path_expr {
                    Ok(x) => x,
                    Err(error) => {
                        runtime.with_source_file_path(|file| {
                            eprintln!("⚠️ {file:?} <bind> failed to resolve path expression `{:?}`: {error}", path_key.as_str());
                        });
                        return None
                    }
                };
                let binder_value = path_expr.evaluate(&scope.binding_scope);
                let binder_value = match binder_value {
                    Some(x) => x,
                    None => {
                        runtime.with_source_file_path(|file| {
                            eprintln!("⚠️ {file:?} <bind> failed to resolve binding `{:?}`", path_key.as_str());
                        });
                        return None
                    }
                };
                Some((binder_value, as_key))
            });
        if let Some((binder_value, as_key)) = target {
            scope.binding_scope.insert(as_key.as_str().to_string(), binder_value);
        }
        MacroIO::wrap(Node::empty())
    }
}

