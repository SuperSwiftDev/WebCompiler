use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;

use macro_types::scope::{BinderValue, MarkupBinderValue};
use web_compiler_types::CompilerRuntime;
use xml_ast::Node;

#[derive(Debug, Clone, Copy, Default)]
pub struct HoistMacroTag;

impl MacroTag for HoistMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "hoist" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let _ = attributes;
        let _ = children;
        let _ = scope;
        let _ = runtime;
        MacroIO::wrap(Node::empty()).and_modify_context(|ctx| {
            ctx.hoisted.push(BinderValue::Markup(MarkupBinderValue(Node::Fragment(children))));
        })
    }
}

