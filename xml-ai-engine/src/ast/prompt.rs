use crate::common::prompt::PromptSettings;

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — PROMPT
// ————————————————————————————————————————————————————————————————————————————


#[derive(Debug, Clone)]
pub struct PromptBlock {
    pub settings: PromptSettings,
    // pub children: Vec<MessageBlock>,
}

