mod content;
mod enumerate;
mod include;
mod provision;
mod context;
mod bind;
mod inject;
mod route;
mod define;
mod hoist;
mod define_title;

use std::rc::Rc;

pub use content::*;
pub use enumerate::*;
pub use include::*;
pub use provision::*;
pub use context::*;
pub use bind::*;
pub use inject::*;
pub use route::*;
pub use define::*;
pub use hoist::*;
pub use define_title::*;

use macro_types::macro_tag::{MacroTag, MacroTagSet};

use web_compiler_types::CompilerRuntime;

pub fn standard_macro_tags() -> Vec<Rc<dyn MacroTag<Runtime = CompilerRuntime>>> {
    vec![
        Rc::new(ContentMacroTag),
        Rc::new(IncludeMacroTag),
        Rc::new(EnumerateMacroTag),
        Rc::new(BindMacroTag),
        Rc::new(InjectMacroTag),
        Rc::new(ProvisionMacroTag),
        Rc::new(ContextMacroTag),
        Rc::new(RouteMacroTag),
        Rc::new(DefineMacroTag),
        Rc::new(HoistMacroTag),
        Rc::new(DefineTitleMacroTag),
    ]
}

pub fn standard_macro_tag_set() -> MacroTagSet<CompilerRuntime> {
    MacroTagSet::from_vec(standard_macro_tags())
}


