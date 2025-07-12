use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use xml_ast::{Element, Node};

use crate::environment::{Featureset, MacroIO, ProcessScope, SourceHostRef};

/// Applied during the bottom-up traversal phase.
pub trait TagRewriteRule {
    type Runtime: Featureset;
    fn tag_name(&self) -> &'static str;
    fn pre_process(
        &self,
        element: Element,
        scope: &mut ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<Node>;
    fn post_process(
        &self,
        element: Element,
        source_host_ref: &SourceHostRef,
    ) -> Node;
}

#[derive(Default, Clone)]
pub struct TagRewriteRuleSet<Runtime: Featureset> {
    pub macros: BTreeMap<&'static str, Rc<dyn TagRewriteRule<Runtime=Runtime>>>,
    supported_tags: BTreeSet<&'static str>,
}

impl<Runtime: Featureset> TagRewriteRuleSet<Runtime> {
    fn synced(mut self) -> Self {
        self.supported_tags = self.macros
            .keys()
            .map(|x| *x)
            .collect::<BTreeSet<_>>();
        self
    }
    pub fn from_vec(macros: Vec<Rc<dyn TagRewriteRule<Runtime=Runtime>>>) -> Self {
        let macros = macros
            .into_iter()
            .map(|x| (x.tag_name(), x))
            .collect::<BTreeMap<_, _>>();
        Self { macros: macros, supported_tags: Default::default() }.synced()
    }
    pub fn try_apply_pre_processors(
        &self,
        element: Element,
        scope: &mut ProcessScope,
        runtime: &Runtime,
    ) -> MacroIO<Node> {
        let element_tag_str = element.tag.as_normalized();
        if self.supported_tags.contains(element_tag_str) {
            if let Some(macro_tag) = self.macros.get(element_tag_str) {
                return macro_tag.pre_process(element, scope, runtime)
            }
        }
        MacroIO::wrap(Node::Element(element))
    }
    pub fn try_apply_post_processors( &self, element: Element, source_host_ref: &SourceHostRef) -> Node {
        let element_tag_str = element.tag.as_normalized();
        if self.supported_tags.contains(element_tag_str) {
            if let Some(macro_tag) = self.macros.get(element_tag_str) {
                return macro_tag.post_process(element, source_host_ref)
            }
        }
        Node::Element(element)
    }
}

