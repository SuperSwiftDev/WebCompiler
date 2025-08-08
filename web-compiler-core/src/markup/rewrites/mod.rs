pub mod attributes;

mod style;
mod document;
mod headings;

use std::rc::Rc;
use macro_types::tag_rewrite_rule::{TagRewriteRule, TagRewriteRuleSet};

use web_compiler_types::CompilerRuntime;

pub fn standard_tag_rewrite_rules() -> Vec<Rc<dyn TagRewriteRule<Runtime=CompilerRuntime>>> {
    vec![
        Rc::new(style::StyleMacroTag),
        Rc::new(document::DocumentHead),
        Rc::new(document::DocumentBody),
        Rc::new(document::Document),
        Rc::new(headings::H1RewriteRule),
        Rc::new(headings::H2RewriteRule),
        Rc::new(headings::H3RewriteRule),
        Rc::new(headings::H4RewriteRule),
        Rc::new(headings::H5RewriteRule),
        Rc::new(headings::H6RewriteRule),
    ]
}

pub fn standard_tag_rewrite_rule_set() -> TagRewriteRuleSet<CompilerRuntime> {
    TagRewriteRuleSet::from_vec(standard_tag_rewrite_rules())
}

