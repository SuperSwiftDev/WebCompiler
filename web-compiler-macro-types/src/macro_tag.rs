use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

use xml_ast::{transform::ProcessMode, AttributeMap, Element, Fragment, Node};

use crate::lexical_env::{Featureset, ProcessScope, MacroIO, SourceHost};

/// Applied during the top-down traversal phase.
pub trait MacroTag {
    type Runtime: Featureset;
    fn tag_name(&self) -> &'static str;
    fn apply(
        &self,
        attributes: AttributeMap,
        children: Fragment,
        scope: &mut ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<Node>;
}

#[derive(Default, Clone)]
pub struct MacroTagSet<Runtime: SourceHost> {
    pub macros: BTreeMap<&'static str, Rc<dyn MacroTag<Runtime = Runtime>>>,
    supported_tags: BTreeSet<&'static str>,
}

impl<Runtime: SourceHost> MacroTagSet<Runtime> {
    fn synced(mut self) -> Self {
        self.supported_tags = self.macros
            .keys()
            .map(|x| *x)
            .collect::<BTreeSet<_>>();
        self
    }
    pub fn from_vec(macros: Vec<Rc<dyn MacroTag<Runtime=Runtime>>>) -> Self {
        let macros = macros
            .into_iter()
            .map(|x| (x.tag_name(), x))
            .collect::<BTreeMap<_, _>>();
        Self { macros: macros, supported_tags: Default::default() }.synced()
    }
    pub fn try_evaluate(
        &self,
        element: Element,
        scope: &mut ProcessScope,
        runtime: &Runtime,
    ) -> MacroIO<ProcessMode<Element, Node>> {
        let Element { tag, attributes, children } = element;
        let element_tag_str = tag.as_normalized();
        if self.supported_tags.contains(element_tag_str) {
            if let Some(macro_tag) = self.macros.get(element_tag_str) {
                return macro_tag
                    .apply(attributes, children, scope, runtime)
                    .map(ProcessMode::Manual)
            }
        }
        MacroIO::wrap(ProcessMode::Default(Element {tag, attributes, children}))
    }
}

// #[derive(Debug, Clone)]
// pub struct MacroTagSchema {}
