use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;

use crate::system::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct ContextMacroTag;

impl MacroTag for ContextMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "context" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let _ = attributes;
        let _ = children;
        let _ = scope;
        let _ = runtime;
        unimplemented!("TODO")
    }
}
