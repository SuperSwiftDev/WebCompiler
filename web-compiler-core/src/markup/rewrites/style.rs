// use macro_types::environment::MacroRuntime;
use macro_types::environment::{ProcessScope, MacroIO};
use macro_types::tag_rewrite_rule::TagRewriteRule;
use xml_ast::{Element, Fragment, Node};

use css::{CssPostprocessor, CssPreprocessor};

use web_compiler_types::CompilerRuntime;

#[derive(Debug, Clone, Copy, Default)]
pub struct StyleMacroTag;

impl TagRewriteRule for StyleMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "style" }
    fn pre_process(
        &self,
        element: Element,
        _: &mut ProcessScope,
        runtime: &Self::Runtime,
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
                    Node::text(stylesheet.value),
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
        let css_post_processor = CssPostprocessor::new(&());
        let result = css_post_processor.execute(&text_contents);
        let children = Fragment::from_nodes(vec![Node::text(result.value)]);
        Node::element(tag, attributes, children)
    }
}
