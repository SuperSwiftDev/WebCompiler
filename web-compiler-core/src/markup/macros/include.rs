use macro_types::macro_tag::MacroTag;
use macro_types::environment::{MacroIO, SourceHost};
use macro_types::project::FileInput;
use macro_types::scope::{BinderValue, JsonBinderValue};
use xml_ast::Node;

use web_compiler_types::CompilerRuntime;

use super::super::pre::{PreProcessError, PreProcessor};

#[derive(Debug, Clone, Copy, Default)]
pub struct IncludeMacroTag;

impl MacroTag for IncludeMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "include" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::environment::LexicalEnvironment,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let mut child_scope = scope.to_owned();
        let mut embedded_scope = scope.to_owned();
        let pre_processor = PreProcessor::new(runtime.clone());
        pre_processor
            .process_sequence(children.to_vec(), &mut child_scope)
            .and_then(|children| {
                if let Some(src_value) = attributes.get("src").cloned() {
                    let dependency = runtime.source_context().file_input().with_dependency_relation(src_value.as_str());
                    // - SCOPE -
                    let host_object = attributes
                        .clone()
                        .into_iter()
                        .filter(|(key, _)| key.as_str() != "src")
                        .map(|(key, value)| {
                            (key.as_str().to_owned(), JsonBinderValue::json_string(value.as_str().to_owned()))
                        })
                        .collect::<Vec<_>>();
                    let host_object = BinderValue::object(host_object);
                    embedded_scope.binding_scope.insert("content", BinderValue::fragment(children));
                    embedded_scope.binding_scope.insert("host", host_object);
                    // - LOAD - 
                    let embedded_path = runtime.source_context().file_input().source_dir().join(src_value.as_str());
                    let new_input_file = FileInput {
                        source: embedded_path.clone(),
                        public: None,
                    };
                    let runtime = runtime.fork(&new_input_file);
                    let pre_processor = PreProcessor::new(runtime.clone());
                    return pre_processor
                        .load_compile(&mut embedded_scope)
                        .unwrap_or_else(|error| {
                            let source_context = runtime.source_context();
                            let source_path = source_context.file_input().source.as_path();
                            match error {
                                PreProcessError::ParserErrors(errors) => {
                                    eprintln!("⚠️ [{source_path:?} -> {embedded_path:?}]: {errors:?}");
                                }
                                PreProcessError::StdIo(error) => {
                                    if error.kind() == std::io::ErrorKind::NotFound {
                                        eprintln!("⚠️ {source_path:?} file not found: {embedded_path:?}");
                                    } else {
                                        eprintln!("⚠️ [{source_path:?} -> {embedded_path:?}]: {error}");
                                    }
                                }
                            }
                            MacroIO::wrap(Node::empty())
                        })
                        .and_modify_context(|ctx| {
                            ctx.dependencies.insert(dependency);
                        })
                }
                MacroIO::wrap(Node::empty())
            })
    }
}
