use std::fmt::Debug;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};
use crate::{AttributeKeyBuf, AttributeMap, AttributeValueBuf, TagBuf};

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — XML NODES
// ————————————————————————————————————————————————————————————————————————————


#[derive(Clone)]
pub enum Node {
    Text(String),
    Element(Element),
    Fragment(Fragment),
}

impl Node {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }
    pub fn element(
        tag: impl Into<TagBuf>,
        attributes: impl Into<AttributeMap>,
        children: impl Into<Fragment>,
    ) -> Self {
        Self::Element(Element {
            tag: tag.into(),
            attributes: attributes.into(),
            children: children.into(),
        })
    }
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(x) => Some(x.as_str()),
            _ => None,
        }
    }
    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Self::Element(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_fragment(&self) -> Option<&Fragment> {
        match self {
            Self::Fragment(x) => Some(x),
            _ => None,
        }
    }
    pub fn to_element(self) -> Option<Element> {
        match self {
            Self::Element(x) => Some(x),
            _ => None,
        }
    }
    pub fn lookup_element_attribute(&self, key: impl AsRef<str>) -> Option<&str> {
        self.as_element()
            .and_then(|element| {
                element.attributes.get(key)
            })
            .map(|value| value.as_str())
    }
    pub fn empty() -> Self {
        Self::Fragment(Fragment::empty())
    }
    pub fn extract_elements(self) -> Vec<Element> {
        match self {
            Node::Element(x) => vec![x],
            Node::Fragment(xs) => xs.extract_elements(),
            Node::Text(_) => Vec::new(),
        }
    }
    pub fn extract_text_strict(self) -> Result<Vec<String>, ()> {
        match self {
            Node::Element(_) => Err(()),
            Node::Fragment(xs) => xs.extract_text_strict(),
            Node::Text(x) => Ok(vec![x]),
        }
    }
    pub fn flatten(self) -> Vec<Node> {
        match self {
            Self::Text(text) => vec![Self::Text(text)],
            Self::Element(element) => vec![Self::Element(element)],
            Self::Fragment(fragment) => fragment.flatten(),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => text.fmt(f),
            Self::Element(element) => element.fmt(f),
            Self::Fragment(nodes) => nodes.fmt(f),
        }
    }
}

impl From<Element> for Node {
    fn from(value: Element) -> Self {
        Node::Element(value)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — XML ELEMENTS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Clone)]
pub struct Element {
    pub tag: TagBuf,
    pub attributes: AttributeMap,
    pub children: Fragment,
}

impl Element {
    pub fn new(tag: impl Into<TagBuf>) -> Self {
        Element { tag: tag.into(), attributes: Default::default(), children: Default::default() }
    }
    pub fn with_attributes(mut self, attributes: AttributeMap) -> Self {
        self.attributes.extend(attributes);
        self
    }
    pub fn with_attribute(mut self, key: impl Into<AttributeKeyBuf>, value: impl Into<AttributeValueBuf>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
    pub fn with_children(mut self, children: impl IntoIterator<Item=Node>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn extract_child_elements(self) -> Vec<Element> {
        self.children.extract_elements()
    }
    pub fn extract_child_text_strict(self) -> Result<Vec<String>, ()> {
        self.children.extract_text_strict()
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Element");
        dbg.field("tag", &self.tag);

        if !self.attributes.is_empty() {
            dbg.field("attributes", &self.attributes);
        }

        if !self.children.is_empty() {
            dbg.field("children", &self.children);
        }

        dbg.finish()
    }
}



// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — XML FRAGMENTS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Clone, Default)]
pub struct Fragment {
    nodes: Vec<Node>,
}

impl Fragment {
    pub fn extract_elements(self) -> Vec<Element> {
        self
            .to_vec()
            .into_iter()
            .flat_map(|node| {
                match node {
                    Node::Element(x) => vec![x],
                    Node::Fragment(xs) => xs.extract_elements(),
                    Node::Text(_) => Vec::default(),
                }
            })
            .collect::<Vec<_>>()
    }
    pub fn extract_text_strict(self) -> Result<Vec<String>, ()> {
        let mut results = Vec::<String>::with_capacity(self.len());
        for node in self.to_vec() {
            match node {
                Node::Element(_) => return Err(()),
                Node::Fragment(xs) => {
                    results.extend(xs.extract_text_strict()?);
                },
                Node::Text(x) => {
                    results.push(x);
                },
            }
        }
        Ok(results)
    }
    pub fn flatten(self) -> Vec<Node> {
        self
            .to_vec()
            .into_iter()
            .flat_map(|node| {
                node.flatten()
            })
            .collect::<Vec<_>>()
    }
}

impl Fragment {
    pub fn empty() -> Self {
        Self { nodes: Vec::with_capacity(0) }
    }
    pub fn as_node_slice(&self) -> &[Node] {
        &self.nodes
    }
    pub fn from_nodes(nodes: impl Into<Vec<Node>>) -> Self {
        Self { nodes: nodes.into() }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }

    pub fn insert(&mut self, index: usize, node: Node) {
        self.nodes.insert(index, node);
    }

    pub fn remove(&mut self, index: usize) -> Node {
        self.nodes.remove(index)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn iter(&self) -> Iter<'_, Node> {
        self.nodes.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Node> {
        self.nodes.iter_mut()
    }

    pub fn to_vec(self) -> Vec<Node> {
        self.nodes
    }

    pub fn retain<F: FnMut(&Node) -> bool>(&mut self, f: F) {
        self.nodes.retain(f);
    }


    pub fn truncate(&mut self, len: usize) {
        self.nodes.truncate(len);
    }

    pub fn resize(&mut self, new_len: usize, value: Node) 
    where
        Node: Clone, // needed because `resize` clones `value`
    {
        self.nodes.resize(new_len, value);
    }

    pub fn append(&mut self, other: &mut Fragment) {
        self.nodes.append(&mut other.nodes);
    }
}

impl Fragment {
    /// Creates an empty `Fragment` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the fragment can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Reserves capacity for at least `additional` more nodes.
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    /// Reserves the minimum capacity for exactly `additional` more nodes.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.nodes.reserve_exact(additional);
    }

    /// Shrinks the capacity of the fragment as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
    }

    /// Shrinks the capacity to at least `min_capacity`.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.nodes.shrink_to(min_capacity);
    }
}


// Conversion APIs
impl From<Vec<Node>> for Fragment {
    fn from(nodes: Vec<Node>) -> Self {
        Self::from_nodes(nodes)
    }
}

impl From<Fragment> for Vec<Node> {
    fn from(fragment: Fragment) -> Self {
        fragment.to_vec()
    }
}

impl Index<usize> for Fragment {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Fragment {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl IntoIterator for Fragment {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Node>;
    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<'a> IntoIterator for &'a Fragment {
    type Item = &'a Node;
    type IntoIter = Iter<'a, Node>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Fragment {
    type Item = &'a mut Node;
    type IntoIter = IterMut<'a, Node>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Extend support
impl Extend<Node> for Fragment {
    fn extend<T: IntoIterator<Item = Node>>(&mut self, iter: T) {
        self.nodes.extend(iter);
    }
}

impl FromIterator<Node> for Fragment {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Element> for Fragment {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().map(Node::Element).collect(),
        }
    }
}

impl AsRef<[Node]> for Fragment {
    fn as_ref(&self) -> &[Node] {
        &self.nodes
    }
}

impl AsMut<[Node]> for Fragment {
    fn as_mut(&mut self) -> &mut [Node] {
        &mut self.nodes
    }
}

impl Debug for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nodes.fmt(f)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// GENERAL HELPERS
// ————————————————————————————————————————————————————————————————————————————


