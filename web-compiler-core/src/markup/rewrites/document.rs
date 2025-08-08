// use macro_types::environment::MacroRuntime;
use macro_types::lexical_env::{MacroIO, ProcessScope, SourceHostRef};
use macro_types::tag_rewrite_rule::TagRewriteRule;
use xml_ast::{Element, Node};

use web_compiler_types::CompilerRuntime;

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT HEAD
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, Default)]
pub struct DocumentHead;
impl TagRewriteRule for DocumentHead {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "document-head" }
    fn pre_process(
        &self,
        element: Element,
        _: &mut ProcessScope,
        _: &Self::Runtime,
    ) -> MacroIO<Node> {
        let Element { tag: _, attributes, children } = element;
        let element = Element::new("head")
            .with_attributes(attributes)
            .with_children(children);
        MacroIO::wrap(Node::Element(element))
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT BODY
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, Default)]
pub struct DocumentBody;
impl TagRewriteRule for DocumentBody {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "document-body" }
    fn pre_process(
        &self,
        element: Element,
        _: &mut ProcessScope,
        _: &Self::Runtime,
    ) -> MacroIO<Node> {
        let Element { tag: _, attributes, children } = element;
        let element = Element::new("body")
            .with_attributes(attributes)
            .with_children(children);
        MacroIO::wrap(Node::Element(element))
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}


// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT 
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, Default)]
pub struct Document;
impl TagRewriteRule for Document {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "document" }
    fn pre_process(
        &self,
        element: Element,
        _: &mut ProcessScope,
        _: &Self::Runtime,
    ) -> MacroIO<Node> {
        let Element { tag: _, attributes, children } = element;
        let element = Element::new("html")
            .with_attributes(attributes)
            .with_children(children);
        MacroIO::wrap(Node::Element(element))
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}
