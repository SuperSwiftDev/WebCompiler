use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Run(RunCli),
}

#[derive(Parser, Debug)]
struct RunCli {
    /// Path to the manifest file.
    // #[arg(long)]
    pub manifest: PathBuf,
    
    /// Name of the project ro run.
    #[arg(short, long)]
    pub project: String,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub async fn execute(self) {
        match self.command {
            SubCommand::Run(build) => build.execute().await,
        }
    }
}

impl RunCli {
    pub async fn execute(self) {
        let manifest = crate::manifest::Manifest::load(self.manifest).unwrap();
        let project = manifest.get_project(&self.project).expect("project not found in manifest file");
        crate::crawler::engine::process_project_spec(project).await;
    }
}


