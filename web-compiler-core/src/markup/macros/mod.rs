mod content;
mod enumerate;
mod include;
mod provision;
mod context;
mod bind;
mod inject;

use std::rc::Rc;

pub use content::*;
pub use enumerate::*;
pub use include::*;
pub use provision::*;
pub use context::*;
pub use bind::*;
pub use inject::*;

use macro_types::macro_tag::{MacroTag, MacroTagSet};

use crate::system::CompilerRuntime;

pub fn standard_macro_tags() -> Vec<Rc<dyn MacroTag<Runtime = CompilerRuntime>>> {
    vec![
        Rc::new(ContentMacroTag),
        Rc::new(IncludeMacroTag),
        Rc::new(EnumerateMacroTag),
        Rc::new(BindMacroTag),
        Rc::new(InjectMacroTag),
        Rc::new(ProvisionMacroTag),
        Rc::new(ContextMacroTag),
    ]
}

pub fn standard_macro_tag_set() -> MacroTagSet<CompilerRuntime> {
    MacroTagSet::from_vec(standard_macro_tags())
}


