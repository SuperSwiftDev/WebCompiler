use macro_types::{environment::SourcePathResolver, tag_rewrite_rule::TagRewriteRuleSet};
use xml_ast::{traversal::ElementVisitor, Element, Node};

use crate::macro_types::project::ResolvedDependencies;

pub struct PostProcessor<'a> {
    pub rules: &'a TagRewriteRuleSet,
    pub path_resolver: SourcePathResolver<'a>,
    pub resolved_dependencies: &'a mut ResolvedDependencies,
}

impl<'a> PostProcessor<'a> {
    pub fn new(
        rules: &'a TagRewriteRuleSet,
        path_resolver: SourcePathResolver<'a>,
        resolved_dependencies: &'a mut ResolvedDependencies,
    ) -> Self {
        Self { rules, path_resolver, resolved_dependencies }
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
        self.rules.try_apply_post_processors(Element { tag, attributes, children })
    }
}
