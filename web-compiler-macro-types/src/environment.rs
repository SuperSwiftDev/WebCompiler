use std::collections::HashSet;

use web_compiler_io_types::IO;

use crate::macro_tag::MacroTagSet;
use crate::scope::BindingScope;
use crate::context::ContextRegistry;

use crate::project::{DependencyRelation, FileInput, ProjectContext};
use crate::tag_rewrite_rule::TagRewriteRuleSet;

pub use web_compiler_io_types::Effectful;

// ————————————————————————————————————————————————————————————————————————————
// SOURCE CONTEXT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy)]
pub struct SourceContext<'a> {
    pub project_context: &'a ProjectContext,
    pub file_input: &'a FileInput,
}

impl<'a> SourceContext<'a> {
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
// TOP-DOWN CONTEXT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct LexicalEnvironment {
    pub binding_scope: BindingScope,
    pub context_registry: ContextRegistry,
}

// ————————————————————————————————————————————————————————————————————————————
// BOTTOM-UP STATE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct AccumulatedEffects {
    pub dependencies: HashSet<DependencyRelation>,
    pub deferred_dependencies: HashSet<DependencyRelation>,
}

impl Effectful for AccumulatedEffects {
    fn extend(&mut self, other: Self) {
        self.dependencies.extend(other.dependencies);
        self.deferred_dependencies.extend(other.deferred_dependencies);
    }
}

pub type MacroIO<T> = IO<T, AccumulatedEffects>;

// ————————————————————————————————————————————————————————————————————————————
// MACRO RUNTIME
// ————————————————————————————————————————————————————————————————————————————

#[derive(Clone, Copy)]
pub struct MacroRuntime<'a> {
    pub project: &'a ProjectContext,
    pub macros: &'a MacroTagSet,
    pub rules: &'a TagRewriteRuleSet,
    pub input: &'a FileInput,
}

impl<'a> MacroRuntime<'a> {
    pub fn fork(&self, file_input: &'a FileInput) -> Self {
        Self {
            input: file_input,
            project: &self.project,
            macros: &self.macros,
            rules: &self.rules,
        }
    }
    pub fn source_context(&self) -> SourceContext {
        SourceContext {
            project_context: &self.project,
            file_input: &self.input,
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// POST-PROCESSING CONTEXT
// ————————————————————————————————————————————————————————————————————————————

/// Provides path resolution for source files and dependencies relative to output structure.
#[derive(Debug, Clone, Copy)]
pub struct PathResolver<'a> {
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
    pub host_context: SourceContext<'a>,
}

