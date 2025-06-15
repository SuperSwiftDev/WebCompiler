pub mod attributes;

mod style;
mod link;

use std::rc::Rc;

pub use style::*;
pub use link::*;

use macro_types::tag_rewrite_rule::{TagRewriteRule, TagRewriteRuleSet};

// pub static STANDARD_MACRO_TAGS: &'static [i8] = &[];
pub fn standard_tag_rewrite_rules() -> Vec<Rc<dyn TagRewriteRule>> {
    vec![
        // Rc::new(StyleMacroTag),
    ]
}

pub fn standard_tag_rewrite_rule_set() -> TagRewriteRuleSet {
    TagRewriteRuleSet::from_vec(standard_tag_rewrite_rules())
}


