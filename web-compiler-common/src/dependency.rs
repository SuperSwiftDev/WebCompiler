//! Data types for project inputs, asset dependencies, and project context.
//!
//! This module defines the core metadata required to resolve files:
//!
//! - [`InputRule`] declares top-level HTML/CSS/JS source files with optional `public` paths.
//! - [`FileDependency`] tracks additional files referenced by those inputs, such as fragments or static assets.
//! - [`ProjectContext`] defines the logical structure of the project, including root and output directories.
//!
//! These types are consumed by the [`PathResolver`](super::PathResolver), which computes resolved output paths and rewrites.

use std::path::{Path, PathBuf};

/// Describes a source file and its intended public output path.
/// 
/// If `public` is `None`, the `source` path relative to `project_root` is used.
#[derive(Debug, Clone)]
pub struct InputRule {
    /// The original source file path (absolute or project-root-relative).
    pub source: PathBuf,
    /// The desired output path, relative to the output directory.
    pub public: Option<PathBuf>,
    pub template: Option<PathBuf>,
    /// Not yet implemented.
    pub subtemplate: Option<PathBuf>,
}

impl InputRule {
    pub fn resolved_target_path(&self, project_context: &ProjectContext) -> PathBuf {
        self.public
            .as_ref()
            .map(|out_path| out_path.to_path_buf())
            .unwrap_or_else(|| {
                let path = self.source.as_path();
                let path = path
                    .strip_prefix(&project_context.project_root)
                    .unwrap_or(path);
                path.to_path_buf()
            })
    }
    pub fn output_file_path(
        &self,
        project_context: &ProjectContext
    ) -> PathBuf {
        let target_file_path = self.resolved_target_path(project_context);
        project_context.output_dir.join(target_file_path)
    }
}

/// Represents a non-transforming file dependency (e.g., images, includes, fonts).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileDependency {
    /// The file path as referenced from another file (the origin).
    pub origin: PathBuf,
    /// The target file path relative to the origin, as written in source (e.g. `"../img.png"`).
    pub target: PathBuf,
}

impl FileDependency {
    pub fn resolved_target(&self) -> PathBuf {
        let origin_dir = self.origin.parent().unwrap();
        origin_dir.join(&self.target)
    }
}

/// Global context for resolving inputs and computing output layout.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    /// Root directory of the project (used for computing relative layout).
    pub project_root: PathBuf,
    /// Final directory where output files will be emitted.
    pub output_dir: PathBuf,
}

impl ProjectContext {
    pub fn new_source_context(&self, input_rule: &InputRule) -> SourceContext {
        SourceContext {
            project_context: self.clone(),
            input_rule: input_rule.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceContext {
    pub project_context: ProjectContext,
    pub input_rule: InputRule,
}

impl SourceContext {
    pub fn source_dir(&self) -> &Path {
        self.input_rule.source.parent().unwrap()
    }
    pub fn source_file(&self) -> &Path {
        self.input_rule.source.as_path()
    }
    pub fn output_file_path(&self) -> PathBuf {
        self.input_rule.output_file_path(&self.project_context)
    }
    pub fn new_relative_source_file_dependency(&self, src: &str) -> FileDependency {
        let origin = self.source_file().to_path_buf();
        let target = PathBuf::from(src);
        FileDependency {
            origin,
            target
        }
    }
}
