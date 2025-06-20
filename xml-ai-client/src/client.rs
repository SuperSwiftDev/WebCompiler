use std::{cell::RefCell, path::Path};
use colored::Colorize;
use futures::{StreamExt, TryFutureExt};

use super::response;

use crate::log::Logger;

thread_local! {
    static RUNTIME: RefCell<tokio::runtime::Runtime> = RefCell::new(tokio::runtime::Runtime::new().unwrap());
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
/// This should be called ‘RequestBuilder’ but it’s already taken.
#[derive(Default)]
pub struct ClientBuilder {
    pub api_url: Option<URL>,
    pub api_key: Option<String>,
    pub request_body: Option<super::request::RequestBuilder>,
    pub timeout: Option<std::time::Duration>,
    pub logger: Option<Box<dyn Logger>>,
}

impl ClientBuilder {
    pub fn with_api_url(mut self, api_url: URL) -> Self {
        self.api_url = Some(api_url);
        self
    }
    pub fn with_api_key(mut self, api_key: impl AsRef<str>) -> Self {
        self.api_key = Some(api_key.as_ref().to_string());
        self
    }
    pub fn with_api_key_path(mut self, path: impl AsRef<Path>) -> Self {
        let contents = std::fs::read_to_string(path).expect("api key file path");
        self.api_key = Some(contents);
        self
    }
    pub fn with_request_body(mut self, request_body: super::request::RequestBuilder) -> Self {
        self.request_body = Some(request_body);
        self
    }
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn with_logger(mut self, logger: impl Logger + 'static) -> Self {
        let logger: Box<dyn Logger> = Box::new(logger);
        self.logger = Some(logger);
        self
    }
    fn build(self) -> Option<IClient> {
        let api_url = self.api_url?;
        let api_key = self.api_key?;
        let request_body = self.request_body?.build()?;
        let timeout = self.timeout;
        let logger: Option<Box<dyn Logger>> = self.logger;
        let client = IClient { api_url, api_key, request_body, timeout, logger };
        Some(client)
    }
    pub fn build_batch_api_call(self) -> Option<BatchClient> {
        Some(BatchClient { client: self.build()? })
    }
    pub fn build_streaming_api_call(self) -> Option<StreamingClient> {
        Some(StreamingClient { client: self.build()? })
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
/// This should be called ‘Request but it’s already taken.
struct IClient {
    pub api_url: URL,
    pub api_key: String,
    pub request_body: super::request::Request,
    pub timeout: Option<std::time::Duration>,
    pub logger: Option<Box<dyn Logger>>,
}

#[derive(Debug, Clone)]
pub enum InvalidConfiguration {
    StreamFlag { should_be: bool, given: bool },
}

impl std::fmt::Display for InvalidConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidConfiguration::StreamFlag { should_be, given } => {
                let should_be = *should_be;
                let given = *given;
                let msg = format!(
                    "Error: Invalid Configuration! Stream flag should be '{should_be}' given '{given}'."
                );
                let msg = msg.red();
                write!(f, "{msg}")
            }
        }
    }
}

impl std::error::Error for InvalidConfiguration {}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct BatchClient {
    client: IClient
}

impl BatchClient {
    /// This calls the streaming client internally.
    pub fn execute_blocking(self) -> Result<response::batch::Response, Box<dyn std::error::Error>> {
        RUNTIME.with_borrow(|runtime| {
            runtime.block_on(self.execute_async())
        })
    }
    /// This calls the streaming client internally.
    pub async fn execute_async(self) -> Result<response::batch::Response, Box<dyn std::error::Error>> {
        let api_url = self.client.api_url.0;
        let api_key = self.client.api_key.as_str();
        let client = {
            if let Some(timeout) = self.client.timeout.as_ref() {
                reqwest::ClientBuilder::new()
                    .timeout(timeout.clone())
                    .build()
                    .unwrap()
            } else {
                reqwest::ClientBuilder::new().build().unwrap()
            }
        };
        let stream_flag = self.client.request_body.stream.unwrap_or(false);
        if stream_flag == true {
            return Err(Box::new(InvalidConfiguration::StreamFlag { should_be: false, given: true }));
        }
        let json_data = serde_json::to_string(&self.client.request_body).unwrap();
        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            // .json(&self.client.request_body)
            .body(json_data)
            .send()
            .await?;
        if let Some(error) = ApiError::from_code(response.status().as_u16()) {
            return Err(Box::new(error))
        }
        let result = response.text().await?;
        let result = serde_json::from_str::<response::batch::Response>(&result)?;
        // let result = response.json::<response::batch::Response>().await?;
        Ok(result)
    }
}


//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
pub struct StreamingClient {
    client: IClient
}

// #[derive(Debug, Clone)]
// pub struct ServerSentEvent {
//     pub message: String
// }


impl StreamingClient {
    pub fn execute_blocking(self) -> Result<ResponseChunkCollection, Box<dyn std::error::Error>> {
        RUNTIME.with_borrow(|runtime| {
            runtime.block_on(self.execute_async())
        })
    }
    pub async fn execute_async(self) -> Result<ResponseChunkCollection, Box<dyn std::error::Error>> {
        let api_url = self.client.api_url.0;
        let api_key = self.client.api_key.as_str();
        let client = {
            if let Some(timeout) = self.client.timeout.as_ref() {
                reqwest::ClientBuilder::new()
                    .timeout(timeout.clone())
                    .build()
                    .unwrap()
            } else {
                reqwest::Client::new()
            }
        };
        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&self.client.request_body)
            .send()
            .map_err(Box::new)
            .map_err(|x| vec![x])
            .await
            .unwrap();
        if let Some(error) = ApiError::from_code(response.status().as_u16()) {
            return Err(Box::new(error))
        }
        let stream = self.client.request_body.stream.unwrap_or(false);
        if !stream {
            return Err(Box::new(InvalidConfiguration::StreamFlag { should_be: true, given: false }));
        }
        let mut outputs = Vec::new();
        if let Some(error) = ApiError::from_code(response.status().as_u16()) {
            return Err(Box::new(error));
        }
        let response = response.bytes_stream();
        tokio::pin!(response);
        let mut logger = self.client.logger;
        while let Some(Ok(data)) = response.next().await {
            let data = String::from_utf8_lossy(&data.to_vec()).to_string();
            let mut results = data
                .lines()
                .filter(|line| {
                    line.starts_with("data: ")
                })
                .filter_map(|line| {
                    let json = &line["data: ".len()..];
                    let json = serde_json::from_str::<response::streaming::ResponseChunk>(json).ok()?;
                    let msg = json.choices
                        .iter()
                        .filter_map(|x| x.delta.content.clone())
                        .collect::<String>();
                    if let Some(logger) = logger.as_mut() {
                        logger.log(&msg);
                    }
                    Some(json)
                })
                .collect::<Vec<_>>();
            outputs.append(&mut results);
        }
        if let Some(logger) = logger.as_mut() {
            logger.log("\n");
        }
        Ok(ResponseChunkCollection(outputs))
    }
}

#[derive(Debug, Clone)]
pub struct ResponseChunkCollection(pub Vec<response::streaming::ResponseChunk>);

impl ResponseChunkCollection {
    pub fn len(&self) -> usize { self.0.len() }
    pub fn content(&self, index: usize) -> Option<String> {
        let output = self.0
            .iter()
            .filter_map(|x| x.choices.get(index).cloned())
            .filter_map(|x| x.delta.content)
            .collect::<Vec<_>>();
        if output.is_empty() {
            return None
        }
        Some(output.join(""))
    }
}



//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone, Copy)]
pub struct URL(pub &'static str);

impl URL {
    pub const OPEN_AI_CHAT_COMPLETIONS: Self = URL("https://api.openai.com/v1/chat/completions");
    pub const OCTO_AI_CHAT_COMPLETIONS: Self = URL("https://text.octoai.run/v1/chat/completions");
    pub const MISTRAL_AI_CHAT_COMPLETIONS: Self = URL("https://api.mistral.ai/v1/chat/completions");
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub enum ApiError {
    /// # TODO
    APIConnectionError,
    /// # TODO
    APITimeoutError,
    /// # TODO
    InternalServerError,
    /// # 401 - Invalid Authentication
    AuthenticationError,
    /// # 400 - Bad Request Error
    BadRequestError,
    /// # 409 - Conflict Error
    ConflictError,
    /// # 404 - Not Found Error
    NotFoundError,
    /// # 403 - Permission Denied Error
    PermissionDeniedError,
    /// # 429 - Rate limit reached for requests
    RateLimitError,
    /// # 422 - Unprocessable Entity Error
    UnprocessableEntityError,
}

impl ApiError {
    pub(crate) fn from_code(status: impl Into<u16>) -> Option<Self> {
        match status.into() {
            400 => Some(ApiError::BadRequestError),
            401 => Some(ApiError::AuthenticationError),
            403 => Some(ApiError::PermissionDeniedError),
            404 => Some(ApiError::NotFoundError),
            409 => Some(ApiError::ConflictError),
            422 => Some(ApiError::UnprocessableEntityError),
            429 => Some(ApiError::RateLimitError),
            _ => None,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ApiError::APIConnectionError => "API ERROR: api connection error.",
            ApiError::APITimeoutError => "API ERROR: api timeout error.",
            ApiError::InternalServerError => "API ERROR: internal server error.",
            ApiError::AuthenticationError => "API ERROR: authentication error.",
            ApiError::BadRequestError => "API ERROR: bad request error.",
            ApiError::ConflictError => "API ERROR: conflict error.",
            ApiError::NotFoundError => "API ERROR: not found error.",
            ApiError::PermissionDeniedError => "API ERROR: permission denied error.",
            ApiError::RateLimitError => "API ERROR: rate limit error.",
            ApiError::UnprocessableEntityError => "API ERROR: unprocessable entity error.",
        };
        write!(f, "{label}")
    }
}

impl std::error::Error for ApiError {}
