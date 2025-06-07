//! Output path resolver for source files and asset dependencies.
//!
//! This module implements [`PathResolver`], which provides on-demand path resolution:
//!
//! - Maps [`InputRule`] and [`FileDependency`] sources to their output locations
//! - Computes relative links between output files (for HTML `href`, `src`, etc.)
//! - Supports rewriting of [`EncodedVirtualPath`] references into usable relative paths
//!
//! ## Why This Design?
//!
//! In HTML and CSS, relative file references are **not associative** — rewriting paths
//! inline during tree traversal (e.g., `<include src="...">`) can produce invalid URLs.
//!
//! To fix this, we:
//!
//! 1. Encode paths at parse time using [`EncodedVirtualPath`] with origin and relative path
//! 2. Delay rewriting until output layout is finalized
//! 3. Use `PathResolver` to compute relative output-safe paths
//!
//! This ensures correctness regardless of file nesting or inclusion depth.

use std::path::{Path, PathBuf};

use path_clean::clean;
use pathdiff::diff_paths;

use crate::SourceContext;

use super::{EncodedVirtualPath, FileDependency, InputRule, ProjectContext};

/// Provides path resolution for source files and dependencies relative to output structure.
///
/// This struct maps input files and asset dependencies to their corresponding
/// output locations. It also handles rewriting of [`EncodedVirtualPath`] references
/// based on the final output layout.
///
/// Use this to:
/// - Rewrite `@virtual:` references after compilation layout is known
/// - Determine where to write or link assets
/// - Check if a given path is recognized as a known input or dependency
#[derive(Debug, Clone)]
pub struct PathResolver {
    /// List of source input files (e.g. HTML, JS, CSS).
    pub inputs: Vec<InputRule>,
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
    pub dependencies: Vec<FileDependency>,
    /// Project-wide layout context (project root + output dir + host source info).
    pub host_context: SourceContext,
}

impl PathResolver {
    pub fn project_root(&self) -> &Path {
        &self.host_context.project_context.project_root
    }
    pub fn output_dir(&self) -> &Path {
        &self.host_context.project_context.output_dir
    }
    pub fn project_context(&self) -> &ProjectContext {
        &self.host_context.project_context
    }
    pub fn host_origin_file_path(&self) -> &Path {
        self.host_context.source_file()
    }
    pub fn host_output_file_path(&self) -> PathBuf {
        self.host_context.output_file_path()
    }

    /// Resolves the given `source` file path to its final output location.
    ///
    /// - If it's a known [`InputRule`], returns its `public` path or its path relative to `project_root`.
    /// - If it's a known [`FileDependency`], resolves based on origin path.
    ///
    /// Returns `None` if the path isn't known.
    ///
    /// ## Examples
    /// ```rust
    /// let output = resolver.resolve_output_path(Path::new("src/pages/index.html"));
    /// ```
    pub fn resolve_output_path(&self, source: impl AsRef<Path>) -> Option<PathBuf> {
        let cleaned_source = clean(source);

        // ── 1. Check InputRules ──
        if let Some(input) = self.inputs.iter().find(|r| clean(&r.source) == cleaned_source) {
            let relative_output_path = input.public.clone().unwrap_or_else(|| {
                input
                    .source
                    .strip_prefix(&self.project_root())
                    .unwrap_or(&input.source)
                    .to_path_buf()
            });
            return Some(clean(self.output_dir().join(relative_output_path)));
        }


        // ── 2. Check FileDependencies ──
        if let Some(dep) = self.dependencies.iter().find(|d| clean(&d.origin) == cleaned_source) {
            let relative_output_path = dep
                .origin
                .strip_prefix(&self.project_root())
                .unwrap_or(&dep.origin)
                .to_path_buf();
            return Some(clean(self.output_dir().join(relative_output_path)));
        }

        None
    }

    /// Resolves a virtual encoded path into a browser-safe relative URL path.
    ///
    /// The `EncodedVirtualPath` must specify:
    /// - The origin file (relative to `project_root`)
    /// - The original relative reference (as written in HTML/CSS/JS)
    ///
    /// This computes the final output-relative link between the two output files.
    ///
    /// Returns `None` if either side is not known or cannot be resolved.
    ///
    /// ## Examples
    /// ```rust
    /// let href = resolver.resolve_virtual_path(&EncodedVirtualPath {
    ///     origin: "pages/index.html".into(),
    ///     rel: "./img/logo.png".into(),
    /// });
    /// assert_eq!(href, Some("img/logo.png".into()));
    /// ```
    pub fn resolve_virtual_path(&self, encoded: &EncodedVirtualPath) -> Option<String> {
        let origin_source = clean(self.project_root().join(&encoded.origin));
        let origin_output = self.resolve_output_path(&origin_source)?;

        let target_source = origin_source.parent()?.join(&encoded.rel);
        let target_output = self.resolve_output_path(&clean(target_source))?;

        let relative = diff_paths(&target_output, origin_output.parent()?)?;
        
        Some(super::to_url_path(&relative))
    }

    /// Resolves an asset source file path to its output location (if known).
    ///
    /// This wraps [`resolve_output_path`] and is intended for assets like images,
    /// fonts, etc., which may be symlinked or copied into the output directory.
    ///
    /// ## Examples
    /// ```rust
    /// let out = resolver.resolve_asset_output_path(Path::new("assets/logo.png"));
    /// ```
    pub fn resolve_asset_output_path(&self, asset_source: &Path) -> Option<PathBuf> {
        self.resolve_output_path(asset_source)
    }

    /// Returns true if the path is a known asset dependency.
    ///
    /// This means it was registered in [`FileDependency::origin`] and may need to
    /// be emitted to the output directory (symlinked, copied, etc.).
    ///
    /// ## Examples
    /// ```rust
    /// assert!(resolver.is_known_dependency(Path::new("assets/icon.svg")));
    /// ```
    pub fn is_known_dependency(&self, source: &Path) -> bool {
        let cleaned_source = clean(source);
        self.dependencies.iter().any(|d| clean(&d.origin) == cleaned_source)
    }

    /// Returns true if the path is a known input (HTML, CSS, JS entrypoint).
    ///
    /// This checks for a match in [`InputRule::source`] and is used to distinguish
    /// primary files from supporting assets or fragments.
    ///
    /// ## Examples
    /// ```rust
    /// assert!(resolver.is_known_input(Path::new("src/index.html")));
    /// ```
    pub fn is_known_input(&self, source: &Path) -> bool {
        let cleaned_source = clean(source);
        self.inputs.iter().any(|r| clean(&r.source) == cleaned_source)
    }
}
