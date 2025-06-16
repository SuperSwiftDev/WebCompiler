use super::{FunctionCall, Integer, LogProbability, ToolCall};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseChunk {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: Integer,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub delta: MessageDelta,
    pub logprobs: Option<LogProbability>,
    pub finish_reason: Option<String>,
    pub index: Integer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: Option<String>,
}
