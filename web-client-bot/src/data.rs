use serde::{Serialize, Deserialize};

pub static SCRAPE_ANCHORS: &'static str = include_str!("../snippets/scrape_anchors.js");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Link {
    pub href: String,
    pub text: String,
}

impl Link {
    pub fn parse_list(source: impl AsRef<str>) -> Result<Vec<Link>, serde_json::Error> {
        serde_json::from_str::<Vec<Link>>(source.as_ref())
    }
    pub async fn scrape_all(page: &chromiumoxide::Page) -> Result<Vec<Link>, serde_json::Error> {
        let result = page
            .evaluate(SCRAPE_ANCHORS)
            .await
            .unwrap();
        result.into_value::<Vec<Link>>()
    }
}

