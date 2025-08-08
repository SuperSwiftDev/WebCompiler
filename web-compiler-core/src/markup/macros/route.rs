use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;

use web_compiler_types::CompilerRuntime;
use xml_ast::{AttributeKeyBuf, AttributeValueBuf, Element, Node};

use crate::markup::PreProcessor;

#[derive(Debug, Clone, Copy, Default)]
pub struct RouteMacroTag;

impl MacroTag for RouteMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "route" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        if attributes.contains_key("href:self") {
            let mut child_scope = scope.to_owned();
            let source_context = runtime.source_context();
            let file_input = source_context.file_input();
            let file_path = file_input.source_file().file_name().unwrap();
            let file_path = file_path.to_str().unwrap();
            let dependency = file_input.with_dependency_relation(file_path);

            return PreProcessor::new(runtime.clone())
                .process_sequence(children.to_vec(), &mut child_scope)
                .map(|children| {
                    let href_key = AttributeKeyBuf::from("href");
                    let href_value = AttributeValueBuf::literal(dependency.encode());
                    return Element::new("a")
                        .with_attribute(href_key, href_value)
                        .with_children(children)
                        .into()
                })
                .and_modify_context(|ctx| {
                    ctx.dependencies.insert(dependency);
                });
        }
        return MacroIO::wrap(Node::empty())
    }
}
