use web_compiler_io_types::IO;
use web_compiler_io_types::Effectful;

use crate::AttributeMap;
use crate::Element;
use crate::Fragment;
use crate::Node;
use crate::TagBuf;

pub trait EffectfulMarkupTransformer {
    /// Output value type.
    type Output;
    /// Top-down lexical environment.
    type Scope: Clone;
    /// Bottom-up accumulated state.
    type Effect: Effectful;

    /// Transforms a raw text node into an output.
    fn transform_text(&self, text: String, scope: &mut Self::Scope) -> IO<Self::Output, Self::Effect>;
    /// Transforms a fragment (sequence of child nodes) into a single output.
    fn transform_fragment(&self, fragment: Vec<Self::Output>, scope: &mut Self::Scope) -> IO<Self::Output, Self::Effect>;
    /// Transforms an element node (tag + attributes + children) into an output.
    fn transform_element(
        &self,
        tag: TagBuf,
        attrs: AttributeMap,
        children: Vec<Self::Output>,
        scope: &mut Self::Scope,
    ) -> IO<Self::Output, Self::Effect>;

    /// Optional override to intercept a node before it is traversed.
    ///
    /// By default, this simply returns `Right(node)` to allow normal recursion.
    /// Override this to implement top-down macro expansion or short-circuit rewriting.
    fn manual_top_down_node_handler(&self, node: Node, scope: &mut Self::Scope) -> IO<ProcessMode<Node, Self::Output>, Self::Effect> {
        let _ = scope;
        IO::wrap(ProcessMode::Default(node))
    }
    /// Optional override to intercept an element before its children are traversed.
    ///
    /// Like the node-level hook, this allows for rewriting or short-circuiting based
    /// on macro syntax or binding constructs.
    fn manual_top_down_element_handler(&self, element: Element, scope: &mut Self::Scope) -> IO<ProcessMode<Element, Self::Output>, Self::Effect> {
        let _ = scope;
        IO::wrap(ProcessMode::Default(element))
    }
}


pub enum ProcessMode<Input, Output> {
    Default(Input),
    Manual(Output)
}

/// Applies a transformer while retaining the IO wrapper.
///
/// This is intended for sub-level processing where internal context
/// propagation or chaining of `IO` operations is still needed.
#[inline]
pub fn apply_effectful_markup_transformer<T: EffectfulMarkupTransformer>(
    node: Node,
    transformer: &T,
    scope: &mut T::Scope
) -> IO<T::Output, T::Effect> {
    node.apply_effectful_markup_transformer(transformer, scope)
}

impl Node {
    fn apply_effectful_markup_transformer<T: EffectfulMarkupTransformer>(self, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
        transformer
            .manual_top_down_node_handler(self, scope)
            .and_then(|process| {
                match process {
                    ProcessMode::Default(node) => {
                        match node {
                            Node::Text(text) => transformer.transform_text(text, scope),
                            Node::Element(element) => element.apply_effectful_markup_transformer(transformer, scope),
                            Node::Fragment(fragment) => fragment.apply_effectful_markup_transformer(transformer, scope),
                        }
                    }
                    ProcessMode::Manual(output) => IO::wrap(output),
                }
            })
    }
}

impl Element {
    fn apply_effectful_markup_transformer<T: EffectfulMarkupTransformer>(self, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
        transformer.manual_top_down_element_handler(self, scope).and_then(|process| {
            match process {
                ProcessMode::Default(element) => {
                    let Element { tag, attributes, children } = element;
                    let mut local_context = scope.clone();
                    process_node_sequence(
                        children.to_vec(),
                        transformer,
                        &mut local_context
                    )
                    .and_then(|children| {
                        transformer.transform_element(tag, attributes, children, scope)
                    })
                }
                ProcessMode::Manual(output) => IO::wrap(output),
            }
        })
    }
}

impl Fragment {
    fn apply_effectful_markup_transformer<T: EffectfulMarkupTransformer>(self, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
        process_node_sequence(self.to_vec(), transformer, scope).and_then(|xs| {
            transformer.transform_fragment(xs, scope)
        })
    }
}


/// Transforms a sequence of nodes and flattens the result.
fn process_node_sequence<T: EffectfulMarkupTransformer>(
    nodes: Vec<Node>,
    transformer: &T,
    scope: &mut T::Scope
) -> IO<Vec<T::Output>, T::Effect> {
    web_compiler_io_types::io_iter_map_mut(nodes.len(), nodes, |child| {
        child.apply_effectful_markup_transformer(transformer, scope)
    })
}

