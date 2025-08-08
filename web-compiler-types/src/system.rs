//! Types for defining the overall compiler.

use std::path::{Path, PathBuf};

use macro_types::lexical_env::{Featureset, SourceHostRef, SourceHost};
use macro_types::macro_tag::MacroTagSet;
use macro_types::project::{FileInput, ProjectContext};
use macro_types::tag_rewrite_rule::TagRewriteRuleSet;

pub struct CompilerInputRule {
    pub source: FileInput,
    /// Will override the global template.
    pub local_template: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum CompilationMode {
    Dev, Production
}

impl Default for CompilationMode {
    fn default() -> Self {
        CompilationMode::Dev
    }
}

impl CompilationMode {
    pub fn is_dev(&self) -> bool {
        match self {
            Self::Dev => true,
            _ => false,
        }
    }
    pub fn is_production(&self) -> bool {
        match self {
            Self::Production => true,
            _ => false,
        }
    }
}

pub struct CompilerInputs {
    pub compilation_mode: CompilationMode,
    pub global_template: Option<PathBuf>,
    pub sources: Vec<CompilerInputRule>,
    pub project: ProjectContext,
}

#[derive(Clone)]
pub struct CompilerFeatureset {
    pub macros: MacroTagSet<CompilerRuntime>,
    pub rules: TagRewriteRuleSet<CompilerRuntime>,
}

impl Featureset for CompilerFeatureset {
    type Runtime = CompilerRuntime;
    fn macros(&self) -> &MacroTagSet<CompilerRuntime> {
        &self.macros
    }
    fn rules(&self) -> &TagRewriteRuleSet<CompilerRuntime> {
        &self.rules
    }
}

#[derive(Clone)]
pub struct CompilerRuntime {
    pub featureset: CompilerFeatureset,
    pub project: ProjectContext,
    pub source_file: FileInput,
}

impl CompilerRuntime {
    pub fn source_context(&self) -> SourceHostRef {
        SourceHostRef {
            project_context: &self.project,
            file_input: &self.source_file,
        }
    }
    pub fn with_source_file_path<T>(&self, apply: impl FnOnce(&Path) -> T) -> T {
        apply(self.source_file.source_file())
    }
}

impl Featureset for CompilerRuntime {
    type Runtime = Self;
    fn macros(&self) -> &MacroTagSet<Self> {
        &self.featureset.macros
    }
    fn rules(&self) -> &TagRewriteRuleSet<Self> {
        &self.featureset.rules
    }
}

impl SourceHost for CompilerRuntime {
    fn project(&self) -> &ProjectContext {
        &self.project
    }
    fn source_file(&self) -> &FileInput {
        &self.source_file
    }
    fn fork(&self, file_input: &FileInput) -> Self {
        Self {
            featureset: self.featureset.clone(),
            source_file: file_input.clone(),
            project: self.project.clone(),
        }
    }
}

pub struct CompilerPipeline {
    pub featureset: CompilerFeatureset,
    pub inputs: CompilerInputs,
}
