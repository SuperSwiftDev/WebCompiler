use std::collections::BTreeSet;

use once_cell::sync::Lazy;
use xml_ast::{AttributeMap, Element, Fragment, Node, TagBuf};

pub fn parse(html_source: impl AsRef<str>) -> Node {
    let html_source = html_source.as_ref();
    xml_ast::parse_document_str(html_source).unwrap()
}

pub fn to_normalized(node: Node) -> Node {
    let mut visitor = NormalizedTree;
    xml_ast::traversal::full_markup_visitor(node, &mut visitor)
}

pub fn to_text_tree(node: Node) -> Node {
    let mut visitor = TextTreeRewriter;
    xml_ast::traversal::full_markup_visitor(node, &mut visitor)
}

pub fn to_plain_text(node: Node) -> String {
    let node = to_text_tree(node);
    let mut visitor = PlainTextRewriter;
    let plain_text_tree = xml_ast::traversal::full_markup_visitor(node, &mut visitor);
    plain_text_tree.extract_text_strict().unwrap().join("\n")
}

// ————————————————————————————————————————————————————————————————————————————
// NORMALIZED REWRITER
// ————————————————————————————————————————————————————————————————————————————

struct NormalizedTree;

impl xml_ast::traversal::MarkupVisitor for NormalizedTree {
    fn visit_element(
        &mut self,
        tag: TagBuf,
        attributes: AttributeMap,
        children: Fragment,
    ) -> Node {
        if BLACKLISTED_TAGS.contains(tag.as_normalized()) {
            let unless = attributes.contains_key_value("type", "application/ld+json");
            if !unless {
                return Node::empty()
            }
        }
        Node::Element(Element { tag, attributes, children })
    }
}

// ————————————————————————————————————————————————————————————————————————————
// TEXT-TREE REWRITER
// ————————————————————————————————————————————————————————————————————————————

struct TextTreeRewriter;

impl xml_ast::traversal::MarkupVisitor for TextTreeRewriter {
    fn visit_text(
        &mut self,
        text: String,
    ) -> Node {
        if text.trim().is_empty() {
            return Node::empty()
        }
        Node::Text(text)
    }
    fn visit_element(
        &mut self,
        tag: TagBuf,
        _: AttributeMap,
        children: Fragment,
    ) -> Node {
        if BLACKLISTED_TAGS.contains(tag.as_normalized()) {
            return Node::empty()
        }
        let attributes = AttributeMap::default();
        if is_empty_fragment(&children) {
            return Node::empty()
        }
        Node::Element(Element { tag, attributes, children })
    }
    fn visit_fragment(
        &mut self,
        fragment: Fragment,
    ) -> Node {
        if is_empty_fragment(&fragment) {
            return Node::empty()
        }
        let fragment: Vec<Node> = fragment
            .clone()
            .flatten();
        if fragment.len() == 1 {
            let node = fragment.get(0).unwrap().to_owned();
            return node
        }
        Node::Fragment(Fragment::from_nodes(fragment))
    }
}



static BLACKLISTED_TAGS: Lazy<BTreeSet<&'static str>> = Lazy::new(|| {
    BTreeSet::from_iter(vec![
        "script",
        "style",
    ])
});

fn is_empty_fragment(element: &Fragment) -> bool {
    element
        .clone()
        .flatten()
        .into_iter()
        .all(|node| {
            match node {
                Node::Text(x) => {
                    x.trim().is_empty()
                }
                Node::Element(_) => {
                    false
                }
                Node::Fragment(_) => {
                    unimplemented!()
                }
            }
        })
}

// ————————————————————————————————————————————————————————————————————————————
// PLAIN-TEXT REWRITER
// ————————————————————————————————————————————————————————————————————————————

struct PlainTextRewriter;

impl xml_ast::traversal::MarkupVisitor for PlainTextRewriter {
    fn visit_element(
        &mut self,
        _: TagBuf,
        _: AttributeMap,
        children: Fragment,
    ) -> Node {
        Node::Fragment(Fragment::from_nodes(children.flatten()))
    }
}

