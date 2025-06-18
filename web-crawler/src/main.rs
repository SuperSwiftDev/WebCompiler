extern crate web_compiler_xml_ast as xml_ast;

pub mod crawler;
pub mod processor;
pub mod manifest;
pub mod cli;

#[tokio::main]
async fn main() {
    // crawl_brick_canvas().await
    // crawl_sumo_fiber().await
    // let manifest = manifest::Manifest::load("web-crawler.toml").unwrap();
    // eprintln!("{manifest:#?}");
    let cli = cli::CommandLineInterface::load();
    cli.execute().await
}

// async fn crawl_brick_canvas() {
//     let input_url = "https://brickcanvas.com/";
//     let output_dir = ".web-crawler/dump";
//     let filter_settings = FilterSettings::default()
//         .with_whitelisted_domain("brickcanvas.com")
//         .with_whitelisted_domain("www.brickcanvas.com")
//         .with_blacklisted_protocol("tel");
//     crawler::engine::crawl_site(input_url, output_dir, &filter_settings).await;
// }

// async fn crawl_sumo_fiber() {
//     let input_url = "https://sumofiber.com";
//     let output_dir = ".web-crawler/dump-dumofiber";
//     let filter_settings = FilterSettings::default()
//         .with_whitelisted_domain("sumofiber.com")
//         .with_whitelisted_domain("www.sumofiber.com")
//         .with_blacklisted_protocol("tel");
//     crawler::engine::crawl_site(input_url, output_dir, &filter_settings).await;
// }

