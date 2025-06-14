use std::collections::BTreeMap;
use xml_ast::{Fragment, Node};


// ————————————————————————————————————————————————————————————————————————————
// TYPING PRIMITIVES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum BinderType {
    Markup(MarkupBinderType),
    Json(JsonBinderType),
}

#[derive(Debug, Clone)]
pub enum MarkupBinderType {
    Text,
    Element,
    Fragment,
}

#[derive(Debug, Clone)]
pub enum JsonBinderType {
    Null,
    Bool,
    Number,
    String,
    Array(Option<Box<JsonBinderType>>),
    Object(Option<RecordType<JsonBinderType>>),
}

#[derive(Debug, Clone)]
pub struct RecordType<Kind> {
    pub fields: BTreeMap<String, Kind>,
}

// ————————————————————————————————————————————————————————————————————————————
// VALUE PRIMITIVES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum BinderValue {
    Markup(MarkupBinderValue),
    Json(JsonBinderValue),
}

impl BinderValue {
    pub fn markup_node(node: Node) -> Self {
        Self::Markup(MarkupBinderValue(node))
    }
    pub fn fragment(nodes: Vec<Node>) -> Self {
        Self::Markup(MarkupBinderValue(Node::Fragment(Fragment::from_nodes(nodes))))
    }
    // pub fn object(map: impl IntoIterator<Item = (String, )>)
    pub fn as_markup(&self) -> Option<&MarkupBinderValue> {
        match self {
            Self::Markup(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_node(&self) -> Option<&Node> {
        match self {
            Self::Markup(x) => Some(&x.0),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkupBinderValue(pub Node);

#[derive(Debug, Clone)]
pub enum JsonBinderValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<JsonBinderValue>),
    Object(BTreeMap<String, JsonBinderValue>),
}

// ————————————————————————————————————————————————————————————————————————————
// BINDING SCOPE ENVIRONMENT
// ————————————————————————————————————————————————————————————————————————————

/// Lexical binding scope.
#[derive(Debug, Clone, Default)]
pub struct BindingScope {
    environment: BTreeMap<String, BinderValue>,
}

impl BindingScope {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<BinderValue>) -> Option<BinderValue> {
        let key = key.into();
        let value = value.into();
        self.environment.insert(key, value)
    }
    pub fn lookup(&self, key: impl AsRef<str>) -> Option<&BinderValue> {
        self.environment.get(key.as_ref())
    }
}



