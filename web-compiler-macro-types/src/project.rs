use std::{collections::HashSet, path::{Path, PathBuf}};

use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

// ————————————————————————————————————————————————————————————————————————————
// PROJECT CONTEXT
// ————————————————————————————————————————————————————————————————————————————

/// Global context for resolving inputs and computing output layout.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    /// Root directory of the project (used for computing relative layout).
    pub project_root: PathBuf,
    /// Final directory where output files will be emitted.
    pub output_dir: PathBuf,
}

// ————————————————————————————————————————————————————————————————————————————
// FILE INPUTS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct FileInput {
    pub source: PathBuf,
    pub public: Option<PathBuf>,
}

impl FileInput {
    pub fn resolved_public_path(&self, project_context: &ProjectContext) -> PathBuf {
        self.public
            .as_ref()
            .map(|out_path| {
                let out_path = out_path
                    .strip_prefix(&project_context.project_root)
                    .unwrap_or(out_path);
                out_path.to_path_buf()
            })
            .unwrap_or_else(|| {
                let path = self.source.as_path();
                let path = path
                    .strip_prefix(&project_context.project_root)
                    .unwrap_or(path);
                path.to_path_buf()
            })
    }
    pub fn source_dir(&self) -> &Path {
        self.source.parent().unwrap()
    }
    pub fn source_file(&self) -> &Path {
        self.source.as_path()
    }
    pub fn to_output_file_path(
        &self,
        project_context: &ProjectContext
    ) -> PathBuf {
        let target_file_path = self.resolved_public_path(project_context);
        project_context.output_dir.join(target_file_path)
    }
    pub fn with_dependency_relation(&self, src: impl AsRef<str>) -> DependencyRelation {
        let from = self.source_file().to_path_buf();
        let from = from.to_str().unwrap().to_string();
        DependencyRelation {
            from,
            to: src.as_ref().to_string(),
        }
    }
    pub fn load_source_file(&self) -> Result<String, std::io::Error> {
        let content = std::fs::read_to_string(self.source_file())?;
        Ok(content)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// FILE DEPENDENCIES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileDependency {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl FileDependency {
    pub fn resolved_target_path(&self) -> PathBuf {
        self.from.parent().unwrap().join(&self.to)
    }
}

/// Arbitrary URLs / local relative paths / anything that may appear in an HTML context.
/// 
/// This supports encoding/decoding to/from strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyRelation {
    pub from: String,
    pub to: String,
}

impl DependencyRelation {
    pub const ENCODED_PREFIX: &'static str = "@virtual://";
    pub fn encode(&self) -> String {
        format!(
            "{}{}|{}",
            Self::ENCODED_PREFIX,
            utf8_percent_encode(&self.from, &NOT_VIRTUAL_PATH_SAFE),
            utf8_percent_encode(&self.to, &NOT_VIRTUAL_PATH_SAFE)
        )
    }
    pub fn decode(input: &str) -> Option<Self> {
        input.strip_prefix(Self::ENCODED_PREFIX).and_then(|rest| {
            let (leading, trailing) = rest.split_once("|")?;
            let from = percent_decode_str(leading).decode_utf8().ok()?.to_string();
            let to = percent_decode_str(trailing).decode_utf8().ok()?.to_string();
            Some(Self { from, to })
        })
    }
    pub fn is_virtual(input: &str) -> bool {
        input.starts_with(Self::ENCODED_PREFIX)
    }
    pub fn as_file_dependency(&self) -> FileDependency {
        FileDependency {
            from: PathBuf::from(&self.from),
            to: PathBuf::from(&self.to),
        }
    }
    pub fn is_external_target(&self) -> bool {
        crate::path_utils::is_external_url(&self.to)
    }
}

const NOT_VIRTUAL_PATH_SAFE: AsciiSet = NON_ALPHANUMERIC
    .remove(b'.')
    .remove(b'-')
    .remove(b'/')
    .remove(b'+')
    .remove(b'_');

// ————————————————————————————————————————————————————————————————————————————
// RESOLVED DEPENDENCIES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct ResolvedDependencies {
    pub dependencies: HashSet<ResolvedDependency>,
    pub emitted_files: HashSet<PathBuf>,
}

impl ResolvedDependencies {
    pub fn extend(&mut self, other: Self) {
        self.dependencies.extend(other.dependencies);
        self.emitted_files.extend(other.emitted_files);
    }
    pub fn include_dependency(&mut self, dependency: ResolvedDependency) {
        self.dependencies.insert(dependency);
    }
    pub fn include_emitted_file(&mut self, resolved_target: impl Into<PathBuf>) {
        let _ = self.emitted_files.insert(resolved_target.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedDependency {
    pub finalized: FileDependency,
    pub original: DependencyRelation,
}

impl ResolvedDependency {
    pub fn cleaned(self) -> Self {
        Self {
            finalized: FileDependency {
                from: path_clean::clean(self.finalized.from),
                to: path_clean::clean(self.finalized.to),
            },
            original: self.original,
        }
    }
}

