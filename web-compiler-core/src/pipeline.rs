#![allow(unused)]
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use web_compiler_html_ast::Html;
use web_compiler_html_ast::ParserMode;
use web_compiler_html_ast::transform::EffectPropagator;
use crate::post_processor::ResolvedDependencies;
use crate::pre_processor::AccumulatedEffects;
use crate::pre_processor::ScopeBindingEnv;
use web_compiler_common::FileDependency;
use web_compiler_common::InputRule;
use web_compiler_common::PathResolver;
use web_compiler_common::ProjectContext;
use web_compiler_common::SourceContext;


const VERBOSE_DEBUG_MODE: bool = true;

#[derive(Debug, Clone)]
pub struct CompilerSettings {
    pub pretty_format_output: bool,
    pub project_context: ProjectContext,
}

#[derive(Debug, Clone)]
pub struct Compiler {
    pub settings: CompilerSettings,
    pub sources: CompilerSources,
}

#[derive(Debug, Clone)]
pub struct CompilerSources {
    pub template_path: Option<PathBuf>,
    pub input_rules: Vec<InputRule>,
    pub bundle_rules: Vec<BundleRule>,
}

#[derive(Debug, Clone)]
pub struct BundleRule {
    pub location: PathBuf,
}

impl Compiler {
    pub fn execute(self) {
        compile_project(self);
    }
}

fn compile_project(
    Compiler { settings, sources }: Compiler
) {
    let template_path = sources.template_path.as_ref().map(|x| x.as_path());
    let template = sources.template_path
        .as_ref()
        .map(|template_path| {
            // let pre_processor = macro_processor::pre_processor::PreProcessor::new(source_context)
        });
    let mut resolved_dependencies = sources.input_rules
        .iter()
        .filter_map(|input| {
            run_pre_processor(&settings, input, template_path)
        })
        .map(|pre_processed| {
            run_post_processor(&settings, &sources.input_rules, pre_processed)
        })
        .map(|post_processed| {
            emit_post_processed_results(&settings, post_processed)
        })
        // .collect::<Vec<_>>();
        .fold(ResolvedDependencies::default(), |mut acc, item| {
            acc.extend(item.resolved_dependencies);
            acc
        });
    
    // eprintln!("HERE: {processed_sources:#?}");
    emit_assets(&settings, &mut resolved_dependencies)
}

#[derive(Debug, Clone)]
struct PreProcessedPayload {
    html: Html,
    effects: AccumulatedEffects,
    input_rule: InputRule,
}

#[derive(Debug, Clone)]
struct PostProcessedPayload {
    html: Html,
    input_rule: InputRule,
    resolved_dependencies: ResolvedDependencies,
}

#[derive(Debug, Clone)]
struct ProcessedSourceArtifact {
    resolved_dependencies: ResolvedDependencies,
}

fn run_pre_processor(
    settings: &CompilerSettings,
    input: &InputRule,
    template_path: Option<&Path>,
) -> Option<PreProcessedPayload> {
    let source_context = settings.project_context.new_source_context(input);
    let source_path = source_context.source_file().to_path_buf();
    let pre_processor = crate::pre_processor::PreProcessor::new(source_context);
    let pre_processed = pre_processor.compile(ParserMode::fragment("div"));
    let (pre_processed, source_effects) = match pre_processed {
        Ok(x) => x,
        Err(error) => {
            log_error(error.as_ref(), Some(source_path.as_path()), None);
            return None
        }
    };
    let with_template = input.template
        .as_ref()
        .map(|x| x.as_path())
        .or_else(|| template_path)
        .and_then(|template_path| {
            let parser_mode = ParserMode::Document;
            let result = pre_processor.process_template_context(
                template_path,
                parser_mode,
                pre_processed.clone(),
            );
            let result = match result {
                Ok(x) => x,
                Err(error) => {
                    log_error(error.as_ref(), Some(template_path), Some(source_path.as_path()));
                    return None
                }
            };
            Some(result)
        });
    // let with_sub_template
    if let Some((result_html, template_effects)) = with_template {
        let total_effects = AccumulatedEffects::union(source_effects, template_effects);
        return Some(PreProcessedPayload {
            html: result_html,
            effects: total_effects,
            input_rule: input.clone(),
        })
    }
    Some(PreProcessedPayload {
        html: pre_processed,
        effects: source_effects,
        input_rule: input.clone(),
    })
}

fn run_post_processor(
    settings: &CompilerSettings,
    all_input_rules: &[InputRule],
    PreProcessedPayload {
        html: pre_html,
        effects,
        input_rule,
    }: PreProcessedPayload
) -> PostProcessedPayload {
    let dependencies = effects.file_dependencies.clone();
    let path_resolver = PathResolver {
        inputs: all_input_rules.to_vec(),
        dependencies: dependencies
            .clone()
            .into_iter()
            .collect::<Vec<_>>(),
        host_context: SourceContext {
            project_context: settings.project_context.clone(),
            input_rule: input_rule.clone(),
        },
    };
    let (result, resolved_dependencies) = crate::post_processor::execute(pre_html, path_resolver);
    PostProcessedPayload {
        html: result,
        input_rule: input_rule,
        resolved_dependencies: resolved_dependencies,
    }
}

fn emit_post_processed_results(
    settings: &CompilerSettings,
    PostProcessedPayload {
        html,
        input_rule,
        mut resolved_dependencies,
    }: PostProcessedPayload
) -> ProcessedSourceArtifact {
    let html_string = html.render_html_string(
        settings.pretty_format_output,
        true
    );
    let resolved_path = input_rule.resolved_target_path(&settings.project_context);
    let output_path = input_rule.output_file_path(&settings.project_context);
    // println!("> compiling {output_path:?}: {resolved_dependencies:#?}");
    println!("> compiling {output_path:?}");
    write_output_file_smart(&output_path, &html_string);
    resolved_dependencies.include_emitted_file(resolved_path);
    ProcessedSourceArtifact {
        resolved_dependencies,
    }
}

fn write_output_file_smart(file_path: impl AsRef<Path>, contents: &str) {
    let file_path = file_path.as_ref();
    let new_bytes = contents.as_bytes();

    let file_needs_write = match std::fs::read(file_path) {
        Ok(existing_bytes) => {
            existing_bytes != new_bytes
        },
        Err(_) => true, // file doesn't exist or can't be read
    };

    if file_needs_write {
        if let Some(parent_dir) = file_path.parent() {
            std::fs::create_dir_all(parent_dir).unwrap();
        }
        println!("> writing {file_path:?}");
        std::fs::write(file_path, new_bytes).unwrap();
    }
}

pub fn log_error(error: &dyn std::error::Error, file_path: Option<&Path>, original: Option<&Path>) {
    use std::io::Write;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();

    let mut parts = vec![format!("{}", error)];
    let mut source = error.source();

    while let Some(err) = source {
        parts.push(format!("{}", err));
        source = err.source();
    }

    let error_chain = parts.join(" : ");

    let cwd = std::env::current_dir().unwrap();
    let leading = if VERBOSE_DEBUG_MODE {
        format!("[{}] ", cwd.display())
    } else {
        String::default()
    };

    match (file_path, original) {
        (Some(file_path), Some(original)) => {
            let file_path = file_path.display();
            let original = original.display();
            let _ = writeln!(handle, "{leading}Error while processing '{file_path}' ({original}): {error_chain}");
        }
        (Some(file_path), None) => {
            let file_path = file_path.display();
            let _ = writeln!(handle, "{leading}Error while processing '{file_path}': {error_chain}");
        }
        (None, Some(original)) => {
            let original = original.display();
            let _ = writeln!(handle, "{leading}{original} Error: {error_chain}");
        }
        (None, None) => {
            let _ = writeln!(handle, "{leading}Error: {error_chain}");
        }
    }
}

fn emit_assets(
    settings: &CompilerSettings,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let remaining_set = resolved_dependencies
        .dependencies
        .clone()
        .into_iter()
        .filter(|dep| {
            let resolved_link = dep.finalized.resolved_target();
            let is_emitted = resolved_dependencies.emitted_files.contains(&resolved_link);
            let is_html_file = resolved_link.extension() == Some("html".as_ref());
            !is_emitted && !is_html_file
        })
        .map(|dep| {
            Asset {
                source: path_clean::clean(dep.original.resolved_target_path()),
                target: path_clean::clean(dep.finalized.resolved_target()),
            }
        })
        .collect::<HashSet<_>>();
    if VERBOSE_DEBUG_MODE {
        let remaining_set = remaining_set
            .clone()
            .into_iter()
            .map(|dep| {
                dep
            })
            .collect::<HashSet<_>>();
        let mut remaining_debug = remaining_set
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        remaining_debug.sort_by(|left, right| {
            left.source.cmp(&right.source)
        });
        println!("REMAINING: {remaining_set:#?}");
    }
    for asset in remaining_set {
        let output_dir = settings.project_context.output_dir.as_path();
        let asset_input_path = asset.source.as_path();
        let asset_output_path = output_dir.join(&asset.target);
        std::fs::create_dir_all(asset.target.parent().unwrap()).unwrap();
        let linked_result = web_compiler_common::symlink::create_relative_symlink(
            &asset_input_path,
            &asset_output_path
        );
        match linked_result {
            Ok(()) => (),
            Err(error) => {
                // if let Some(error) = error.downcast_ref::<std::io::Error>() {}
                eprintln!("Failed to create symlink {:?} → {:?}: {error}", asset_input_path, asset_output_path);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Asset {
    pub source: PathBuf,
    pub target: PathBuf,
}
