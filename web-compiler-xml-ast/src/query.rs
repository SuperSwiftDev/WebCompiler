use crate::{Element, Fragment, Node, TagBuf};

impl Node {
    pub fn find_first(&self, target: &TagBuf) -> Option<Node> {
        match self {
            Self::Element(element) => element.find_first(target),
            Self::Fragment(fragment) => fragment.find_first(target),
            Self::Text(_) => None,
        }
    }
}

impl Element {
    pub fn find_first(&self, target: &TagBuf) -> Option<Node> {
        if self.tag.matches(target) {
            return Some(Node::Element(self.to_owned()))
        }
        self.children.find_first(target)
    }
}

impl Fragment {
    pub fn find_first(&self, target: &TagBuf) -> Option<Node> {
        self.iter()
            .find_map(|x| x.find_first(target))
    }
}