use macro_types::macro_tag::MacroTag;
use macro_types::environment::{MacroIO, MacroRuntime};

#[derive(Debug, Clone, Copy, Default)]
pub struct ProvisionMacroTag;

impl MacroTag for ProvisionMacroTag {
    fn tag_name(&self) -> &'static str { "provision" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        runtime: &MacroRuntime,
    ) -> MacroIO<xml_ast::Node> {
        let _ = attributes;
        let _ = children;
        let _ = scope;
        let _ = runtime;
        unimplemented!("TODO")
    }
}
