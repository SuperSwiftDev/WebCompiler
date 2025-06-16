pub mod attributes;
mod style;
pub use style::*;

use std::rc::Rc;
use macro_types::tag_rewrite_rule::{TagRewriteRule, TagRewriteRuleSet};

pub fn standard_tag_rewrite_rules() -> Vec<Rc<dyn TagRewriteRule>> {
    vec![
        Rc::new(StyleMacroTag),
    ]
}

pub fn standard_tag_rewrite_rule_set() -> TagRewriteRuleSet {
    TagRewriteRuleSet::from_vec(standard_tag_rewrite_rules())
}

