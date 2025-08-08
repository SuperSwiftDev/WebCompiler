use std::collections::HashSet;

use web_compiler_io_types::IO;

use crate::breadcrumbs::BreadcrumbPathListValue;
use crate::macro_tag::MacroTagSet;
use crate::scope::{BinderValue, BindingScope};
use crate::context::ContextRegistry;

use crate::project::{DependencyRelation, FileInput, ProjectContext};
use crate::tag_rewrite_rule::TagRewriteRuleSet;

pub use web_compiler_io_types::Effectful;

// ————————————————————————————————————————————————————————————————————————————
// TOP-DOWN CONTEXT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct HostInfo {
    breadcrumbs: Option<BreadcrumbPathListValue>,
    hoisted: Vec<BinderValue>,
}

impl HostInfo {
    pub fn new(
        breadcrumbs: Option<BreadcrumbPathListValue>,
        hoisted: Vec<BinderValue>,
    ) -> Self {
        Self { breadcrumbs, hoisted }
    }
    pub fn breadcrumbs(&self) -> Option<&BreadcrumbPathListValue> {
        self.breadcrumbs.as_ref()
    }
    pub fn hoisted(&self) -> &[BinderValue] {
        &self.hoisted
    }

    pub fn with_chained_state(mut self, chained_state: ChainedState) -> Self {
        self.hoisted = chained_state.hoisted;
        self
    }
    pub fn with_breadcrumbs(mut self, breadcrumbs: BreadcrumbPathListValue) -> Self {
        self.breadcrumbs = Some(breadcrumbs);
        self
    }
    pub fn with_breadcrumb_opt(mut self, breadcrumbs: Option<BreadcrumbPathListValue>) -> Self {
        self.breadcrumbs = breadcrumbs.or_else(|| self.breadcrumbs);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ProcessScope {
    pub binding_scope: BindingScope,
    pub context_registry: ContextRegistry,
    host_info: HostInfo,
}

impl ProcessScope {
    pub fn new(host_info: HostInfo) -> Self {
        let breadcrumbs = host_info.breadcrumbs.clone();
        Self {
            binding_scope: Default::default(),
            context_registry: Default::default(),
            host_info,
        }
        .and_insert_binder_value(
            "breadcrumbs",
            BinderValue::json(breadcrumbs)
        )
    }
    // pub fn fresh(self) -> Self {
    //     Self::new(self.host_info)
    // }
    pub fn with_binding_scope(mut self, binding_scope: BindingScope) -> Self {
        self.binding_scope = binding_scope;
        self
    }
    pub fn and_insert_binder_value(mut self, key: impl Into<String>, value: impl Into<BinderValue>) -> Self {
        self.binding_scope.insert(key, value);
        self
    }
    pub fn with_context_registry(mut self, context_registry: ContextRegistry) -> Self {
        self.context_registry = context_registry;
        self
    }
    pub fn host_info(&self) -> &HostInfo {
        &self.host_info
    }
}

// ————————————————————————————————————————————————————————————————————————————
// HOISTED STATE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct ChainedState {
    hoisted: Vec<BinderValue>,
}

impl ChainedState {
    pub fn push_hoisted(&mut self, new: impl Into<BinderValue>) {
        self.hoisted.push(new.into());
    }
    pub fn lookup_hoisted_first_where(&mut self, predicate: impl Fn(&BinderValue) -> bool) -> Option<&BinderValue> {
        self.hoisted.iter().find(|x| predicate(x))
    }
    pub fn hoisted(&self) -> &[BinderValue] {
        &self.hoisted
    }
}

// ————————————————————————————————————————————————————————————————————————————
// BOTTOM-UP STATE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct AccumulatedEffects {
    pub dependencies: HashSet<DependencyRelation>,
    pub deferred_dependencies: HashSet<DependencyRelation>,
    pub hoisted: Vec<BinderValue>,
}

impl AccumulatedEffects {
    pub fn chained_state(&self) -> ChainedState {
        ChainedState {
            hoisted: self.hoisted.clone(),
        }
    }
}

impl Effectful for AccumulatedEffects {
    fn extend(&mut self, other: Self) {
        self.dependencies.extend(other.dependencies);
        self.deferred_dependencies.extend(other.deferred_dependencies);
        self.hoisted.extend(other.hoisted);
    }
}

pub type MacroIO<T> = IO<T, AccumulatedEffects>;

// ————————————————————————————————————————————————————————————————————————————
// MACRO RUNTIME
// ————————————————————————————————————————————————————————————————————————————

pub trait Featureset: Sized + Clone {
    type Runtime: SourceHost;
    fn macros(&self) -> &MacroTagSet<Self::Runtime>;
    fn rules(&self) -> &TagRewriteRuleSet<Self::Runtime>;
}

pub trait SourceHost: Featureset {
    fn project(&self) -> &ProjectContext;
    fn source_file(&self) -> &FileInput;
    fn fork(&self, file_input: &FileInput) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct SourceHostRef<'a> {
    pub project_context: &'a ProjectContext,
    pub file_input: &'a FileInput,
}

impl<'a> SourceHostRef<'a> {
    pub fn new(project_context: &'a ProjectContext, file_input: &'a FileInput) -> Self {
        Self { project_context, file_input }
    }
    pub fn new_source_file(&self, file_input: &'a FileInput) -> Self {
        Self { project_context: self.project_context, file_input }
    }
    pub fn project_context(&self) -> &ProjectContext {
        self.project_context
    }
    pub fn file_input(&self) -> &FileInput {
        self.file_input
    }
}

// ————————————————————————————————————————————————————————————————————————————
// POST-PROCESSING CONTEXT
// ————————————————————————————————————————————————————————————————————————————

/// Provides path resolution for source files and dependencies relative to output structure.
#[derive(Debug, Clone, Copy)]
pub struct SourcePathResolver<'a> {
    /// List of source input files (e.g. HTML, JS, CSS).
    pub inputs: &'a [FileInput],
    /// List of paths referenced by input files (HTML, CSS, etc.) that do not have their own output rules.
    /// 
    /// May include:
    /// - Public assets (e.g. <img src="logo.png">)
    /// - Internal fragments (e.g. <include src="partial.html">)
    /// - Template data or modules
    ///
    /// Used for:
    /// - Path rewriting
    /// - Build graph traversal
    /// - Invalidation and dependency tracking
    pub dependencies: &'a [DependencyRelation],
    /// Project-wide layout context (project root + output dir + host source info).
    pub source_host: SourceHostRef<'a>,
}

impl<'a> SourcePathResolver<'a> {
    pub fn lookup_input_rule(&self, relation: &DependencyRelation) -> Option<&'a FileInput> {
        let dependency = relation.as_file_dependency();
        let target = {
            let parent_dir = dependency.from.parent().unwrap();
            path_clean::clean(parent_dir.join(dependency.to.as_path()))
        };
        self.inputs
            .iter()
            .find(|input| {
                path_clean::clean(&input.source) == target
            })
    }
    pub fn lookup_dependency(&self, relation: &DependencyRelation) -> Option<&'a DependencyRelation> {
        let dependency = relation.as_file_dependency();
        let target = path_clean::clean(dependency.resolved_target_path());
        self.dependencies
            .iter()
            .find(|dep| {
                path_clean::clean(dep.as_file_dependency().resolved_target_path()) == target
            })
    }
}
