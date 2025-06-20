use macro_types::macro_tag::MacroTag;
use macro_types::environment::MacroIO;
use macro_types::scope::BinderValue;
use xml_ast::{Fragment, Node};

use web_compiler_types::CompilerRuntime;

use super::super::pre::PreProcessor;

// use crate::pre_processor::PreProcessor;

#[derive(Debug, Clone, Copy, Default)]
pub struct EnumerateMacroTag;

impl MacroTag for EnumerateMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "enumerate" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let children = children.to_vec();
        attributes
            .get("path")
            .and_then(|target| {
                scope.binding_scope.lookup(target.as_str())
            })
            .and_then(|value| {
                value.as_fragment()
            })
            .cloned()
            .map(|x| x.to_vec())
            .map(|nodes| {
                nodes
                    .into_iter()
                    .filter_map(|x| x.to_element())
                    .collect::<Vec<_>>()
            })
            .and_then(|elements| {
                attributes
                    .get("as")
                    .map(|key| {
                        (key, elements)
                    })
            })
            .map(|(key, items)| {
                items
                    .into_iter()
                    .map(|item| {
                        let mut sub_scope = scope.clone();
                        // let runtime = runtime.fork(&new_input_file);
                        let pre_processor = PreProcessor::new(runtime.clone());
                        // println!("{item:?}");
                        sub_scope.binding_scope.insert(key.as_str(), BinderValue::node(item));
                        let children = children.clone();
                        xml_ast::transform::apply_effectful_markup_transformer_node_vec(
                            children,
                            &pre_processor,
                            &mut sub_scope,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .map(|xs| {
                MacroIO::flatten_vec_deep(xs)
                    .map(Fragment::from_nodes)
                    .map(Node::Fragment)
            })
            .unwrap_or_else(|| {
                MacroIO::wrap(Node::empty())
            })
    }
}
