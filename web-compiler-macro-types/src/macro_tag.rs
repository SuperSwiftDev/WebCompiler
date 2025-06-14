use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

use xml_ast::{transform::ProcessMode, AttributeMap, Element, Fragment, Node};

use crate::environment::{LexicalEnvironment, MacroIO, MacroRuntime};

/// Applied during the top-down traversal phase.
pub trait MacroTag {
    fn tag_name(&self) -> &'static str;
    fn apply(
        &self,
        attributes: AttributeMap,
        children: Fragment,
        scope: &mut LexicalEnvironment,
        runtime: &MacroRuntime,
    ) -> MacroIO<Node>;
}

#[derive(Default, Clone)]
pub struct MacroTagSet {
    pub macros: BTreeMap<&'static str, Rc<dyn MacroTag>>,
    supported_tags: BTreeSet<&'static str>,
}

impl MacroTagSet {
    fn synced(mut self) -> Self {
        self.supported_tags = self.macros
            .keys()
            .map(|x| *x)
            .collect::<BTreeSet<_>>();
        self
    }
    pub fn from_vec(macros: Vec<Rc<dyn MacroTag>>) -> Self {
        let macros = macros
            .into_iter()
            .map(|x| (x.tag_name(), x))
            .collect::<BTreeMap<_, _>>();
        Self { macros: macros, supported_tags: Default::default() }.synced()
    }
    pub fn try_evaluate(
        &self,
        element: Element,
        scope: &mut LexicalEnvironment,
        runtime: &MacroRuntime,
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
