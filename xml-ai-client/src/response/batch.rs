use super::{FunctionCall, Integer, LogProbability, ToolCall};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: Integer,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: Integer,
    pub message: Message,
    pub logprobs: Option<LogProbability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: String,
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: Integer,
    pub prompt_tokens: Integer,
    pub total_tokens: Integer,
}
