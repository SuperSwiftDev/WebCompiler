#![allow(unused)]
use crate::dsl::Role;

// ————————————————————————————————————————————————————————————————————————————
// RUNTIME - DATA MODEL - MESSAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
struct MessageObject {
    pub role: Role,
    pub content: String,
}

// ————————————————————————————————————————————————————————————————————————————
// RUNTIME - DATA MODEL - PROMPT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
struct PromptObject {
    pub messages: Vec<MessageObject>,
}

// ————————————————————————————————————————————————————————————————————————————
// RUNTIME CONTEXT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct Runtime {}

// ————————————————————————————————————————————————————————————————————————————
// RUNTIME - DOCUMENT ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

impl crate::dsl::Document {
    // pub fn evaluate(runtime: &mut Runtime) {
    //     unimplemented!()
    // }
}

// ————————————————————————————————————————————————————————————————————————————
// RUNTIME - PROMPT ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

impl crate::dsl::PromptTag {
    pub fn evaluate(&self) {
        unimplemented!()
    }
}

impl crate::dsl::MessageTag {}

impl crate::dsl::MessageBreakpointTag {}

