use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use web_compiler_types::{CompilationMode, CompilerFeatureset, CompilerInputRule, CompilerInputs, CompilerPipeline};
use web_compiler_macro_types::project::{FileInput, ProjectContext};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectSpec {
    output: PathBuf,
    template: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TargetSpec {
    r#type: String,
    id: String,
    output: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceSpec {
    input: String,
    #[serde(default)]
    strip_prefix: Option<String>,
    /// will override the default global template.
    #[serde(default)]
    template: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSpec {
    project: ProjectSpec,
    #[serde(default)]
    targets: Vec<TargetSpec>,
    #[serde(default)]
    sources: Vec<SourceSpec>,
}

#[derive(Debug, Clone)]
pub struct Manifest {
    spec: ManifestSpec,
    file_path: PathBuf,
}

impl Manifest {
    pub fn navigate_to_working_dir(&self) {
        let directory = self.file_path.parent().expect("path should have a parent");
        let _ = std::env::set_current_dir(directory)
            .inspect_err(|_| {
                if !directory.as_os_str().is_empty() {
                    eprintln!("Failed to set current directory: {directory:?}");
                }
            });
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
    pub fn to_compiler_pipeline(&self, target: Option<&String>, featureset: CompilerFeatureset) -> CompilerPipeline {
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
            output_dir: {
                target
                    .map(|target_id| {
                        let target_spec = self.spec.targets
                            .iter()
                            .find(|x| x.id.as_str() == target_id)
                            .expect("target not found");
                        target_spec.output.clone()
                    })
                    .unwrap_or_else(|| self.spec.project.output.clone())
            },
        };
        if let Some(target_id) = target {
            let target = self.spec.targets
                .iter()
                .find(|x| x.id.as_str() == target_id)
                .expect("target not found");
            let mode = match target.r#type.to_lowercase().trim() {
                "production" => CompilationMode::Production,
                "dev" => CompilationMode::Dev,
                _ => panic!("target type not valid"),
            };
            return CompilerPipeline {
                featureset,
                inputs: CompilerInputs {
                    compilation_mode: mode,
                    sources,
                    project,
                    global_template: Some(self.spec.project.template.clone()),
                },
            }
        }
        CompilerPipeline {
            featureset,
            inputs: CompilerInputs {
                compilation_mode: CompilationMode::default(),
                sources,
                project,
                global_template: Some(self.spec.project.template.clone()),
            },
        }
    }
}


