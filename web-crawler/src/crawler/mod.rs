pub mod db;
pub mod common;
pub mod engine;

pub trait CrawlerIngestor {
    fn for_html(source: impl Into<String>) -> String;
}
