#![allow(unused)]
use io_types::Effectful;
// use lightningcss::error;
use macro_types::environment::{AccumulatedEffects, Featureset, ProcessScope, MacroIO, SourceHostRef, SourceHost};
use macro_types::macro_tag::MacroTagSet;
use macro_types::project::FileInput;
use xml_ast::{transform::{EffectfulMarkupTransformer, ProcessMode}, AttributeMap, Element, Fragment, Node, TagBuf};

use web_compiler_types::CompilerRuntime;

pub struct PreProcessor {
    pub runtime: CompilerRuntime,
}

impl PreProcessor {
    pub fn new(runtime: CompilerRuntime) -> Self {
        Self { runtime }
    }
    pub fn fork(&self, file_input: &FileInput) -> Self {
        Self { runtime: self.runtime.fork(file_input) }
    }
    pub fn process_sequence(&self, nodes: Vec<Node>, scope: &mut ProcessScope) -> MacroIO<Vec<Node>> {
        xml_ast::transform::apply_effectful_markup_transformer_node_vec(nodes, self, scope)
    }
    pub fn load_compile(&self, scope: &mut ProcessScope) -> Result<MacroIO<Node>, PreProcessError> {
        let source = self.runtime
            .source_context()
            .file_input()
            .load_source_file()
            .map_err(PreProcessError::StdIo)?;
        let source_tree = {
            let output = xml_ast::parse_str_auto(&source);
            if !output.errors.is_empty() {
                let path = self.runtime.source_context();
                let path = path.file_input().source_file().to_str().unwrap();
                for error in output.errors {
                    eprintln!("Error while processing '{path}': {error}");
                }
            }
            output.output
        };
        // let source_tree = Node::Fragment(source_tree);
        let output = xml_ast::transform::apply_effectful_markup_transformer(source_tree, self, scope);
        Ok(output)
    }
}

#[derive(Debug)]
pub enum PreProcessError {
    StdIo(std::io::Error),
    ParserErrors(Vec<String>),
}

impl std::fmt::Display for PreProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdIo(error) => write!(f, "{error}"),
            Self::ParserErrors(errors) => {
                let error = errors.to_owned().join(" • ");
                write!(f, "{error}")
            },
        }
    }
}

impl std::error::Error for PreProcessError {}

impl EffectfulMarkupTransformer for PreProcessor {
    /// Output value type.
    type Output = Node;
    /// Top-down lexical environment.
    type Scope = ProcessScope;
    /// Bottom-up accumulated state.
    type Effect = AccumulatedEffects;

    /// Transforms a raw text node into an output.
    fn transform_text(&self, text: String, scope: &mut Self::Scope) -> MacroIO<Node> {
        MacroIO::wrap(Node::Text(text))
    }
    /// Transforms a fragment (sequence of child nodes) into a single output.
    fn transform_fragment(&self, fragment: Vec<Self::Output>, scope: &mut Self::Scope) -> MacroIO<Node> {
        MacroIO::wrap(Node::Fragment(Fragment::from_nodes(fragment)))
    }
    /// Transforms an element node (tag + attributes + children) into an output.
    fn transform_element(
        &self,
        tag: TagBuf,
        mut attributes: AttributeMap,
        children: Vec<Self::Output>,
        scope: &mut Self::Scope,
    ) -> MacroIO<Node> {
        let mut effects = AccumulatedEffects::default();
        super::rewrites::attributes::resolve_attribute_path_expressions(&mut attributes, scope, &self.runtime);
        super::rewrites::attributes::virtualize_attribute_paths(
            &tag,
            &mut attributes,
            &mut effects,
            self.runtime.source_context(),
        );
        let attribute_command = super::rewrites::attributes::AttributeCommand::from_attributes(
            &mut attributes,
            scope,
            &self.runtime
        );
        let children = Fragment::from_nodes(children);
        let element = Element { tag, attributes, children };
        self.runtime.rules()
            .try_apply_pre_processors(element, scope, &self.runtime)
            .and_modify_context(|ctx| {
                ctx.extend(effects)
            })
            .map(|node| {
                use super::rewrites::attributes::AttributeCommand;
                match attribute_command {
                    Some(attribute_command) => {
                        attribute_command.apply(node)
                    }
                    None => node
                }
            })
    }

    /// Optional override to intercept an element before its children are traversed.
    ///
    /// Like the node-level hook, this allows for rewriting or short-circuiting based
    /// on macro syntax or binding constructs.
    fn manual_top_down_element_handler(&self, element: Element, scope: &mut Self::Scope) -> MacroIO<ProcessMode<Element, Node>> {
        self.runtime.macros().try_evaluate(element, scope, &self.runtime)
    }
}


