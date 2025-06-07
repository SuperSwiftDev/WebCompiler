use serde::Deserialize;
use web_compiler_common::{InputRule, ProjectContext};
use std::path::{Path, PathBuf};

use web_compiler_core::pipeline::{BundleRule, Compiler, CompilerSettings, CompilerSources};

/// The full config file
#[derive(Debug, Deserialize)]
pub struct ManifestSpec {
    #[serde(default = "default_root")]
    pub root: PathBuf,
    
    #[serde(default = "default_output")]
    pub output_dir: PathBuf,
    
    #[serde(default)]
    pub template: Option<PathBuf>,

    #[serde(default)]
    pub pretty_print: Option<bool>,

    #[serde(default)]
    pub sources: Vec<SourceSpec>,

    #[serde(default)]
    pub manual: Vec<ManualRewriteSpec>,

    #[serde(default)]
    pub assets: Vec<AssetSpec>,

    #[serde(default)]
    pub bundles: Vec<BundleSpec>,
}

// #[derive(Debug, Deserialize)]
// pub struct Project {}

fn default_root() -> PathBuf {
    PathBuf::from(".")
}

fn default_output() -> PathBuf {
    PathBuf::from("output")
}

// fn default_pretty_print() -> bool {
//     true
// }

/// Glob-based rewrite rules
#[derive(Debug, Deserialize)]
pub struct SourceSpec {
    /// Glob pattern to match files, relative to project root
    pub input: String,

    /// Prefix to strip from the matched path
    #[serde(default)]
    pub strip_prefix: Option<String>,

    /// Set the root template for the given file; will overrdie the root project template if defined.
    pub template: Option<PathBuf>,

    /// Optional Wrapper between the root template and a subtemplate
    pub subtemplate: Option<PathBuf>,
}

/// Manual rewrite rules for specific files
#[derive(Debug, Deserialize)]
pub struct ManualRewriteSpec {
    /// Input file path
    pub source: PathBuf,

    /// Desired output path
    pub target: PathBuf,
}

/// Static assets to copy into output directory
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AssetSpec {
    Glob {
        /// Glob pattern to match asset files
        pattern: String,

        /// Prefix to strip when copying assets to output
        #[serde(default)]
        strip_prefix: Option<String>,
    }
}

/// Static assets to copy into output directory
#[derive(Debug, Deserialize)]
pub struct BundleSpec {
    /// Glob pattern to match asset files
    pub location: PathBuf,
}


impl ManifestSpec {
    pub fn load(path: impl AsRef<Path>) -> Result<ManifestSpec, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)?;
        let config: ManifestSpec = toml::from_str(&text)?;
        Ok(config)
    }
    pub fn execute(self, working_dir: impl AsRef<Path>, pretty_format: bool) {
        let working_dir = working_dir.as_ref();
        let working_dir = working_dir.join(&self.root);
        std::env::set_current_dir(&working_dir).unwrap();
        let bundle_rules = self.bundles
            .iter()
            .map(|bundle| {
                BundleRule {
                    location: bundle.location.clone(),
                }
            })
            .collect::<Vec<_>>();
        let input_rules = self.sources
            .iter()
            .flat_map(|rule| {
                web_compiler_common::resolve_file_path_paterns(&[rule.input.clone()])
                    .into_iter()
                    .flat_map(|x| x)
                    .map(|path| {
                        let public = rule.strip_prefix
                            .as_ref()
                            .map(|x| {
                                path.strip_prefix(x).unwrap().to_path_buf()
                            });
                        InputRule {
                            source: path,
                            public,
                            template: rule.template.clone(),
                            subtemplate: rule.subtemplate.clone(),
                        }
                    })
            })
            .collect::<Vec<_>>();
        let compiler = Compiler {
            settings: CompilerSettings {
                pretty_format_output: pretty_format,
                project_context: ProjectContext {
                    project_root: self.root,
                    output_dir: self.output_dir,
                },
            },
            sources: CompilerSources {
                template_path: self.template,
                input_rules: input_rules,
                bundle_rules: bundle_rules,
            },
        };
        compiler.execute();
    }
}

// impl ProjectManifest {
//     pub fn execute(&self, manifest_dir: impl AsRef<Path>, pretty_print: Option<bool>) {
//         let manifest_dir = manifest_dir.as_ref();
//         let working_dir = manifest_dir.join(&self.root);
//         std::env::set_current_dir(&working_dir).unwrap();
//         let bundles = self.bundles
//             .iter()
//             .map(|bundle| {
//                 crate::compile::BundleRule {
//                     location: bundle.location.clone(),
//                 }
//             })
//             .collect::<Vec<_>>();
//         let inputs = self.globs
//             .iter()
//             .flat_map(|rule| {
//                 crate::path_utils::resolve_file_path_paterns(&[rule.pattern.clone()])
//                     .into_iter()
//                     .flat_map(|x| x)
//                     .map(|path| {
//                         let target = rule.strip_prefix
//                             .as_ref()
//                             .map(|x| {
//                                 path.strip_prefix(x).unwrap().to_path_buf()
//                             });
//                         crate::compile::InputRule {
//                             source: path,
//                             target,
//                         }
//                     })
//             })
//             .collect::<Vec<_>>();
//         let compiler = Compiler {
//             project_root: working_dir,
//             input_paths: inputs,
//             template_path: self.template.clone(),
//             output_dir: self.output_dir.clone(),
//             pretty_print: self.pretty_print.unwrap_or(pretty_print.unwrap_or(true)),
//             bundles,
//         };
//         compiler.run();
//     }
// }

