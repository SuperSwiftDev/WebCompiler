// ======================= Wait Framework =======================
// ✅ Supports: DomReady, SelectorExists, NetworkIdle
// ✅ Combinators: All, Any
// ✅ Configurable polling via WaitOptions
// ✅ Trait-based and extensible
// =============================================================

use async_trait::async_trait;
use std::time::Duration;
use chromiumoxide::cdp::js_protocol::runtime::EvaluateParams;
use serde_json::Value;

use crate::WebClientTab;

// ======================= WaitOptions ==========================
#[derive(Debug, Clone)]
pub struct WaitOptions {
    pub timeout: Duration,
    pub interval: Duration,
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            interval: Duration::from_millis(250),
        }
    }
}

// ======================= WaitCondition Trait ==================
#[async_trait]
pub trait WaitCondition: Send + Sync {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>>;

    /// Optional: Cleanup after the condition has finished (default: no-op)
    async fn cleanup(&self, _tab: &WebClientTab) {}
}

// ======================= Built-In Conditions ==================

// --- DOM Ready ---
pub struct DomReady;

#[async_trait]
impl WaitCondition for DomReady {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>> {
        let expr = "document.readyState === 'complete'";
        let result = tab.evaluate(expr).await?.as_bool().unwrap_or(false);
        Ok(result)
    }
}

// --- Network Idle ---
pub struct NetworkIdle {
    pub quiet_window: Duration,
    pub total_timeout: Duration,
    pub poll_interval: Duration,
}

impl NetworkIdle {
    pub fn new(quiet_window: Duration, total_timeout: Duration, poll_interval: Duration) -> Self {
        Self {
            quiet_window,
            total_timeout,
            poll_interval,
        }
    }
}

// #[async_trait]
// impl WaitCondition for NetworkIdle {
//     async fn is_satisfied(&self, tab: &WebClientTab) -> bool {
//         let js = format!(
//             r#"
//             (function() {{
//                 if (!window._networkIdleState) {{
//                     window._networkIdleState = {{
//                         lastCount: performance.getEntriesByType("resource").length,
//                         lastChange: Date.now()
//                     }};
//                     setInterval(() => {{
//                         const count = performance.getEntriesByType("resource").length;
//                         if (count !== window._networkIdleState.lastCount) {{
//                             window._networkIdleState.lastChange = Date.now();
//                             window._networkIdleState.lastCount = count;
//                         }}
//                     }}, {});
//                 }}
//                 return Date.now() - window._networkIdleState.lastChange >= {};
//             }})()
//             "#,
//             self.poll_interval.as_millis(),
//             self.quiet_window.as_millis()
//         );

//         tab.evaluate(&js).await.as_bool().unwrap_or(false)
//     }
// }

#[async_trait]
impl WaitCondition for NetworkIdle {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>> {
        let js = format!(
            r#"
            (() => {{
                const resources = performance.getEntriesByType("resource");
                const now = performance.now();
                let latest = 0;
                for (let r of resources) {{
                    if (r.responseEnd > latest) latest = r.responseEnd;
                }}
                return now - latest >= {};
            }})()
            "#,
            self.quiet_window.as_millis()
        );

        let result = tab.evaluate(&js).await?.as_bool().unwrap_or(false);
        Ok(result)
    }
}

// pub struct FetchIdle;

// #[async_trait]
// impl WaitCondition for FetchIdle {
//     async fn is_satisfied(&self, tab: &WebClientTab) -> bool {
//         let js = r#"
//         (() => {
//             const ns = '__chromiumoxide_internal';

//             if (!window[ns]) {
//                 window[ns] = { activeRequests: 0 };

//                 const refState = window[ns];

//                 const origFetch = window.fetch;
//                 window.fetch = function(...args) {
//                     refState.activeRequests++;
//                     return origFetch.apply(this, args).finally(() => refState.activeRequests--);
//                 };

//                 const origOpen = XMLHttpRequest.prototype.open;
//                 XMLHttpRequest.prototype.open = function(...args) {
//                     this.addEventListener('loadend', () => refState.activeRequests--);
//                     refState.activeRequests++;
//                     return origOpen.apply(this, args);
//                 };
//             }

//             return window[ns].activeRequests === 0;
//         })()
//         "#;

//         tab.evaluate(js).await.as_bool().unwrap_or(false)
//     }
// }

pub struct FetchIdle;

#[async_trait]
impl WaitCondition for FetchIdle {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>> {
        let js = r#"
        (() => {
            const ns = '__chromiumoxide_internal';

            if (!window[ns]) {
                window[ns] = {
                    activeRequests: 0,
                    originalFetch: window.fetch,
                    originalXHROpen: XMLHttpRequest.prototype.open
                };

                const refState = window[ns];

                const origFetch = window.fetch;
                window.fetch = function(...args) {
                    refState.activeRequests++;
                    return origFetch.apply(this, args).finally(() => refState.activeRequests--);
                };

                const origOpen = XMLHttpRequest.prototype.open;
                XMLHttpRequest.prototype.open = function(...args) {
                    this.addEventListener('loadend', () => refState.activeRequests--);
                    refState.activeRequests++;
                    return origOpen.apply(this, args);
                };
            }

            return window[ns].activeRequests === 0;
        })()
        "#;

        let result = tab.evaluate(js).await?.as_bool().unwrap_or(false);
        Ok(result)
    }

    async fn cleanup(&self, tab: &WebClientTab) {
        let js = r#"
        (() => {
            const ns = '__chromiumoxide_internal';
            const state = window[ns];

            if (state && state.originalFetch && state.originalXHROpen) {
                window.fetch = state.originalFetch;
                XMLHttpRequest.prototype.open = state.originalXHROpen;
            }

            delete window[ns];
        })()
        "#;

        let _ = tab.evaluate(js).await;
    }
}




// ======================= Combinators ==========================

pub struct All(pub Vec<Box<dyn WaitCondition>>);

#[async_trait]
impl WaitCondition for All {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>> {
        for cond in &self.0 {
            if !cond.is_satisfied(tab).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

pub struct Any(pub Vec<Box<dyn WaitCondition>>);

#[async_trait]
impl WaitCondition for Any {
    async fn is_satisfied(&self, tab: &WebClientTab) -> Result<bool, Box<dyn std::error::Error>> {
        for cond in &self.0 {
            if cond.is_satisfied(tab).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

// ======================= Wait Runner ==========================

// pub struct WaitRunner;

// impl WaitRunner {
//     pub async fn run<C: WaitCondition>(
//         condition: &C,
//         tab: &WebClientTab,
//         options: &WaitOptions,
//     ) {
//         let start = Instant::now();

//         while start.elapsed() < options.timeout {
//             if condition.is_satisfied(tab).await {
//                 return;
//             }
//             tokio::time::sleep(options.interval).await;
//         }

//         panic!("Timeout while waiting for condition");
//     }
// }

pub struct WaitRunner;

impl WaitRunner {
    pub async fn run<C: WaitCondition>(
        condition: &C,
        tab: &WebClientTab,
        options: &WaitOptions,
    ) -> Result<(), WaitError> {
        let start = tokio::time::Instant::now();

        while start.elapsed() < options.timeout {
            let condition_is_satisfied = condition
                .is_satisfied(tab)
                .await
                .map_err(WaitError::Other)?;
            if condition_is_satisfied {
                condition.cleanup(tab).await;
                return Ok(());
            }

            tokio::time::sleep(options.interval).await;
        }

        Err(WaitError::Timeout(options.timeout))
    }
}

// use std::fmt;
// use std::time::Duration;
// use std::error::Error;

#[derive(Debug)]
pub enum WaitError {
    Timeout(Duration),
    Other(Box<dyn std::error::Error>)
    // EvaluationFailed(String),
    // UnexpectedJsResult,
}

impl std::fmt::Display for WaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaitError::Timeout(dur) => write!(f, "Timeout while waiting for condition after {:?}", dur),
            WaitError::Other(error) => write!(f, "{}", error),
            // WaitError::EvaluationFailed(msg) => write!(f, "JavaScript evaluation failed: {msg}"),
            // WaitError::UnexpectedJsResult => write!(f, "Unexpected JavaScript result during evaluation"),
        }
    }
}

impl std::error::Error for WaitError {}

// impl From<chromiumoxide::error::CdpError> for WaitError {
//     fn from(err: chromiumoxide::error::CdpError) -> Self {
//         WaitError::EvaluationFailed(err.to_string())
//     }
// }



// ======================= WebClientTab Extension ===============

impl WebClientTab {
    /// Evaluate JS in the page context
    pub async fn evaluate(&self, js_expr: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let eval = EvaluateParams::builder()
            .expression(js_expr)
            .return_by_value(true)
            .build()?;

        let result = self.page.evaluate(eval).await?;
        let result = result
            .value()
            .map(|x| x.to_owned())
            .unwrap_or(Value::Null);
        Ok(result)
    }

    /// Wait until a condition is satisfied
    pub async fn wait_until<C: WaitCondition>(
        &self,
        condition: C,
        options: WaitOptions,
    ) -> Result<(), WaitError> {
        WaitRunner::run(&condition, self, &options).await
    }
}

impl WebClientTab {
    /// General-purpose wait heuristic for arbitrary pages:
    /// - Waits for DOM ready
    /// - Waits for JS network (fetch/XHR) idle
    /// - Waits for network resources to finish
    /// - Sleeps a bit more to let rendering settle
    pub async fn wait_until_fully_settled(&self) -> Result<(), WaitError> {
        self.wait_for_navigation().await;

        let condition = All(vec![
            Box::new(DomReady),
            Box::new(FetchIdle),
            Box::new(NetworkIdle {
                quiet_window: Duration::from_millis(500),
                total_timeout: Duration::from_secs(15),
                poll_interval: Duration::from_millis(200),
            }),
        ]);

        let options = WaitOptions {
            timeout: Duration::from_secs(20),
            interval: Duration::from_millis(200),
        };

        let () = self.wait_until(condition, options).await?;

        // Optional: delay after all activity has settled
        tokio::time::sleep(Duration::from_secs(1)).await;

        Ok(())
    }
}


