mod content;
mod enumerate;
mod include;
mod provision;
mod context;

use std::rc::Rc;

pub use content::*;
pub use enumerate::*;
pub use include::*;
pub use provision::*;
pub use context::*;

use macro_types::macro_tag::{MacroTag, MacroTagSet};

// pub static STANDARD_MACRO_TAGS: &'static [i8] = &[];
pub fn standard_macro_tags() -> Vec<Rc<dyn MacroTag>> {
    vec![
        Rc::new(ContentMacroTag),
        Rc::new(EnumerateMacroTag),
        Rc::new(IncludeMacroTag),
        Rc::new(ProvisionMacroTag),
        Rc::new(ContextMacroTag),
    ]
}

pub fn standard_macro_tag_set() -> MacroTagSet {
    MacroTagSet::from_vec(standard_macro_tags())
}


