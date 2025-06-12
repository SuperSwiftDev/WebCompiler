use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::{AttributeMap, AttributeValueBuf, Element, Fragment, Node, TagBuf};

use std::str::from_utf8;

/// Parses an XML string into a `Fragment` AST.
pub fn parse_xml_to_ast(input: &str) -> Result<Fragment, String> {
    let mut reader = Reader::from_str(input);

    let mut buf = Vec::new();
    let mut stack: Vec<Element> = Vec::new();
    let mut root_nodes: Vec<Node> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = from_utf8(e.name().as_ref()).map_err(|e| format!("Invalid UTF-8 in tag name: {e}"))?.to_string();
                let attributes = parse_attributes(e)?;

                let element = Element {
                    tag: TagBuf::new(tag),
                    attributes,
                    children: Fragment::default(),
                };
                stack.push(element);
            }

            Ok(Event::Empty(ref e)) => {
                let tag = from_utf8(e.name().as_ref()).map_err(|e| format!("Invalid UTF-8 in tag name: {e}"))?.to_string();
                let attributes = parse_attributes(e)?;

                let element = Element {
                    tag: TagBuf::new(tag),
                    attributes,
                    children: Fragment::default(),
                };

                let node = Node::Element(element);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root_nodes.push(node);
                }
            }

            Ok(Event::End(ref e)) => {
                let tag = from_utf8(e.name().as_ref()).map_err(|e| format!("Invalid UTF-8 in tag name: {e}"))?.to_string();
                let Some(element) = stack.pop() else {
                    return Err(format!("Unexpected closing tag </{}> with no matching opening tag", tag));
                };

                if element.tag.as_original() != tag {
                    return Err(format!(
                        "Mismatched closing tag: expected </{}> but got </{}>",
                        element.tag, tag
                    ));
                }

                let node = Node::Element(element);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root_nodes.push(node);
                }
            }

            Ok(Event::Text(e)) => {
                let text = e.unescape().map_err(|e| format!("Invalid text entity: {e}"))?.to_string();
                if !text.trim().is_empty() {
                    let node = Node::Text(text);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root_nodes.push(node);
                    }
                }
            }

            Ok(Event::CData(e)) => {
                let text = from_utf8(&e).map_err(|e| format!("Invalid CDATA UTF-8: {e}"))?.to_string();
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(Node::Text(text));
                } else {
                    root_nodes.push(Node::Text(text));
                }
            }

            Ok(Event::Comment(_)) |
            Ok(Event::Decl(_)) |
            Ok(Event::PI(_)) => {
                // Ignored for now
            }

            Ok(Event::Eof) => break,

            Err(e) => return Err(format!("XML parse error: {}", e)),

            _ => {}
        }

        buf.clear();
    }

    Ok(Fragment::from_nodes(root_nodes))
}

fn parse_attributes(start: &BytesStart<'_>) -> Result<AttributeMap, String> {
    let mut map = AttributeMap::default();

    for attr in start.attributes() {
        let attr = attr.map_err(|e| format!("Attribute parse error: {}", e))?;
        let key = from_utf8(attr.key.as_ref())
            .map_err(|e| format!("Invalid UTF-8 in attribute key: {e}"))?
            .to_string();

        let value = attr.unescape_value()
            .map_err(|e| format!("Invalid entity in attribute value: {e}"))?
            .to_string();

        map.insert(key, AttributeValueBuf::literal(value));
    }

    Ok(map)
}
