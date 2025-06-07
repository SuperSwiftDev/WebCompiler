#![allow(unused)]
use std::collections::HashMap;

use web_compiler_html_ast::{Element, Html, ParserMode, TagBuf};
use web_compiler_html_ast::traversal::ElementVisitor;
use web_compiler_common::InputRule;
use web_compiler_common::ProjectContext;
use web_compiler_common::SourceContext;
use web_compiler_common::PathResolver;

mod environment;
mod resolve_links;

pub use environment::*;
pub use resolve_links::*;

#[derive(Debug, Clone)]
pub struct PostProcessor {
    path_resolver: PathResolver,
    resolved_dependencies: ResolvedDependencies,
}

impl PostProcessor {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self {
            resolved_dependencies: Default::default(),
            path_resolver
        }
    }
    pub fn path_resolver(&self) -> &PathResolver {
        &self.path_resolver
    }
}

impl ElementVisitor for PostProcessor {
    fn visit_element(
        &mut self,
        tag: TagBuf,
        mut attrs: HashMap<String, String>,
        children: Vec<Html>,
    ) -> Html {
        resolve_virtual_path_attributes(&tag, &mut attrs, &self.path_resolver, &mut self.resolved_dependencies);
        Html::Element(Element { tag, attrs, children })
    }
}



pub fn execute(
    html: Html,
    path_resolver: PathResolver,
) -> (Html, ResolvedDependencies) {
    let mut post_processor = PostProcessor::new(path_resolver);
    let result = html.apply_element_visitor(&mut post_processor);
    (result, post_processor.resolved_dependencies)
}


