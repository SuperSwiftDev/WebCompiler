pub mod cli;

#[tokio::main]
async fn main() {
    let cli = cli::CommandLineInterface::load();
    cli.execute().await
}
