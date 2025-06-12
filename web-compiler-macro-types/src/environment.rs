use std::collections::BTreeSet;

use crate::scope::BindingScope;
use crate::context::ContextRegistry;

use crate::project::DependencyRelation;

#[derive(Debug, Clone, Default)]
pub struct LexicalEnvironment {
    pub binding_scope: BindingScope,
    pub context_registry: ContextRegistry,
}

#[derive(Debug, Clone, Default)]
pub struct AccumulatedEffects {
    pub dependencies: BTreeSet<DependencyRelation>,
}
