use crate::{AttributeMap, Element, Fragment, Node, TagBuf};

pub trait ElementVisitor {
    fn visit_element(
        &mut self,
        tag: TagBuf,
        attributes: AttributeMap,
        children: Fragment,
    ) -> Node {
        Node::Element(Element { tag, attributes, children })
    }
}

pub fn apply_element_visitor<V: ElementVisitor>(node: Node, visitor: &mut V) -> Node {
    node.apply_element_visitor(visitor)
}

impl Node {
    fn apply_element_visitor<V: ElementVisitor>(self, visitor: &mut V) -> Node {
        match self {
            Self::Text(text) => Self::Text(text),
            Self::Element(element) => element.apply_element_visitor(visitor),
            Self::Fragment(fragment) => fragment.apply_element_visitor(visitor),
        }
    }
}

impl Element {
    fn apply_element_visitor<V: ElementVisitor>(self, visitor: &mut V) -> Node {
        let Element { tag, attributes, children } = self;
        let children = children
            .into_iter()
            .map(|element| {
                element.apply_element_visitor(visitor)
            })
            .collect::<Vec<_>>();
        let children = Fragment::from_nodes(children);
        visitor.visit_element(tag, attributes, children)
    }
}

impl Fragment {
    fn apply_element_visitor<V: ElementVisitor>(self, visitor: &mut V) -> Node {
        let nodes = self
            .into_iter()
            .map(|element| {
                element.apply_element_visitor(visitor)
            })
            .flat_map(|node| {
                match node {
                    Node::Fragment(fragment) => fragment.to_vec(),
                    node => vec![node]
                }
            })
            .collect::<Vec<_>>();
        Node::Fragment(Fragment::from_nodes(nodes))
    }
}

