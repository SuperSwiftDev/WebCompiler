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
    /// Path to the prompt file.
    // #[arg(long)]
    pub file: PathBuf,
    /// The name of the prompt.
    #[arg(short, long)]
    pub name: String,
    /// Path to the output file.
    #[arg(short, long)]
    pub output: PathBuf,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub async fn execute(self) {
        match self.command {
            SubCommand::Run(run) => run.execute().await,
        }
    }
}

impl RunCli {
    pub async fn execute(self) {
        let document = web_compiler_core::xml_ai::XmlAiDocument::load(self.file.as_path());
        let result = document.run_prompt(self.name).await;
        match self.output.extension().unwrap().to_str().unwrap() {
            "json" => {
                let debug_output = serde_json::to_string_pretty(&result).unwrap();
                std::fs::write(&self.output, debug_output).unwrap();
            },
            "toml" => {
                let debug_output = toml::to_string_pretty(&result).unwrap();
                std::fs::write(&self.output, debug_output).unwrap();
            },
            _ => panic!("NOT A VALID OUTPUT FILE"),
        }
    }
}

// #[derive(Debug, Clone)]
// enum OutputFormat {
//     Json,
//     Toml,
// }

// impl OutputFormat {
//     pub fn from_file_ext(ext: impl AsRef<Path>) -> Option<Self> {
//         match ext.as_ref().extension()?.to_str()? {
//             "json" => Some(Self::Json),
//             "toml" => Some(Self::Toml),
//             _ => None,
//         }
//     }
//     pub fn write_to()
// }


