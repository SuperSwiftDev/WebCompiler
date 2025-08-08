use macro_types::{macro_tag::MacroTag, scope::JsonBinderValue};
use macro_types::lexical_env::MacroIO;
use macro_types::scope::BinderValue;
use xml_ast::{Fragment, Node};

use web_compiler_types::CompilerRuntime;

use super::super::pre::PreProcessor;

// use crate::pre_processor::PreProcessor;

#[derive(Debug, Clone, Copy, Default)]
pub struct EnumerateMacroTag;

impl MacroTag for EnumerateMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "enumerate" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        runtime: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        fn resolved_node_binder_pipeline(
            ident: impl AsRef<str>,
            binder: Node,
            body: &xml_ast::Fragment,
            scope: &mut macro_types::lexical_env::ProcessScope,
            runtime: &CompilerRuntime,
        ) -> MacroIO<xml_ast::Node> {
            let fragment = binder
                .flatten()
                .into_iter()
                .filter_map(|x| x.to_element())
                .map(|item| {
                    let mut sub_scope = scope.clone();
                    // let runtime = runtime.fork(&new_input_file);
                    let pre_processor = PreProcessor::new(runtime.clone());
                    // println!("{item:?}");
                    sub_scope.binding_scope.insert(ident.as_ref(), BinderValue::node(item));
                    xml_ast::transform::apply_effectful_markup_transformer_node_vec(
                        body.clone().to_vec(),
                        &pre_processor,
                        &mut sub_scope,
                    )
                })
                .collect::<Vec<_>>();
            // - -
            MacroIO::flatten_vec_deep(fragment)
                .map(Fragment::from_nodes)
                .map(Node::Fragment)
        }
        fn resolved_json_array_binder_pipeline(
            ident: impl AsRef<str>,
            binder: &JsonBinderValue,
            body: xml_ast::Fragment,
            scope: &mut macro_types::lexical_env::ProcessScope,
            runtime: &CompilerRuntime,
        ) -> MacroIO<xml_ast::Node> {
            let binder = match binder {
                JsonBinderValue::Array(xs) => xs,
                _ => {
                    runtime.with_source_file_path(|file| {
                        eprintln!("⚠️ {file:?} <enumerate> failed to resolve binding as an array");
                    });
                    return MacroIO::wrap(Node::empty())
                }
            };
            let fragment = binder
                .into_iter()
                .map(|item| {
                    let mut sub_scope = scope.clone();
                    // let runtime = runtime.fork(&new_input_file);
                    let pre_processor = PreProcessor::new(runtime.clone());
                    // println!("{item:?}");
                    sub_scope.binding_scope.insert(ident.as_ref(), BinderValue::Json(item.to_owned()));
                    xml_ast::transform::apply_effectful_markup_transformer_node_vec(
                        body.clone().to_vec(),
                        &pre_processor,
                        &mut sub_scope,
                    )
                })
                .collect::<Vec<_>>();
            // - -
            MacroIO::flatten_vec_deep(fragment)
                .map(Fragment::from_nodes)
                .map(Node::Fragment)
        }
        // let children = children.to_vec();
        let binder_key = attributes.get("path");
        let binder_value = binder_key
            .and_then(|target| {
                let result = scope.binding_scope.lookup(target.as_str());
                match result {
                    Some(x) => Some(x.to_owned()),
                    None => {
                        runtime.with_source_file_path(|file| {
                            eprintln!("⚠️ {file:?} <enumerate> failed to resolve binding for {target:?}\n\t{:?}", scope.binding_scope);
                        });
                        None
                    }
                }
            });
        let as_ident = match attributes.get("as") {
            Some(x) => x,
            None => {
                runtime.with_source_file_path(|file| {
                    eprintln!("⚠️ {file:?} <enumerate> failed to resolve binding identifier");
                });
                return MacroIO::wrap(Node::empty())
            }
        };
        match binder_value {
            Some(BinderValue::Markup(binder)) => {
                // eprintln!(" » MARKUP {attributes:?}");
                resolved_node_binder_pipeline(as_ident.as_str(), binder.0, &children, scope, runtime)
            }
            Some(BinderValue::Json(binder)) => {
                // eprintln!(" » JSON {attributes:?} | {binder:?}");
                resolved_json_array_binder_pipeline(as_ident.as_str(), &binder, children, scope, runtime)
            }
            None => {
                // eprintln!(" » NOPE {attributes:?}");
                runtime.with_source_file_path(|file| {
                    if let Some(_) = binder_key {
                        // ASSUME ALREADY LOGGED
                    } else {
                        eprintln!("⚠️ {file:?} <enumerate> failed to resolve binding value");
                    }
                });
                MacroIO::wrap(Node::empty())
            }
        }
    }
}
