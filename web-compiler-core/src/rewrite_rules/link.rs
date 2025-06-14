#![allow(unused)]
use macro_types::environment::{LexicalEnvironment, MacroRuntime};
use macro_types::environment::{AccumulatedEffects, MacroIO};
use macro_types::tag_rewrite_rule::TagRewriteRule;
use xml_ast::{AttributeValueBuf, Element, Node};

#[derive(Debug, Clone, Copy, Default)]
pub struct LinkMacroTag;

// impl TagRewriteRule for LinkMacroTag {
//     fn tag_name(&self) -> &'static str {
//         "link"
//     }
//     fn pre_process(
//         &self,
//         element: Element,
//         scope: &mut LexicalEnvironment,
//         runtime: &MacroRuntime,
//     ) -> MacroIO<Node> {
//         let Element { tag, mut attributes, children } = element;
//         let mut effects = AccumulatedEffects::default();
//         let href_value = attributes.get("href");
//         let rel_value = attributes.get("rel");
//         fn is_valid_ref(value: &AttributeValueBuf) -> bool {
//             value.as_str().to_ascii_lowercase() == "stylesheet"
//         }
//         match (href_value, rel_value) {
//             (Some(href_value), Some(rel_value)) if is_valid_ref(rel_value) => {
//                 let dependency = runtime.source_context().with_dependency_relation(href_value.as_str());
//                 let encoded_url = dependency.encode();
//                 let href_value = AttributeValueBuf::literal(encoded_url);
//                 attributes.insert("href", href_value);
//             }
//             _ => ()
//         }
//         MacroIO::wrap(Node::Element(Element { tag, attributes, children }))
//     }
//     fn post_process(&self, element: Element) -> MacroIO<Node> {
//         unimplemented!()
//     }
// }
