use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;
use xml_ast::Node;

use crate::system::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct ContentMacroTag;

impl MacroTag for ContentMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "content" }
    fn apply(
        &self,
        _: xml_ast::AttributeMap,
        _: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        _: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let node = scope.binding_scope
            .lookup("content")
            .and_then(|x| x.as_node())
            .map(|x| x.to_owned())
            .unwrap_or_else(|| Node::empty());
        MacroIO::wrap(node)
    }
}
