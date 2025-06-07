pub mod cli;
pub mod manifest;

fn main() {
    let cli = cli::Cli::load();
    cli.execute();
}
