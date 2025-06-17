use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use web_compiler_core::system::{CompilerFeatureset, CompilerInputRule, CompilerInputs, CompilerPipeline};
use web_compiler_macro_types::project::{FileInput, ProjectContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectSpec {
    output: PathBuf,
    template: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceSpec {
    input: String,
    #[serde(default)]
    strip_prefix: Option<String>,
    #[serde(default)]
    template: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSpec {
    project: ProjectSpec,
    sources: Vec<SourceSpec>,
}

#[derive(Debug, Clone)]
pub struct Manifest {
    spec: ManifestSpec,
    file_path: PathBuf,
}

impl Manifest {
    pub fn navigate_to_working_dir(&self) {
        std::env::set_current_dir(self.file_path.parent().unwrap()).unwrap();
    }
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let text = std::fs::read_to_string(file_path)?;
        let spec = toml::from_str::<ManifestSpec>(&text)?;
        Ok(Manifest {
            spec,
            file_path: file_path.to_path_buf(),
        })
    }
    pub fn to_compiler_pipeline(&self, featureset: CompilerFeatureset) -> CompilerPipeline {
        let sources = self.spec.sources
            .iter()
            .flat_map(|source| {
                web_compiler_core::common::path_utils::resolve_file_path_patern(&source.input)
                    .unwrap()
                    .into_iter()
                    .map(|src_path| {
                        let public_path = source.strip_prefix
                            .as_ref()
                            .map(|prefix| {
                                src_path.strip_prefix(prefix).ok().unwrap().to_path_buf()
                            });
                        CompilerInputRule {
                            source: FileInput {
                                source: src_path,
                                public: public_path,
                            },
                            local_template: source.template.clone(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let project = ProjectContext {
            project_root: self.file_path.parent().unwrap().to_path_buf(),
            output_dir: self.spec.project.output.clone(),
        };
        CompilerPipeline {
            featureset,
            inputs: CompilerInputs {
                sources,
                project,
                global_template: Some(self.spec.project.template.clone()),
            },
        }
    }
}


