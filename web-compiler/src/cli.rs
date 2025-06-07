use std::path::PathBuf;
use clap::{Parser, Subcommand};
use web_compiler_common::{InputRule, ProjectContext};
use web_compiler_core::pipeline::{Compiler, CompilerSettings, CompilerSources};
// use pretty_tree::PrettyTreePrinter;

// use crate::compile::Compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Compile sources manually via CLI interface.
    IO(IoCli),
    /// Run the compiler and execute the plan as defined by the provided manifest (TOML) file.
    Run(RunCli),
}

#[derive(Parser, Debug)]
pub struct IoCli {
    /// Wrap all contents in the given template file.
    #[arg(long)]
    template: Option<PathBuf>,
    /// Array of file paths or unix style glob patterns.
    /// 
    /// The system will try to automatically resolve whether each respective input is a glob or a file path. To disable glob mode checking and treat each input as a file path see the `no_globs` flag.
    #[arg(long, num_args = 1..)]
    input: Vec<String>,
    /// The output directory.
    #[arg(long)]
    output: PathBuf,
    /// The project root.
    #[arg(long)]
    root: PathBuf,
    /// Pretty-print HTML(5) files (more pretty); default value is true.
    #[arg(long)]
    pretty_format: Option<bool>,
}

#[derive(Parser, Debug)]
pub struct RunCli {
    /// Path to the manifest file.
    #[arg(long)]
    pub manifest: PathBuf,
    /// Pretty-print HTML(5) files (more pretty); default value is true.
    #[arg(long)]
    pretty_format: Option<bool>,
}

impl Cli {
    pub fn load() -> Self {
        Cli::parse()
    }
    pub fn execute(self) {
        match self.command {
            Command::IO(compile_cli) => compile_cli.execute(),
            Command::Run(build_cli) => build_cli.execute(),
        }
    }
}

impl IoCli {
    pub fn execute(self) {
        let input_rules = web_compiler_common::resolve_file_path_paterns(&self.input)
            .unwrap()
            .into_iter()
            .map(|path| {
                InputRule {
                    source: path,
                    public: None,
                    template: None,
                    subtemplate: None,
                }
            })
            .collect::<Vec<_>>();
        let compiler = Compiler {
            settings: CompilerSettings {
                pretty_format_output: self.pretty_format.unwrap_or(true),
                project_context: ProjectContext {
                    project_root: self.root,
                    output_dir: self.output,
                },
            },
            sources: CompilerSources {
                template_path: self.template,
                input_rules: input_rules,
                bundle_rules: Vec::default(),
            },
        };
        compiler.execute();
    }
}

impl RunCli {
    pub fn execute(self) {
        let manifest_dir = self.manifest.parent().unwrap();
        let manifest_spec = crate::manifest::ManifestSpec::load(&self.manifest);
        let manifest_spec = match manifest_spec {
            Ok(x) => x,
            Err(error) => {
                web_compiler_core::pipeline::log_error(
                    error.as_ref(),
                    Some(self.manifest.as_path()),
                    None
                );
                std::process::exit(1)
            }
        };
        manifest_spec.execute(manifest_dir, self.pretty_format.unwrap_or(true));
    }
}

