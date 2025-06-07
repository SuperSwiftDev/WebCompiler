use std::{borrow::{Borrow, Cow}, collections::HashMap};
use pretty_tree::ToPrettyTree;

// #[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;

mod html_parser;
mod html_parser2;
mod constants;

pub mod traversal;
pub mod transform;
pub mod html_string;

pub use constants::*;

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — BASICS
// ————————————————————————————————————————————————————————————————————————————
pub trait TagIdentifier {
    fn as_original(&self) -> &str;

    /// Case-insensitive normalized form. Returns a `Cow` so implementers can choose
    /// to precompute (borrowed) or allocate (owned).
    fn as_normalized(&self) -> Cow<'_, str>;

    /// Case-insensitive match against another TagIdentifier
    fn matches_tag<T: TagIdentifier>(&self, other: T) -> bool {
        self.as_normalized() == other.as_normalized()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagBuf {
    original: String,
    normalized: String,
}

impl TagBuf {
    pub fn new(tag: impl Into<String>) -> Self {
        let original = tag.into();
        let normalized = original.to_ascii_lowercase();
        Self { original, normalized }
    }

    pub fn as_str(&self) -> &str {
        &self.original
    }

    pub fn as_normalized(&self) -> &str {
        &self.normalized
    }
    pub fn as_ref(&self) -> TagRef<'_> {
        TagRef {
            original: &self.original,
            normalized: &self.normalized,
        }
    }
    // pub fn as_tag(&self) -> &Tag {
    //     // SAFETY: Tag is #[repr(transparent)] around str
    //     unsafe { &*(self.original.as_str() as *const str as *const Tag) }
    // }
}

impl TagIdentifier for TagBuf {
    fn as_original(&self) -> &str {
        &self.original
    }

    fn as_normalized(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.normalized)
    }
}

// impl Deref for TagBuf {
//     type Target = Tag;

//     fn deref(&self) -> &Self::Target {
//         self.as_tag()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagRef<'a> {
    original: &'a str,
    normalized: &'a str,
}

impl<'a> TagRef<'a> {
    pub fn from_tag(tag: &'a TagBuf) -> Self {
        Self {
            original: &tag.original,
            normalized: &tag.normalized,
        }
    }

    pub fn matches(&self, other: impl AsRef<str>) -> bool {
        self.normalized == other.as_ref().to_ascii_lowercase()
    }

    pub fn as_str(&self) -> &'a str {
        self.original
    }

    pub fn as_normalized(&self) -> &'a str {
        self.normalized
    }
    pub fn as_ref(&self) -> TagRef<'_> {
        TagRef {
            original: &self.original,
            normalized: &self.normalized,
        }
    }

}

impl<'a> TagIdentifier for TagRef<'a> {
    fn as_original(&self) -> &str {
        self.original
    }

    fn as_normalized(&self) -> Cow<'a, str> {
        Cow::Borrowed(self.normalized)
    }
}

// pub fn compare_tag(a: &impl TagIdentifier, b: &impl TagIdentifier) -> bool {
//     a.as_normalized() == b.as_normalized()
// }
pub fn compare_tag(a: &impl TagIdentifier, b: &impl TagIdentifier) -> bool {
    a.as_normalized() == b.as_normalized()
}

impl<'a> TagIdentifier for &'a TagBuf {
    fn as_original(&self) -> &str {
        &self.original
    }

    fn as_normalized(&self) -> Cow<'a, str> {
        Cow::Borrowed(&self.normalized)
    }
}

impl<'a, 'b> TagIdentifier for &'a TagRef<'b> {
    fn as_original(&self) -> &str {
        self.original
    }

    fn as_normalized(&self) -> Cow<'a, str> {
        Cow::Borrowed(self.normalized)
    }
}

impl From<&str> for TagBuf {
    fn from(s: &str) -> Self {
        TagBuf::new(s)
    }
}

impl From<String> for TagBuf {
    fn from(s: String) -> Self {
        TagBuf::new(s)
    }
}

impl From<&String> for TagBuf {
    fn from(s: &String) -> Self {
        TagBuf::new(s.to_owned())
    }
}

impl Borrow<str> for TagBuf {
    fn borrow(&self) -> &str {
        &self.original
    }
}

// impl Borrow<Tag> for TagBuf {
//     fn borrow(&self) -> &Tag {
//         self.as_tag()
//     }
// }
// impl AsRef<Tag> for TagBuf {
//     fn as_ref(&self) -> &Tag {
//         self.as_tag()
//     }
// }

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — HTML AST
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum Html {
    Element(Element),
    Text(String),
    Fragment(Vec<Html>),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: TagBuf,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Html>,
}

impl Html {
    pub fn empty() -> Html {
        Html::Fragment(Vec::default())
    }
    pub fn parse(source: &str, mode: ParserMode) -> Html {
        let result = match mode {
            ParserMode::Document => Self::parse_document(source),
            ParserMode::Fragment { context } => Self::parse_fragment(source, &context)
        };
        result
    }
    fn parse_fragment(source: &str, context: &str) -> Html {
        crate::html_parser2::parse_html_fragment(source, context).normalize()
        // let result = crate::html_parser::parse_html_str(source).payload;
        // if result.len() == 1 {
        //     return result[0].to_owned()
        // }
        // Html::Fragment(result)
    }
    fn parse_document(source: &str) -> Html {
        crate::html_parser2::parse_html_document(source).normalize()
        // let result = crate::html_parser::parse_html_str(source);
        // if result.payload.len() == 1 {
        //     return result.payload.get(0).unwrap().clone()
        // }
        // Html::Fragment(result.payload)
    }
    pub fn text_contents(&self) -> Result<String, ()> {
        match self {
            Self::Element(x) => x.to_text(),
            Self::Text(x) => Ok(x.to_owned()),
            Self::Fragment(xs) => fragment_to_text(xs),
        }
    }
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        match self {
            Self::Element(element) => element.get_attribute(key),
            Self::Fragment(xs) if xs.len() == 1 => xs.first().and_then(|x| x.get_attribute(key)),
            Self::Fragment(_) => None,
            Self::Text(_) => None,
        }
    }
    pub fn as_fragment(&self) -> Option<&Vec<Html>> {
        match self {
            Self::Fragment(xs) => Some(xs),
            _ => None,
        }
    }
}

impl Element {
    pub fn to_text(&self) -> Result<String, ()> {
        fragment_to_text(&self.children)
    }
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }
    /// Case-insensitive match against another TagIdentifier
    pub fn matches_tag<T: TagIdentifier>(&self, other: T) -> bool {
        self.tag.matches_tag(other)
    }
}

fn fragment_to_text(nodes: &[Html]) -> Result<String, ()> {
    let results = nodes
        .into_iter()
        .map(|x| x.text_contents())
        .collect::<Vec<_>>();
    let mut output = String::default();
    for result in results {
        match result {
            Ok(txt) => {
                output.push_str(&txt);
            }
            Err(_) => {
                return Err(())
            }
        }
    }
    Ok(output)
}

// ————————————————————————————————————————————————————————————————————————————
// CONVERSTION
// ————————————————————————————————————————————————————————————————————————————
impl crate::html_parser2::Html {
    fn normalize(self) -> Html {
        match self {
            crate::html_parser2::Html::Element(element) => element.normalize(),
            crate::html_parser2::Html::Fragment(nodes) => {
                let nodes = nodes.into_iter().map(|x| x.normalize()).collect();
                Html::Fragment(nodes)
            },
            crate::html_parser2::Html::Text(text) => Html::Text(text),
        }
    }
}

impl crate::html_parser2::Element {
    fn normalize(self) -> Html {
        let children = self.children
            .into_iter()
            .map(|x| x.normalize())
            .collect();
        Html::Element(
            Element {
                tag: self.tag,
                attrs: self.attrs,
                children: children,
            }
        )
    }
}

// ————————————————————————————————————————————————————————————————————————————
// DEBUG
// ————————————————————————————————————————————————————————————————————————————
impl ToPrettyTree for Html {
    fn to_pretty_tree(&self) -> pretty_tree::PrettyTree {
        match self {
            Self::Element(element) => element.to_pretty_tree(),
            Self::Text(text) => {
                pretty_tree::PrettyTree::str(text)
            }
            Self::Fragment(nodes) => {
                let nodes = nodes
                    .iter()
                    .map(|x| x.to_pretty_tree())
                    .collect::<Vec<_>>();
                pretty_tree::PrettyTree::fragment(nodes)
            }
        }
    }
}

impl ToPrettyTree for Element {
    fn to_pretty_tree(&self) -> pretty_tree::PrettyTree {
        let attrs = self.attrs
            .iter()
            .map(|(key, value)| {
                format!("{key} = {value:?}")
            })
            .collect::<Vec<_>>()
            .join(" ");
        let mut children = self.children
            .iter()
            .map(|x| x.to_pretty_tree())
            .collect::<Vec<_>>();
        if attrs.is_empty() {
            let label = format!("{}:", self.tag.as_original());
            return pretty_tree::PrettyTree::branch_of(label, &children)
        }
        if attrs.len() > 80 {
            let label = format!("{}:", self.tag.as_original());
            let mut content = vec![
                pretty_tree::PrettyTree::leaf(format!("[ {attrs} ]")),
            ];
            content.append(&mut children);
            return pretty_tree::PrettyTree::branch_of(label, &content)
        }
        let label = format!("{} [ {attrs} ]:", self.tag.as_original());
        return pretty_tree::PrettyTree::branch_of(label, &children)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// HTML API UTILITIES
// ————————————————————————————————————————————————————————————————————————————

impl Html {
    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Self::Element(element) => Some(element),
            _ => None,
        }
    }
    pub fn is_inline_element(&self) -> bool {
        self.as_element().map(|x| x.is_inline_node()).unwrap_or(false)
    }
    pub fn is_header_element(&self) -> bool {
        self.as_element().map(|x| x.is_header_tag()).unwrap_or(false)
    }
    pub fn is_void_element(&self) -> bool {
        self.as_element().map(|x| x.is_void_tag()).unwrap_or(false)
    }
}

impl Element {
    pub fn is_inline_node(&self) -> bool {
        is_inline_tag(&self.tag)
    }

    pub fn is_header_tag(&self) -> bool {
        is_header_tag(&self.tag)
    }

    pub fn is_void_tag(&self) -> bool {
        is_void_tag(&self.tag)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReduceWhitespaceEnv {
    scope: Vec<TagBuf>,
}

impl ReduceWhitespaceEnv {
    pub fn new_scope(&self, tag: &TagBuf) -> Self {
        let mut copy = self.clone();
        copy.scope.push(tag.clone());
        copy
    }
    pub fn is_inline_mode(&self) -> bool {
        self.scope
            .last()
            .map(|tag| is_inline_tag(tag))
            .unwrap_or(false)
    }
}

impl ReduceWhitespaceEnv {
    fn reduce_whitespace_for_fragment(&self, nodes: Vec<Html>) -> Vec<Html> {
        let mut output = Vec::with_capacity(nodes.len());
        let mut buffer: Option<String> = None;

        let is_inline = self.is_inline_mode();

        for node in nodes {
            match node {
                Html::Text(t) => {
                    let trimmed = t.trim();

                    if trimmed.is_empty() {
                        // Aggressively drop whitespace-only text nodes in block mode
                        if is_inline {
                            buffer = Some(" ".to_string()); // retain single space
                        } else {
                            // discard entirely in block context
                            continue;
                        }
                    } else {
                        let text = if is_inline {
                            trimmed.to_string()
                        } else {
                            trimmed.to_string()
                        };

                        // merge with buffer
                        if let Some(prev) = buffer.take() {
                            output.push(Html::Text(prev + &text));
                        } else {
                            output.push(Html::Text(text));
                        }
                    }
                }
                Html::Element(el) => {
                    // flush buffered space before element if inline
                    if let Some(prev) = buffer.take() {
                        if is_inline {
                            output.push(Html::Text(prev));
                        }
                    }

                    output.push(Html::Element(el.reduce_whitespace(self)));
                }
                Html::Fragment(frag) => {
                    let children = self.reduce_whitespace_for_fragment(frag);
                    if let Some(prev) = buffer.take() {
                        if is_inline {
                            output.push(Html::Text(prev));
                        }
                    }
                    output.push(Html::Fragment(children));
                }
            }
        }

        // flush final buffer
        if let Some(final_text) = buffer {
            if is_inline {
                output.push(Html::Text(final_text));
            }
        }

        output
    }
}


impl Element {
    /// Generally for debugging.
    pub fn reduce_whitespace(self, env: &ReduceWhitespaceEnv) -> Self {
        let child_env = env.new_scope(&self.tag);
        let children = child_env.reduce_whitespace_for_fragment(self.children);
        Self { children, ..self }
    }
}

impl Html {
    /// Generally for debugging.
    pub fn reduce_whitespace(self, env: &ReduceWhitespaceEnv) -> Self {
        match self {
            Self::Element(element) => Self::Element(element.reduce_whitespace(env)),
            Self::Fragment(nodes) => {
                let nodes = nodes
                    .into_iter()
                    .map(|x| x.reduce_whitespace(env))
                    .collect::<Vec<_>>();
                Self::Fragment(nodes)
            }
            Self::Text(text) => Self::Text(text),
        }
    }
}

impl Html {
    pub fn cleanup_fragments(self) -> Self {
        match self {
            Html::Fragment(nodes) => {
                let mut flat = Vec::new();

                for node in nodes {
                    match node.cleanup_fragments() {
                        Html::Fragment(inner) => {
                            flat.extend(inner);
                        }
                        other => {
                            flat.push(other);
                        }
                    }
                }

                match flat.len() {
                    0 => Html::Fragment(vec![]),
                    1 => flat.into_iter().next().unwrap(),
                    _ => Html::Fragment(flat),
                }
            }
            Html::Element(el) => Html::Element(el.cleanup_fragments()),
            Html::Text(_) => self,
        }
    }
}


impl Element {
    pub fn cleanup_fragments(mut self) -> Self {
        self.children = self.children
            .into_iter()
            .flat_map(|child| match child.cleanup_fragments() {
                Html::Fragment(nodes) => nodes, // flatten
                other => vec![other],
            })
            .collect();
        self
    }
}


impl Html {
    /// Generally for debugging.
    pub fn compact(self, env: &ReduceWhitespaceEnv) -> Self {
        let cleaned = self.reduce_whitespace(env).cleanup_fragments();

        match cleaned {
            Html::Fragment(nodes) => {
                let trimmed = ReduceWhitespaceEnv::trim_whitespace_edges(nodes);
                match trimmed.len() {
                    0 => Html::Fragment(vec![]),
                    1 => trimmed.into_iter().next().unwrap(),
                    _ => Html::Fragment(trimmed),
                }
            }
            other => other,
        }
    }
}

impl ReduceWhitespaceEnv {
    pub fn trim_whitespace_edges(mut nodes: Vec<Html>) -> Vec<Html> {
        // Remove leading whitespace text nodes
        while let Some(Html::Text(t)) = nodes.first() {
            if t.trim().is_empty() {
                nodes.remove(0);
            } else {
                break;
            }
        }

        // Remove trailing whitespace text nodes
        while let Some(Html::Text(t)) = nodes.last() {
            if t.trim().is_empty() {
                nodes.pop();
            } else {
                break;
            }
        }

        nodes
    }
}




// ————————————————————————————————————————————————————————————————————————————
// PARSER
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserMode {
    Document, Fragment { context: String }
}

impl ParserMode {
    pub fn fragment(context: impl AsRef<str>) -> Self {
        Self::Fragment { context: context.as_ref().to_string() }
    }
}
