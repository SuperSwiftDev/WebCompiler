pub mod data;
pub mod wait_framework;

use std::i64;

use chromiumoxide::browser::{Browser, BrowserConfig};
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



// impl WebClient {
//     pub async fn open_new_tab_at_url(&mut self, url: impl AsRef<str>) -> WebClientTab {
//         let original_url = url.as_ref().to_string();
//         let mut current_url = url.as_ref().to_string();
//         let max_redirects = 10;
//         let mut redirects_followed = 0;

//         loop {
//             // Create a new page for each request
//             let page = self.browser.new_page(current_url.clone()).await.unwrap();

//             // Wait for the navigation event to complete
//             page.wait_for_navigation().await.unwrap();

//             // Get the final URL after potential redirects
//             let final_url = page.url().await.unwrap_or(None);

//             // If final_url is Some and different from current_url, follow redirect
//             if let Some(ref new_url) = final_url {
//                 if new_url != &current_url {
//                     redirects_followed += 1;
//                     if redirects_followed >= max_redirects {
//                         panic!("Too many redirects for URL: {}", url.as_ref());
//                     }

//                     // Close the old tab before continuing
//                     page.close().await.unwrap();

//                     current_url = new_url.clone();
//                     continue;
//                 }

//                 if redirects_followed != 0 {
//                     eprintln!("ⓘ Redirected {original_url:?} => {current_url:?}");
//                 }

//                 // No redirect occurred, return the loaded tab
//                 return WebClientTab { page };
//             } else {
//                 // Navigation failed or URL unavailable
//                 panic!("Failed to retrieve final URL after navigation.");
//             }
//         }
//     }
// }


// impl WebClient {
//     pub async fn open_new_tab_at_url_follow_redirects(&mut self, url: impl AsRef<str>) -> WebClientTab {
//         unimplemented!("TODO")
//     }
// }



impl WebClient {
    pub async fn open_new_tab_at_url(&mut self, url: impl AsRef<str>) -> WebClientTab {
        let requested_url = url.as_ref().to_string();

        let page = self.browser.new_page(requested_url.clone()).await.unwrap();
        page.wait_for_navigation().await.unwrap();

        let actual_url = page.evaluate("window.location.href").await.unwrap();
        let actual_url = actual_url.value().unwrap().as_str();

        if let Some(final_url) = actual_url {
            if final_url != requested_url {
                println!("ⓘ Redirected: {} => {}", requested_url, final_url);
            }
        }

        WebClientTab { page, status_code: None }
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
    status_code: Option<i64>,
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
    pub fn status_code(&self) -> Option<i64> {
        self.status_code.clone()
    }
}


impl WebClient {
    pub async fn open_new_tab_at_url_with_network_tracking(
        &mut self,
        url: impl AsRef<str>,
    ) -> WebClientTab {
        use chromiumoxide::cdp::browser_protocol::network::{EnableParams, EventResponseReceived, ResourceType};
        use futures::StreamExt;

        let requested_url = url.as_ref().to_string();

        // Create a new blank page
        let page = self.browser.new_page("about:blank").await.unwrap();

        // Get the main frame ID (used to identify top-level responses)
        let main_frame_id = page.mainframe().await.unwrap().unwrap();

        // Enable network tracking
        page.execute(EnableParams::default()).await.unwrap();

        // Start listening to response events BEFORE navigation
        let mut responses = page.event_listener::<EventResponseReceived>().await.unwrap();

        // Start navigation, and allow it to fail without panic
        let nav_result = page.goto(&requested_url).await;
        if let Err(err) = nav_result {
            eprintln!("⚠️ Navigation to {requested_url:?} failed: {err} — continuing anyway.");
        }

        // Try `wait_for_navigation`, but fallback if needed
        if let Err(err) = page.wait_for_navigation().await {
            eprintln!("⚠️ wait_for_navigation failed: {err} — falling back to JS polling.");
            loop {
                let ready = page.evaluate("document.readyState").await.unwrap();
                let state = ready.value().unwrap().as_str().unwrap_or("");
                if state == "complete" || state == "interactive" {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }

        // Extract the HTTP status code for the main document
        let mut status_code: Option<i64> = None;
        let deadline = tokio::time::sleep(std::time::Duration::from_secs(5));
        tokio::pin!(deadline);

        loop {
            tokio::select! {
                Some(event) = responses.next() => {
                    if event.r#type != ResourceType::Document {
                        continue;
                    }

                    let frame_match = event.frame_id.as_ref() == Some(&main_frame_id);
                    let url_match = event.response.url == requested_url
                        || event.response.url.starts_with(&requested_url);

                    if frame_match || url_match {
                        status_code = Some(event.response.status);
                        break;
                    }
                }
                _ = &mut deadline => {
                    eprintln!("⚠️ Timed out waiting for document response from {:?}", requested_url);
                    break;
                }
            }
        }

        // Confirm where we landed
        let actual_url = page.evaluate("window.location.href").await.unwrap();
        let actual_url = actual_url.value().unwrap().as_str().unwrap_or("").to_string();

        if actual_url != requested_url {
            println!("ⓘ Redirected: {} => {}", requested_url, actual_url);
        }

        WebClientTab {
            page,
            status_code,
        }
    }
}

