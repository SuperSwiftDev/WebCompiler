pub mod manifest;
pub mod cli;

// use std::path::PathBuf;

// use web_compiler_core::pipeline::{GlobalPipelineSpec, SourcePipeline};
// use web_compiler_macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};

fn main() {
    let cli = cli::CommandLineInterface::load();
    cli.execute();
}
