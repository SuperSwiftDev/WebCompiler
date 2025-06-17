use std::path::PathBuf;

use clap::{Parser, Subcommand};
use web_compiler_core::system::web_publishing_compiler_featureset;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Build(BuildCli),
}

#[derive(Parser, Debug)]
struct BuildCli {
    /// Path to the manifest file.
    // #[arg(long)]
    pub manifest: PathBuf,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub fn execute(self) {
        match self.command {
            SubCommand::Build(build) => build.execute(),
        }
    }
}

impl BuildCli {
    pub fn execute(self) {
        let manifest = crate::manifest::Manifest::load(self.manifest).unwrap();
        manifest.navigate_to_working_dir();
        let compiler_pipeline = manifest.to_compiler_pipeline(web_publishing_compiler_featureset());
        web_compiler_core::system::execute_compiler_pipeline(compiler_pipeline);
    }
}


