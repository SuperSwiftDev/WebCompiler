use std::path::PathBuf;

use crate::dsl::{Document, MessageBreakpointElement, MessageContent, MessageElement, PromptChild, PromptElement, Role};
use serde::{Deserialize, Serialize};
use xml_ai_client::request::Message;

// ————————————————————————————————————————————————————————————————————————————
// INTERNAL - CORE
// ————————————————————————————————————————————————————————————————————————————

async fn invoke(messages: &[Message]) -> String {
    use xml_ai_client::client::URL;
    let api_key_path = PathBuf::from("secrets/open-ai.key");
    assert!(api_key_path.exists());
    use xml_ai_client::request::OpenAiModels;
    let request_builder = xml_ai_client::request::RequestBuilder::default()
        .with_messages(messages.to_owned())
        .with_model(OpenAiModels::gpt_4)
        .with_stream(true);
    let client_builder = xml_ai_client::client::ClientBuilder::default()
        .with_api_url(URL::OPEN_AI_CHAT_COMPLETIONS)
        .with_api_key_path(&api_key_path)
        .with_request_body(request_builder)
        .with_logger(xml_ai_client::log::StdErrLogger::default().with_colorize(true));
    let client = client_builder.build_streaming_api_call().unwrap();
    let output_result = client.execute_async().await;
    let output = output_result.unwrap();
    output.content(0).unwrap()
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum DocumentTarget {
    PromptName(String),
}

impl DocumentTarget {
    pub fn matches_prompt_element(&self, prompt: &PromptElement) -> bool {
        match self {
            Self::PromptName(name) => {
                name == prompt.attributes.name.as_str()
            }
        }
    }
}

impl Document {
    pub async fn evaluate(&self, prompt_name: impl AsRef<str>) -> CompletedPrompt {
        let prompt = self
            .lookup_prompt(prompt_name)
            .unwrap();
        prompt.evaluate().await
    }
}

impl PromptElement {
    pub async fn evaluate(&self) -> CompletedPrompt {
        // let mut history = Vec::<Message>::new();
        let mut prompt_context = PromptContext::default();
        for child in self.children.iter() {
            prompt_context.push(child).await;
        }
        prompt_context.finalize().await
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromptContext {
    history: Vec<Message>,
    evaluated: bool,
}

impl PromptContext {
    async fn push(&mut self, prompt_child: &PromptChild) {
        match prompt_child {
            PromptChild::Message(message) => {
                self.push_message_element(message).await;
                self.evaluated = false;
            }
            PromptChild::MessageBreakpoint(message) => {
                let message_element = message.evaluate(&self.history).await;
                self.push_message_element(&message_element).await;
                self.evaluated = true;
            }
        }
    }
    async fn push_message_element(&mut self, message: &MessageElement) {
        match message.attributes.role {
            Role::System => self.history.push(Message::system(message.content.to_string())),
            Role::User => self.history.push(Message::user(message.content.to_string())),
            Role::Assistant => self.history.push(Message::assistant(message.content.to_string())),
        }
    }
    async fn finalize(self) -> CompletedPrompt {
        if !self.evaluated {
            let output = invoke(&self.history).await;
            return CompletedPrompt { history: self.history, output: Some(output) }
        }
        CompletedPrompt { history: self.history, output: None }
    }
}

impl MessageBreakpointElement {
    pub async fn evaluate(&self, history: &[Message]) -> MessageElement {
        let output = invoke(history).await;
        MessageElement {
            attributes: self.attributes.to_owned(),
            content: MessageContent::new(output),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompletedPrompt {
    pub history: Vec<Message>,
    pub output: Option<String>,
}

impl CompletedPrompt {
    pub fn finale_output_content(&self) -> String {
        if let Some(output) = self.output.as_ref() {
            return output.to_owned()
        }
        self.history.last().unwrap().content().to_owned()
    }
}

