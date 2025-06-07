//! # HTML Transformation Engine
//!
//! This module provides a context-aware HTML transformation system intended for advanced
//! use cases such as macro expansion, lowering, or custom binding-aware rewrites.
//!
//! Unlike a traditional visitor (which is also available in this crate), this system
//! is designed to support **two-phase transformation logic**:
//!
//! ## Traversal Semantics
//!
//! - **Top-down propagation**: A context object (`Self::Context`) is passed down from
//!   parent nodes to children. This allows the implementer to carry scoping information,
//!   binding environments, or stateful macro evaluation environments.
//!
//! - **Bottom-up composition**: After child nodes are transformed, their outputs are
//!   passed upward and reassembled via `transform_element` / `transform_fragment`.
//!
//! ## Core Traits and Types
//!
//! - [`HtmlTransformer`] — the main trait for implementing recursive transformations with
//!   optional interception logic at both node and element levels.
//!
//! - [`IO<Value, Context>`] — a lightweight, monadic wrapper around transformation results,
//!   used to thread contextual information. Currently implemented using `PhantomData`, but
//!   designed to evolve toward richer context management in downstream crates.
//!
//! - [`Either<L, R>`] — a branching return type that supports short-circuiting transformation
//!   of specific nodes or elements (e.g. for macros, placeholders, special syntax).
//!
//! ## Design Philosophy
//!
//! This module exists in the `html-ast` crate and deliberately avoids coupling to macro-specific
//! semantics or evaluation logic. The goal is to expose just enough generic traversal functionality
//! to support higher-level use cases, **without introducing cyclic dependencies** or opinionated
//! rewriting strategies at this layer.
//!
//! ### Key Design Constraints:
//!
//! - **No context mutation** or binding logic is implemented here — implementers define their own
//!   context types and manage state outside of this crate.
//!
//! - **No macro knowledge** exists here — this crate knows nothing about `@if`, `@repeat`, or similar.
//!   Macro recognition and expansion are left to downstream consumers.
//!
//! - **Extensible composition** is supported by convention — transformers may chain or compose
//!   through user-defined combinators, though the base trait only allows one implementation at a time.
//!
//! ## Use Cases
//!
//! - Tree rewriting (e.g. normalize or collapse HTML fragments)
//! - Macro expansion in templating engines
//! - AST lowering in interpreters or transpilers
//! - Static analysis or binding resolution
//!
//! ## Related Tools
//!
//! - For simpler stateless traversals, see the `html_visitor` module.
//! - For macro engines or pass orchestration, see external crates like `macro_env`.
//!
//! This module is intentionally **unergonomic by default**, providing power and flexibility
//! to downstream consumers that need it — while remaining decoupled from specific language
//! semantics or runtime behaviors.
#![allow(unused)]
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{Element, Html, TagBuf};
use pretty_tree::value;

// trait Functor {
//     type Wrapped<T>;
//     fn fmap<T, U>(input: Self::Wrapped<T>, f: impl Fn(T) -> U) -> Self::Wrapped<U>;
// }

/// A simple sum type used to represent a branching result:
///
/// - `Left(L)`: A final transformed result that short-circuits further processing
/// - `Right(R)`: The original or rewritten node to continue traversal on
///
/// This is used to allow early returns from manual hooks in the transformation system.
#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait EffectPropagator: Clone + Default {
    fn merge_mut(&mut self, other: Self);
    fn union(left: Self, right: Self) -> Self {
        let mut left = left;
        left.merge_mut(right);
        left
    }
}

/// A wrapper representing a transformation result that may carry a context type.
///
/// This struct is designed to model an extensible monadic computation pattern, allowing
/// top-down context propagation alongside bottom-up node expansion.
///
/// `Context` is tracked at the type level via `PhantomData`, and is not actually used
/// during runtime computation at this layer. The intent is to permit higher-level logic
/// (in macro-aware crates) to define richer behavior without requiring changes to the
/// `html-ast` crate.
///
/// This type is **currently a stub**, and may be extended in the future to propagate
/// real context or support error-aware monadic chaining.
#[derive(Debug, Clone)]
pub struct IO<Value, Effect: EffectPropagator> {
    pub(crate) value: Value,
    /// Usually the local context.
    /// 
    /// By default I intend for everything to be lexically scoped but fragments
    /// (internal compiler mechanism) complicates this, a binder (that modifies
    /// the environment for subsequent nodes) declared in a fragment need to
    /// propagate that context beyond their fragment level or so I presume.
    /// So this gives the parent a view into the inner scope.
    /// 
    /// **TODO:** EMPTY
    pub(crate) env: Effect,
}

impl<Value, Effect: EffectPropagator> IO<Value, Effect> {
    /// Wraps a value in the IO monad.
    #[inline]
    pub fn wrap(value: Value) -> IO<Value, Effect> {
        Self { value, env: Effect::default() }
    }
    /// Applies a transformation to the contained value.
    #[inline]
    pub fn map<Result>(self, apply: impl FnOnce(Value) -> Result) -> IO<Result, Effect> {
        IO { value: apply(self.value), env: self.env }
    }
    /// Chains an operation that returns another IO.
    #[inline]
    pub fn and_then<Result>(self, apply: impl FnOnce(Value) -> IO<Result, Effect>) -> IO<Result, Effect> {
        let IO { value, mut env } = self;
        let IO { value, env: new_env } = apply(value);
        env.merge_mut(new_env);
        IO { value, env }
    }
    pub fn and_modify_context(self, apply: impl FnOnce(&mut Effect) -> ()) -> IO<Value, Effect> {
        let IO { value, mut env } = self;
        apply(&mut env);
        Self { value, env }
    }
    /// Lifts a collection of IOs into an IO of a collection, discarding context.
    pub fn flatten(size_hint: usize, items: impl IntoIterator<Item=IO<Value, Effect>>) -> IO<Vec<Value>, Effect> {
        let initial_state = IO::<Vec<Value>, Effect>::wrap(Vec::with_capacity(size_hint));
        items
            .into_iter()
            .fold(initial_state, |mut acc, item| {
                let IO { value, env } = item;
                acc.env.merge_mut(env);
                acc.value.push(value);
                acc
            })
    }
    pub fn flatten_vev(items: Vec<IO<Value, Effect>>) -> IO<Vec<Value>, Effect> {
        Self::flatten(items.len(), items)
    }
    /// Non-compositional deep-flattening of nested IOs.
    pub fn flatten_deep(items: Vec<IO<Vec<Value>, Effect>>) -> IO<Vec<Value>, Effect> {
        let initial_state = IO::<Vec<Value>, Effect>::wrap(Vec::with_capacity(items.len() * 3));
        items
            .into_iter()
            .fold(initial_state, |mut acc, item| {
                let IO { value, env } = item;
                acc.env.merge_mut(env);
                acc.value.extend(value);
                acc
            })
    }
    pub fn collapse(self) -> (Value, Effect) {
        ( self.value, self.env )
    }
}

/// A trait for implementing context-aware HTML tree transformations.
///
/// Unlike simple visitors, this trait supports a two-phase traversal strategy:
///
/// - **Top-down override hooks** (`manual_top_down_*`) that can intercept and rewrite nodes
///   before default traversal continues — useful for macro expansion, syntax rewrites, etc.
/// - **Bottom-up reconstruction** via `transform_*` methods that consume already-transformed children.
///
/// This trait is intentionally designed for use by higher-level crates — such as macro processors,
/// transpilers, or compiler front-ends — where special-case expansion and scoping logic is required.
pub trait HtmlTransformer where Self: Sized {
    /// The output type produced by each node transformation.
    type Output;
    /// The type of top-down context that may influence transformations.
    /// 
    /// Top-down environment.
    type Scope: Clone;
    /// Bottom-up effect accumulator.
    type Effect: EffectPropagator;
    /// Transforms a raw text node into an output.
    fn transform_text(&self, text: String, scope: &mut Self::Scope) -> IO<Self::Output, Self::Effect>;
    /// Transforms a fragment (sequence of child nodes) into a single output.
    fn transform_fragment(&self, fragment: Vec<Self::Output>, scope: &mut Self::Scope) -> IO<Self::Output, Self::Effect>;
    /// Transforms an element node (tag + attributes + children) into an output.
    fn transform_element(
        &self,
        tag: TagBuf,
        attrs: HashMap<String, String>,
        children: Vec<Self::Output>,
        scope: &mut Self::Scope,
    ) -> IO<Self::Output, Self::Effect>;
    /// Optional override to intercept a node before it is traversed.
    ///
    /// By default, this simply returns `Right(node)` to allow normal recursion.
    /// Override this to implement top-down macro expansion or short-circuit rewriting.
    fn manual_top_down_node_handler(&self, node: Html, scope: &mut Self::Scope) -> IO<Either<Self::Output, Html>, Self::Effect> {
        IO::wrap(Either::Right(node))
    }
    /// Optional override to intercept an element before its children are traversed.
    ///
    /// Like the node-level hook, this allows for rewriting or short-circuiting based
    /// on macro syntax or binding constructs.
    fn manual_top_down_element_handler(&self, element: Element, scope: &mut Self::Scope) -> IO<Either<Self::Output, Element>, Self::Effect> {
        IO::wrap(Either::Right(element))
    }
    
    // /// Optional override for flattening a transformed output into multiple outputs.
    // ///
    // /// This is primarily used when expanding macros that produce multiple sibling nodes.
    // /// The default simply wraps the value into a single-item vector.
    // fn flatten(&self, output: Self::Output) -> Self::Output {
    //     output
    // }
    
    /// Entry point to transform a node.
    ///
    /// This method should not be overridden — it calls `apply_html_transformer`, which
    /// will automatically apply the appropriate traversal logic.
    /// 
    /// Keep this the default — should not override this method.
    #[inline]
    fn enter(&self, node: Html, scope: &mut Self::Scope) -> IO<Self::Output, Self::Effect> {
        node.apply_html_transformer(self, scope)
    }

    /// Entry point to transform a sequence of nodes.
    ///
    /// This method should not be overridden — it  will automatically apply the appropriate traversal logic.
    /// 
    /// Keep this the default — should not override this method.
    #[inline]
    fn enter_node_sequence(&self, fragment: Vec<Html>, scope: &mut Self::Scope) -> IO<Vec<Self::Output>, Self::Effect> {
        process_node_sequence(fragment, self, scope)
    }
}

/// Transforms an HTML node with a transformer - discarding internal context.
///
/// This is the entry point when no context propagation is needed —
/// typically from the root level of the program.
pub fn evaluate_html_transformer<T: HtmlTransformer>(html: Html, transformer: &T, scope: &mut T::Scope) -> (T::Output, T::Effect) {
    let IO { value, env } = html.apply_html_transformer(transformer, scope);
    (value, env)
}

/// Applies a transformer while retaining the IO wrapper.
///
/// This is intended for sub-level processing where internal context
/// propagation or chaining of `IO` operations is still needed.
pub fn apply_html_transformer<T: HtmlTransformer>(html: Html, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
    html.apply_html_transformer(transformer, scope)
}

impl Html {
    /// Applies the transformer to this node, honoring all manual and default semantics.
    ///
    /// This is the core dispatch logic — it checks if the transformer overrides
    /// the node manually, otherwise recurses into default bottom-up behavior.
    fn apply_html_transformer<T: HtmlTransformer>(self, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
        transformer.manual_top_down_node_handler(self, scope).and_then(|result| match result {
            Either::Left(left) => IO::wrap(left),
            Either::Right(right) => {
                match right {
                    Self::Element(element) => process_element(element, transformer, scope),
                    Self::Fragment(fragment) => process_fragment(fragment, transformer, scope),
                    Self::Text(text) => transformer.transform_text(text, scope),
                }
            }
        })
    }
}

/// Processes an element node, applying manual and default logic.
fn process_element<T: HtmlTransformer>(element: Element, transformer: &T, parent_context: &mut T::Scope) -> IO<T::Output, T::Effect> {
    transformer.manual_top_down_element_handler(element, parent_context).and_then(|result| match result {
        Either::Left(left) => IO::wrap(left),
        Either::Right(right) => {
            let mut local_context = parent_context.clone();
            let Element { tag, attrs, children } = right;
            process_node_sequence(children, transformer, &mut local_context).and_then(|children| {
                transformer.transform_element(tag, attrs, children, parent_context)
            })
        }
    })
}

/// Processes a fragment node by transforming all children and composing.
fn process_fragment<T: HtmlTransformer>(fragment: Vec<Html>, transformer: &T, scope: &mut T::Scope) -> IO<T::Output, T::Effect> {
    process_node_sequence(fragment, transformer, scope).and_then(|fragment| {
        transformer.transform_fragment(fragment, scope)
    })
}

/// Transforms a sequence of nodes and flattens the result.
fn process_node_sequence<T: HtmlTransformer>(fragment: Vec<Html>, transformer: &T, scope: &mut T::Scope) -> IO<Vec<T::Output>, T::Effect> {
    let mut processed: Vec<T::Output> = Vec::with_capacity(fragment.len() * 2);
    let mut return_env = T::Effect::default();
    for node in fragment {
        let output = node.apply_html_transformer(transformer, scope);
        let IO { value, env } = output;
        return_env.merge_mut(env);
        processed.push(value);
    }
    IO { value: processed, env: return_env }
}

