use crate::frontend_ast::PromptAttributes;

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


// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — MESSAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum TextExpression {
    Text(String),
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — MESSAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct MessageBlock {
    pub attributes: PromptAttributes,
    pub breakpoint_mode: bool,
    pub children: Vec<TextExpression>,
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — PROMPT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct PromptBlock {
    pub attributes: PromptAttributes,
    pub children: Vec<MessageBlock>,
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — DOCUMENT CHILD NODE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum Block {}


// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — DOCUMENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct Document {}

