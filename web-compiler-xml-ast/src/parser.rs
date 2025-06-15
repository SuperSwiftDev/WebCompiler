use web_compiler_markup_parser::HtmlTreeBuilder;

use crate::{AttributeKeyBuf, AttributeMap, AttributeValueBuf, Element, Fragment, Node, TagBuf};

struct Parser;

impl HtmlTreeBuilder for Parser {
    type Output = Node;
    fn text_node(&mut self, text: String) -> Self::Output {
        Node::Text(text)
    }
    fn element_node(&mut self, name: String, attributes: Vec<(String, String)>, children: Vec<Self::Output>) -> Self::Output {
        let attributes = attributes
            .into_iter()
            .map(|(k, v)| {
                (AttributeKeyBuf::new(k), AttributeValueBuf::literal(v))
            })
            .collect::<Vec<_>>();
        let attributes = AttributeMap::from_iter(attributes);
        let children = Fragment::from_nodes(children);
        Node::Element(Element {
            tag: TagBuf::from(name),
            attributes,
            children,
        })
    }
    fn fragment_node(&mut self, fragment: Vec<Self::Output>) -> Self::Output {
        Node::Fragment(Fragment::from_nodes(fragment))
    }
    fn comment_node(&mut self, _: String) -> Self::Output {
        Node::empty()
    }
}


pub fn parse_fragment_str(source: impl AsRef<str>) -> ParserPayload<Node> {
    let mut parser = Parser;
    let payload = web_compiler_markup_parser::parse_fragment_str(source, &mut parser);
    let output = Fragment::from_nodes(payload.output);
    let errors = payload.errors;
    ParserPayload {
        output: Node::Fragment(output),
        errors,
    }
}

pub fn parse_document_str(source: impl AsRef<str>) -> ParserPayload<Node> {
    let mut parser = Parser;
    let payload = web_compiler_markup_parser::parse_document_str(source, &mut parser);
    let output = Fragment::from_nodes(payload.output);
    let errors = payload.errors;
    ParserPayload {
        output: Node::Fragment(output),
        errors,
    }
}

pub fn parse_str_auto(source: impl AsRef<str>) -> ParserPayload<Node> {
    let source = source.as_ref();
    let normalized = source.to_ascii_lowercase();
    let has_doctype = normalized.contains("<!doctype html>");
    // let has_doctype = normalized.contains("<!doctype html>");
    if has_doctype {
        parse_document_str(source)
    } else {
        parse_fragment_str(source)
    }
}

pub struct ParserPayload<Output> {
    pub output: Output,
    pub errors: Vec<String>,
}
