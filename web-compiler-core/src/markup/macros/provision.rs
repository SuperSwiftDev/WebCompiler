use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;

use web_compiler_types::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct ProvisionMacroTag;

impl MacroTag for ProvisionMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "provision" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let _ = attributes;
        let _ = children;
        let _ = scope;
        let _ = runtime;
        unimplemented!("TODO")
    }
}
