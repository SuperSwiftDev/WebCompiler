use std::collections::HashMap;

use crate::{Element, Html, TagBuf};

pub trait HtmlVisitor {
    fn visit_html_element(
        &self,
        tag: TagBuf,
        attrs: HashMap<String, String>,
        children: Vec<Html>,
    ) -> Html {
        Html::Element(Element { tag, attrs, children })
    }
    fn visit_html_fragment(&self, fragment: Vec<Html>) -> Html {
        Html::Fragment(fragment)
    }
    fn visit_html_text(&self, text: String) -> Html {
        Html::Text(text)
    }
}

pub trait ElementVisitor {
    fn visit_element(
        &mut self,
        tag: TagBuf,
        attrs: HashMap<String, String>,
        children: Vec<Html>,
    ) -> Html {
        Html::Element(Element { tag, attrs, children })
    }
}

impl Html {
    pub fn apply_html_visitor<V: HtmlVisitor>(self, visitor: &V) -> Self {
        match self {
            Self::Element(Element { tag, attrs, children }) => {
                let children = children
                    .into_iter()
                    .map(|node| node.apply_html_visitor(visitor))
                    .collect::<Vec<_>>();
                visitor.visit_html_element(tag, attrs, children)
            }
            Self::Fragment(nodes) => {
                let nodes = nodes
                    .into_iter()
                    .map(|x| x.apply_html_visitor(visitor))
                    .collect::<Vec<_>>();
                visitor.visit_html_fragment(nodes)
            }
            Self::Text(text) => {
                visitor.visit_html_text(text)
            }
        }
    }
    pub fn apply_element_visitor<V: ElementVisitor>(self, visitor: &mut V) -> Self {
        match self {
            Self::Element(Element { tag, attrs, children }) => {
                let children = children
                    .into_iter()
                    .map(|node| node.apply_element_visitor(visitor))
                    .collect::<Vec<_>>();
                visitor.visit_element(tag, attrs, children)
            }
            Self::Fragment(nodes) => {
                let nodes = nodes
                    .into_iter()
                    .map(|x| x.apply_element_visitor(visitor))
                    .collect::<Vec<_>>();
                Html::Fragment(nodes)
            }
            Self::Text(text) => {
                Html::Text(text)
            }
        }
    }
}

