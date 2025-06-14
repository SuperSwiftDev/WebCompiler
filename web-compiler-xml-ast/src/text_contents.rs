use crate::{Element, Fragment, Node};

impl Node {
    pub fn text_contents(&self) -> Vec<String> {
        match self {
            Self::Text(text) => vec![text.to_string()],
            Self::Element(element) => element.children.text_contents(),
            Self::Fragment(fragment) => fragment.text_contents(),
        }
    }
}

impl Element {
    pub fn text_contents(&self) -> Vec<String> {
        self.children.text_contents()
    }
}

impl Fragment {
    pub fn text_contents(&self) -> Vec<String> {
        self
            .iter()
            .flat_map(|x| x.text_contents())
            .collect::<Vec<_>>()
    }
}

