use macro_types::{lexical_env::{SourceHostRef, SourcePathResolver}, tag_rewrite_rule::TagRewriteRuleSet};
use xml_ast::{traversal::ElementVisitor, Element, Node};

use crate::macro_types::project::ResolvedDependencies;
use web_compiler_types::CompilerRuntime;

pub struct PostProcessor<'a> {
    pub rules: &'a TagRewriteRuleSet<CompilerRuntime>,
    pub path_resolver: SourcePathResolver<'a>,
    pub resolved_dependencies: &'a mut ResolvedDependencies,
    pub source_host: SourceHostRef<'a>,
}

impl<'a> PostProcessor<'a> {
    pub fn new(
        rules: &'a TagRewriteRuleSet<CompilerRuntime>,
        path_resolver: SourcePathResolver<'a>,
        resolved_dependencies: &'a mut ResolvedDependencies,
        source_host: SourceHostRef<'a>,
    ) -> Self {
        Self { rules, path_resolver, resolved_dependencies, source_host }
    }
    pub fn apply(&mut self, node: Node) -> Node {
        xml_ast::traversal::apply_element_visitor(node, self)
    }
}

impl<'a> ElementVisitor for PostProcessor<'a> {
    fn visit_element(
        &mut self,
        tag: xml_ast::TagBuf,
        mut attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
    ) -> Node {
        super::rewrites::attributes::resolve_virtual_path_attributes(
            &tag,
            &mut attributes,
            self.path_resolver,
            &mut self.resolved_dependencies,
        );
        self.rules.try_apply_post_processors(Element { tag, attributes, children }, &self.source_host)
    }
}
