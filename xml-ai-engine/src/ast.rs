// use crate::frontend_ast::PromptAttributes;
pub mod value_types;
pub mod text;
pub mod message;
pub mod prompt;
pub mod document;

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — SETTINGS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct TargetPrompt {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DocumentEvaluationSetting {
    target_prompt: TargetPrompt,
}

impl DocumentEvaluationSetting {
    pub fn new(target_prompt: TargetPrompt) -> Self {
        Self { target_prompt }
    }
    pub fn target_prompt(&self) -> &TargetPrompt {
        &self.target_prompt
    }
}

// ————————————————————————————————————————————————————————————————————————————
// EVALUATION ENVIRONMENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct EvaluationEnvironment {
    // pub scope
}

// // ————————————————————————————————————————————————————————————————————————————
// // DATA MODEL — DOCUMENT CHILD NODE
// // ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub enum Block {}


// // ————————————————————————————————————————————————————————————————————————————
// // DATA MODEL — DOCUMENT
// // ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub struct Document {}

