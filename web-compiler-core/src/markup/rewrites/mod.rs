pub mod attributes;

mod style;
mod document;

pub use style::*;
pub use document::*;

use std::rc::Rc;
use macro_types::tag_rewrite_rule::{TagRewriteRule, TagRewriteRuleSet};

use web_compiler_types::CompilerRuntime;

pub fn standard_tag_rewrite_rules() -> Vec<Rc<dyn TagRewriteRule<Runtime=CompilerRuntime>>> {
    vec![
        Rc::new(StyleMacroTag),
        Rc::new(DocumentHead),
        Rc::new(DocumentBody),
        Rc::new(Document),
    ]
}

pub fn standard_tag_rewrite_rule_set() -> TagRewriteRuleSet<CompilerRuntime> {
    TagRewriteRuleSet::from_vec(standard_tag_rewrite_rules())
}

