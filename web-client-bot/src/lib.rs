pub mod data;
pub mod wait_framework;

use chromiumoxide::browser::{Browser, BrowserConfig};
// use chromiumoxide::cdp::browser_protocol::target::CreateTargetParams;
use chromiumoxide::Page;
use futures::StreamExt;


#[derive(Debug)]
pub struct WebClient {
    browser: Browser,
}

impl WebClient {
    pub async fn start() -> WebClient {
        let browser_config = BrowserConfig::builder().build().unwrap();
        let (browser, mut handler) = Browser::launch(browser_config).await.unwrap();
        tokio::spawn(async move {
            while let Some(payload) = handler.next().await {
                match payload {
                    Ok(()) => (),
                    Err(error) => {
                        eprintln!(
                            "{}",
                            format!("⚠️ {error}: {error:#?}")
                        );
                        break;
                    }
                }
            }
        });
        WebClient { browser }
    }
    pub async fn close(mut self) {
        let _ = self.browser.close().await.unwrap();
    }
    // pub async fn open_new_tab_at_url(&mut self, url: impl AsRef<str>) -> WebClientTab {
    //     let url = url.as_ref();
    //     let page = self.browser.new_page(url).await.unwrap();
    //     WebClientTab { page }
    // }
    // pub async fn open_new_tab(&mut self, open: impl Into<OpenRequest>) -> WebClientTab {
    //     let open = open.into();
    //     let page = self.browser.new_page(open).await.unwrap();
    //     WebClientTab { page }
    // }
}



impl WebClient {
    pub async fn open_new_tab_at_url(&mut self, url: impl AsRef<str>) -> WebClientTab {
        let mut current_url = url.as_ref().to_string();
        let max_redirects = 10;
        let mut redirects_followed = 0;

        loop {
            // Create a new page for each request
            let page = self.browser.new_page(current_url.clone()).await.unwrap();

            // Wait for the navigation event to complete
            page.wait_for_navigation().await.unwrap();

            // Get the final URL after potential redirects
            let final_url = page.url().await.unwrap_or(None);

            // If final_url is Some and different from current_url, follow redirect
            if let Some(ref new_url) = final_url {
                if new_url != &current_url {
                    redirects_followed += 1;
                    if redirects_followed >= max_redirects {
                        panic!("Too many redirects for URL: {}", url.as_ref());
                    }

                    // Close the old tab before continuing
                    page.close().await.unwrap();

                    current_url = new_url.clone();
                    continue;
                }

                // No redirect occurred, return the loaded tab
                return WebClientTab { page };
            } else {
                // Navigation failed or URL unavailable
                panic!("Failed to retrieve final URL after navigation.");
            }
        }
    }
}


// —— OPEN REQUEST ————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub struct OpenRequest {
//     url: String,
// }

// impl OpenRequest {
//     pub fn new_page_at_url(url: impl Into<String>) -> Self {
//         Self { url: url.into() }
//     }
// }

// impl Into<CreateTargetParams> for OpenRequest {
//     fn into(self) -> CreateTargetParams {
//         CreateTargetParams::builder()
//             .url(self.url)
//             .build()
//             .unwrap()
//     }
// }

// —— WEB CLIENT TAB ——————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct WebClientTab {
    page: Page,
}

impl WebClientTab {
    /// This resolves once the navigation finished and the page is loaded.
    pub async fn wait_for_navigation(&self) {
        let _ = self.page.wait_for_navigation().await.unwrap();
    }
    /// Returns the HTML content of the page
    pub async fn html_content(&self) -> String {
        self.page.content().await.unwrap()
    }
    /// Scrape all anchor links in the DOM tree.
    pub async fn scrape_all_anchor_links(&self) -> Vec<crate::data::Link> {
        crate::data::Link::scrape_all(&self.page).await.unwrap()
    }
    /// Returns the current url of the page
    pub async fn url(&self) -> Option<String> {
        self.page.url().await.unwrap()
    }
    /// Close this page.
    pub async fn close(self) {
        self.page.close().await.unwrap()
    }
    pub async fn is_text_html_document(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let js = "document.contentType";
        let value = self.evaluate(js).await?;
    
        Ok(value
            .as_str()
            .map(|s| s.eq_ignore_ascii_case("text/html"))
            .unwrap_or(false))
    }
}
