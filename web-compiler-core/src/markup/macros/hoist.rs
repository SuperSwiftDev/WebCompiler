use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;

use macro_types::scope::{BinderValue, MarkupBinderValue};
use web_compiler_types::CompilerRuntime;
use xml_ast::{Fragment, Node};
use super::super::pre::PreProcessor;

#[derive(Debug, Clone, Copy, Default)]
pub struct HoistMacroTag;

impl MacroTag for HoistMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "hoist" }
    fn apply(
        &self,
        _: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let mut child_scope = scope.to_owned();
        // let mut embedded_scope = scope.to_owned();
        let pre_processor = PreProcessor::new(runtime.clone());
        pre_processor
            .process_sequence(children.to_vec(), &mut child_scope)
            .and_then(|children| {
                MacroIO::wrap(Node::empty()).and_modify_context(|ctx| {
                    let children = Fragment::from_nodes(children);
                    ctx.hoisted.push(BinderValue::Markup(MarkupBinderValue(Node::Fragment(children))));
                })
            })
    }
}

