use std::collections::BTreeMap;
use serde::Serialize;
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
    pub fn node(node: impl Into<Node>) -> Self {
        Self::Markup(MarkupBinderValue(node.into()))
    }
    pub fn fragment(nodes: Vec<Node>) -> Self {
        Self::Markup(MarkupBinderValue(Node::Fragment(Fragment::from_nodes(nodes))))
    }
    pub fn object(map: impl IntoIterator<Item = (String, JsonBinderValue)>) -> Self {
        let map = map.into_iter().collect::<BTreeMap<_, _>>();
        Self::Json(JsonBinderValue::Object(map))
    }
    pub fn json_string(text: impl Into<String>) -> Self {
        Self::Json(JsonBinderValue::String(text.into()))
    }
    pub fn json<T: Serialize>(value: T) -> Self {
        let value = serde_json::to_value(value).unwrap();
        Self::Json(JsonBinderValue::from_json_value(value))
    }
    pub fn as_markup(&self) -> Option<&MarkupBinderValue> {
        match self {
            Self::Markup(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_fragment(&self) -> Option<&Fragment> {
        match self {
            Self::Markup(MarkupBinderValue(node)) => node.as_fragment(),
            _ => None,
        }
    }
    pub fn as_node(&self) -> Option<&Node> {
        match self {
            Self::Markup(x) => Some(&x.0),
            _ => None,
        }
    }
    pub fn try_cast_to_string(&self) -> Option<&str> {
        match self {
            Self::Markup(MarkupBinderValue(Node::Text(text))) => Some(text),
            Self::Json(JsonBinderValue::String(string)) => Some(string),
            _ => None
        }
    }
    pub fn try_cast_to_boolean(&self) -> Option<bool> {
        match self {
            Self::Json(JsonBinderValue::Bool(bool)) => Some(*bool),
            _ => None
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

impl JsonBinderValue {
    pub fn json_string(text: impl Into<String>) -> Self {
        Self::String(text.into())
    }
    pub fn object(map: impl IntoIterator<Item = (String, JsonBinderValue)>) -> Self {
        let map = map.into_iter().collect::<BTreeMap<_, _>>();
        JsonBinderValue::Object(map)
    }
    pub fn from_json_value(value: impl Into<serde_json::Value>) -> Self {
        match value.into() {
            serde_json::Value::Null => {
                Self::Null
            }
            serde_json::Value::Bool(bool) => {
                Self::Bool(bool)
            }
            serde_json::Value::Number(number) => {
                let number = number.to_string();
                Self::Number(number)
            }
            serde_json::Value::String(string) => {
                Self::String(string)
            }
            serde_json::Value::Array(xs) => {
                let xs = xs
                    .into_iter()
                    .map(|x| Self::from_json_value(x))
                    .collect::<Vec<_>>();
                Self::Array(xs)
            }
            serde_json::Value::Object(xs) => {
                let xs = xs
                    .into_iter()
                    .map(|(k, v)| {
                        (k, Self::from_json_value(v))
                    })
                    .collect::<BTreeMap<_, _>>();
                Self::Object(xs)
            }
        }
    }
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
    pub fn extend(mut self, other: Self) -> Self {
        self.environment.extend(other.environment);
        self
    }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<BinderValue>) -> Option<BinderValue> {
        let key = key.into();
        let value = value.into();
        self.environment.insert(key, value)
    }
    pub fn lookup(&self, key: impl AsRef<str>) -> Option<&BinderValue> {
        self.environment.get(key.as_ref())
    }
}



