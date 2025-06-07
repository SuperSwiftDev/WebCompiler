use std::collections::{HashMap, HashSet};

use web_compiler_html_ast::Html;
use web_compiler_html_ast::transform::{EffectPropagator, IO};
use web_compiler_common::FileDependency;

#[derive(Debug, Clone)]
pub enum BinderValue {
    Literal(String),
    Object(HashMap<String, String>),
    Html(Html),
}

impl BinderValue {
    pub fn object(map: impl IntoIterator<Item = (String, String)>) -> Self {
        Self::Object(map.into_iter().collect::<HashMap<_, _>>())
    }
    pub fn fragment(nodes: Vec<Html>) -> Self {
        Self::Html(Html::Fragment(nodes))
    }
    pub fn as_object(&self) -> Option<&HashMap<String, String>> {
        match self {
            Self::Object(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_html(&self) -> Option<&Html> {
        match self {
            Self::Html(html) => Some(html),
            _ => None,
        }
    }
    pub fn get_attribute(&self, key: impl AsRef<str>) -> Option<&String> {
        match self {
            Self::Html(Html::Element(element)) => element.get_attribute(key.as_ref()),
            Self::Object(map) => map.get(key.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ScopeBindingEnv {
    pub bindings: HashMap<String, BinderValue>,
}

impl ScopeBindingEnv {
    pub fn lookup_binding(&self, key: &str) -> Option<&BinderValue> {
        self.bindings.get(key)
    }
    pub fn define_binding(&mut self, name: impl Into<String>, binding: BinderValue) {
        let _ = self.bindings.insert(name.into(), binding);
    }
    pub fn merge_mut(&mut self, other: Self) {
        self.bindings.extend(other.bindings);
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccumulatedEffects {
    pub file_dependencies: HashSet<FileDependency>,
}

impl AccumulatedEffects {
    pub fn with_file_dependency(mut self, file_dependency: FileDependency) -> Self {
        self.file_dependencies.insert(file_dependency);
        self
    }
    pub fn insert_file_dependency(&mut self, file_dependency: FileDependency) {
        self.file_dependencies.insert(file_dependency);
    }
}

impl EffectPropagator for AccumulatedEffects {
    fn merge_mut(&mut self, other: Self) {
        self.file_dependencies = self.file_dependencies.union(&other.file_dependencies).cloned().collect::<HashSet<_>>();
    }
}

pub type PreProcessIO<Value> = IO<Value, AccumulatedEffects>;




