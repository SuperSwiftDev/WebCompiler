use std::path::PathBuf;

use macro_types::project::ProjectContext;

#[derive(Debug, Clone)]
pub struct CompilerSettings {
    pub pretty_format_output: bool,
    pub project_context: ProjectContext,
}

#[derive(Debug, Clone)]
pub struct CompilerSources {
    pub global_template_path: Option<PathBuf>,
    // pub global_input_rules: Vec<InputRule>,
    // pub global_bundle_rules: Vec<BundleRule>,
}

// pub struct 

// #[derive(Debug, Clone)]
// pub struct Compiler {
//     pub settings: CompilerSettings,
//     pub sources: CompilerSources,
// }
