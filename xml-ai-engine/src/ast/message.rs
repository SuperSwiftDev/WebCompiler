
// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — MESSAGE
// ————————————————————————————————————————————————————————————————————————————

use crate::ast::text::TextNode;

#[derive(Debug, Clone)]
pub struct MsgBlock {
    pub breakpoint_mode: bool,
    pub children: Vec<TextNode>,
}

