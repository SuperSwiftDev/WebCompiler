//! The data model for ChatGPT (and ChatGPT compatible) responses.
//!
//! Supports both streaming and regular (batch) responses.
use serde::{Deserialize, Serialize};
pub use super::request::{Integer, Number};

pub mod batch;
pub mod streaming;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub index: Integer,
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbability {
    pub content: Option<Vec<MessageLogProbability>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLogProbability {
    pub token: String,
    pub logprob: Number,
    pub bytes: Vec<Integer>,
    pub top_logprobs: Vec<TopLogProbability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProbability {
    pub token: String,
    pub logprob: Number,
    pub bytes: Vec<Integer>,
}


