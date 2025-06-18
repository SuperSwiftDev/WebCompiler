use crate::{AttributeMap, Element, Fragment, Node, TagBuf};

// ————————————————————————————————————————————————————————————————————————————
// ELEMENT ONLY VISITOR
// ————————————————————————————————————————————————————————————————————————————

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

// ————————————————————————————————————————————————————————————————————————————
// FULL HTML VISITOR
// ————————————————————————————————————————————————————————————————————————————

/// Full-spectrum markup visitor
pub trait MarkupVisitor {
    fn visit_fragment(
        &mut self,
        fragment: Fragment,
    ) -> Node {
        Node::Fragment(fragment)
    }
    fn visit_text(
        &mut self,
        text: String,
    ) -> Node {
        Node::Text(text)
    }
    fn visit_element(
        &mut self,
        tag: TagBuf,
        attributes: AttributeMap,
        children: Fragment,
    ) -> Node {
        Node::Element(Element { tag, attributes, children })
    }
}

pub fn full_markup_visitor<V: MarkupVisitor>(node: Node, visitor: &mut V) -> Node {
    node.full_markup_visitor(visitor)
}

impl Node {
    fn full_markup_visitor<V: MarkupVisitor>(self, visitor: &mut V) -> Node {
        match self {
            Self::Text(text) => visitor.visit_text(text),
            Self::Element(element) => element.full_markup_visitor(visitor),
            Self::Fragment(fragment) => fragment.full_markup_visitor(visitor),
        }
    }
}

impl Element {
    fn full_markup_visitor<V: MarkupVisitor>(self, visitor: &mut V) -> Node {
        let Element { tag, attributes, children } = self;
        let children = children
            .flatten()
            .into_iter()
            .map(|element| {
                element.full_markup_visitor(visitor)
            })
            .collect::<Vec<_>>();
        let children = Fragment::from_nodes(children);
        let children = Fragment::from_nodes(visitor.visit_fragment(children).flatten());
        visitor.visit_element(tag, attributes, children)
    }
}

impl Fragment {
    fn full_markup_visitor<V: MarkupVisitor>(self, visitor: &mut V) -> Node {
        let nodes = self
            .into_iter()
            .map(|element| {
                element.full_markup_visitor(visitor)
            })
            .flat_map(|node| {
                match node {
                    Node::Fragment(fragment) => fragment.to_vec(),
                    node => vec![node]
                }
            })
            .collect::<Vec<_>>();
        visitor.visit_fragment(Fragment::from_nodes(nodes))
    }
}
