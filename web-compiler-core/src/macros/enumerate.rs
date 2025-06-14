use macro_types::macro_tag::MacroTag;
use macro_types::environment::{MacroIO, MacroRuntime};

// use crate::pre_processor::PreProcessor;

#[derive(Debug, Clone, Copy, Default)]
pub struct EnumerateMacroTag;

impl MacroTag for EnumerateMacroTag {
    fn tag_name(&self) -> &'static str { "enumerate" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        runtime: &MacroRuntime,
    ) -> MacroIO<xml_ast::Node> {
        // let processor = PreProcessor::new(source_context, macro_tag_set)
        let _ = attributes;
        let _ = children;
        let _ = scope;
        let _ = runtime;
        unimplemented!("TODO")
    }
}
