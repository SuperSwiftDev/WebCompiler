// //! Client side DSL

// use std::collections::{BTreeMap, BTreeSet};

// use macro_types::lexical_env::{MacroIO, ProcessScope};
// use web_compiler_types::CompilerRuntime;
// use xml_ast::{Element, Fragment, Node, TagBuf};

// use crate::markup::PreProcessor;

// // ————————————————————————————————————————————————————————————————————————————
// // TYPES : ELEMENT RULES
// // ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub struct ElementRewriteRuleExpr {
//     element: TagBuf,
//     function: String,
//     body: Fragment,
// }

// impl ElementRewriteRuleExpr {
//     fn key(&self) -> &str {
//         self.element.as_normalized()
//     }
//     pub fn matches(&self, _: &ProcessScope, instance: &Element) -> bool {
//         self.element.matches(&instance.tag)
//     }
//     pub fn evaluate(
//         &self,
//         instance: &Element,
//         process_scope: &mut ProcessScope,
//         compiler_runtime: &CompilerRuntime,
//     ) -> Result<MacroIO<Node>, ()> {
//         if !self.matches(process_scope, instance) {
//             return Err(())
//         }
//         let function = self.function.trim().to_ascii_lowercase();
//         match function.as_str() {
//             "map" => Ok(Self::map_function(
//                 &self.element,
//                 &self.body,
//                 instance,
//                 process_scope,
//                 compiler_runtime,
//             )?),
//             _ => Err(())
//         }
//     }
//     fn map_function(
//         _: &TagBuf,
//         definition_body: &Fragment,
//         instance: &Element,
//         process_scope: &mut ProcessScope,
//         runtime: &CompilerRuntime,
//     ) -> Result<MacroIO<Node>, ()> {
//         let mut child_scope = process_scope.clone().and_insert_binder_value("self", instance);
//         let pre_processor = PreProcessor::new(runtime.clone());
//         let definition_body = definition_body.clone().to_vec();
//         let result = pre_processor
//             .process_sequence(definition_body, &mut child_scope)
//             .map(|nodes| {
//                 Node::Fragment(Fragment::from_nodes(nodes))
//             });
//         Ok(result)
//     }
// }

// // ————————————————————————————————————————————————————————————————————————————
// // TYPES : RULES
// // ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub enum RewriteRuleExpr {
//     ElementRule(ElementRewriteRuleExpr),
// }

// impl RewriteRuleExpr {
//     fn key(&self) -> &str {
//         match self {
//             Self::ElementRule(x) => x.key(),
//         }
//     }
//     pub fn element(
//         element: impl Into<String>,
//         function: impl Into<String>,
//         body: impl Into<Fragment>,
//     ) -> Self {
//         Self::ElementRule(ElementRewriteRuleExpr {
//             element: TagBuf::from(element.into()),
//             function: function.into(),
//             body: body.into(),
//         })
//     }
//     pub fn matches(&self, process_scope: &ProcessScope, instance: &Element) -> bool {
//         match self {
//             Self::ElementRule(x) => x.matches(process_scope, instance),
//         }
//     }
//     pub fn evaluate(
//         &self,
//         instance: &Element,
//         process_scope: &mut ProcessScope,
//         compiler_runtime: &CompilerRuntime,
//     ) -> Result<MacroIO<Node>, ()> {
//         match self {
//             Self::ElementRule(rule) => rule.evaluate(instance, process_scope, compiler_runtime),
//         }
//     }
// }

// // ————————————————————————————————————————————————————————————————————————————
// // TYPES : RULE SETS
// // ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone, Default)]
// pub struct RewriteRuleExprSet {
//     pub macros: BTreeMap<String, Vec<RewriteRuleExpr>>,
//     supported_tags: BTreeSet<String>,
// }

// impl RewriteRuleExprSet {
//     fn sync_mut(&mut self) {
//         self.supported_tags = self.macros
//             .keys()
//             .map(|x| x.clone())
//             .collect::<BTreeSet<_>>();
//     }
//     // fn synced(mut self) -> Self {
//     //     self.sync_mut();
//     //     self
//     // }
//     pub fn push(&mut self, expr: RewriteRuleExpr) {
//         self.macros
//             .entry(expr.key().to_string())
//             .and_modify(|value| {
//                 value.push(expr.clone());
//             })
//             .or_insert(vec![expr]);
//         self.sync_mut();
//     }
//     pub fn try_apply(
//         &self,
//         element: Element,
//         scope: &mut ProcessScope,
//         runtime: &CompilerRuntime,
//     ) -> MacroIO<Node> {
//         let tag_id = element.tag.as_normalized();
//         if !self.supported_tags.contains(tag_id) {
//             return MacroIO::wrap(Node::Element(element))
//         }
//         let macro_tag = match self.macros.get(tag_id) {
//             Some(x) => x,
//             None => return MacroIO::wrap(Node::Element(element)),
//         };
//         // let element_tag_str = element.tag.as_normalized();
//         // if self.supported_tags.contains(element_tag_str) {
//         //     if let Some(macro_tag) = self.macros.get(element_tag_str) {
//         //         return macro_tag.post_process(element, source_host_ref)
//         //     }
//         // }
//         // MacroIO::wrap(Node::Element(element))
//         unimplemented!("TODO")
//     }
// }

