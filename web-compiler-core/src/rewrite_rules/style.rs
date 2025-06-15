use macro_types::environment::MacroRuntime;
use macro_types::environment::{LexicalEnvironment, MacroIO};
use macro_types::tag_rewrite_rule::TagRewriteRule;
use xml_ast::{Element, Fragment, Node};

use crate::css_processor::{CssPostprocessor, CssPreprocessor};

#[derive(Debug, Clone, Copy, Default)]
pub struct StyleMacroTag;

impl TagRewriteRule for StyleMacroTag {
    fn tag_name(&self) -> &'static str { "style" }
    fn pre_process(
        &self,
        element: Element,
        _: &mut LexicalEnvironment,
        runtime: &MacroRuntime,
    ) -> MacroIO<Node> {
        let text_contents = element
            .text_contents()
            .join("");
        let css_preprocessor = CssPreprocessor::new(runtime.source_context());
        css_preprocessor
            .execute(&text_contents)
            .map(|stylesheet| {
                let Element { tag, attributes, children: _ } = element;
                let children = Fragment::from_nodes(vec![
                    Node::text(stylesheet),
                ]);
                Node::Element(Element {
                    tag,
                    attributes,
                    children
                })
            })
    }
    fn post_process(&self, element: Element) -> Node {
        let Element { tag, attributes, children } = element;
        let text_contents = children
            .text_contents()
            .join("");
        let environment = ();
        let css_post_processor = CssPostprocessor::new(&environment);
        let result = css_post_processor.execute(&text_contents);
        let children = Fragment::from_nodes(vec![Node::text(result)]);
        Node::element(tag, attributes, children)
    }
}
