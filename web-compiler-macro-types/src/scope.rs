use std::collections::BTreeMap;
use xml_ast::Node;


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



