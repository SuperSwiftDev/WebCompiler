pub mod pre;
pub mod post;
pub mod macros;
pub mod rewrites;

use std::path::PathBuf;
use macro_types::environment::{AccumulatedEffects, LexicalEnvironment, MacroIO, MacroRuntime, SourcePathResolver, SourceContext};
use macro_types::macro_tag::MacroTagSet;
use macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};
use macro_types::scope::BinderValue;
use macro_types::tag_rewrite_rule::TagRewriteRuleSet;
use xml_ast::Node;

pub use post::PostProcessor;
pub use pre::{PreProcessError, PreProcessor};

#[derive(Clone)]
pub struct GlobalPipelineSpec {
    pub macros: MacroTagSet,
    pub rules: TagRewriteRuleSet,
    pub project: ProjectContext,
    pub global_template: Option<PathBuf>,
}

/// Individual soruce file pipeline
#[derive(Clone)]
pub struct SourcePipeline {
    pub file_input: FileInput,
    pub pipeline_spec: GlobalPipelineSpec,
    pub local_template: Option<PathBuf>,
    pub all_input_rules: Vec<FileInput>,
    pub resolved_dependencies: ResolvedDependencies,
}

impl SourcePipeline {
    pub fn source_context(&self) -> SourceContext {
        SourceContext {
            project_context: &self.pipeline_spec.project,
            file_input: &self.file_input,
        }
    }
    pub fn macro_runtime(&self) -> MacroRuntime {
        MacroRuntime {
            project: &self.pipeline_spec.project,
            macros: &self.pipeline_spec.macros,
            rules: &self.pipeline_spec.rules,
            input: &self.file_input,
        }
    }
    pub fn source_file_input(&self) -> &FileInput {
        &self.file_input
    }
    pub fn execute(&mut self) {
        let result = self
            .execute_pre_process_phase()
            .map(|payload| {
                self.execute_post_process_phase(payload)
            })
            .map(|(finalized, effects)| {
                let _ = effects;
                self.emit_post_processed_file(&finalized)
            });
        match result {
            Ok(()) => (),
            Err(error) => {
                eprintln!("{}", error.to_string())
            }
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// PRE-PROCESSING PHASE
// ————————————————————————————————————————————————————————————————————————————

impl SourcePipeline {
    fn execute_pre_process_phase(&self) -> Result<MacroIO<Node>, PipelineError> {
        let runtime = self.macro_runtime();
        let pre_processor = PreProcessor::new(runtime);
        let content = {
            let mut env = LexicalEnvironment::default();
            match pre_processor.load_compile(&mut env) {
                Ok(x) => x,
                Err(error) => {
                    let source_path = self.source_file_input().source_file().to_path_buf();
                    return Err(PipelineError::PreProcessError { source_path, error })
                }
            }
        };
        let template_path = self.local_template
            .as_ref()
            .or_else(|| self.pipeline_spec.global_template.as_ref());
        let template_input = template_path.map(|path| FileInput {
            source: path.to_path_buf(),
            public: None,
        });
        if let Some(template_input) = template_input {
            let pre_processor = pre_processor.fork(&template_input);
            let finale = content.and_then(|content| {
                let mut env = LexicalEnvironment::default();
                env.binding_scope.insert("content", BinderValue::node(content.clone()));
                match pre_processor.load_compile(&mut env) {
                    Ok(x) => x,
                    Err(error) => {
                        let source_path = self.source_file_input().source_file();
                        crate::common::log::log_error(&error, Some(source_path), None);
                        MacroIO::wrap(content)
                    }
                }
            });
            return Ok(finale)
        }
        return Ok(content)
    }
    fn execute_post_process_phase(&mut self, processed: MacroIO<Node>) -> (Node, AccumulatedEffects) {
        let ( processed, effects ) = processed.collapse();
        let dependencies = effects.dependencies
            .clone()
            .into_iter()
            .chain(effects.deferred_dependencies.clone().into_iter())
            .collect::<Vec<_>>();
        // println!("deferred_dependencies: {:#?}", effects.deferred_dependencies);
        let path_resolver = SourcePathResolver {
            inputs: &self.all_input_rules,
            dependencies: &dependencies,
            host_context: self.source_context().to_owned(),
            project_context: &self.pipeline_spec.project,
        };
        let mut resolved_dependencies = ResolvedDependencies::default();
        let mut post_processor = PostProcessor::new(
            &self.pipeline_spec.rules,
            path_resolver,
            &mut resolved_dependencies
        );
        let finalized = post_processor.apply(processed);
        // println!("path_resolver: {path_resolver:#?}");
        self.resolved_dependencies.extend(resolved_dependencies);
        ( finalized, effects )
    }
    fn emit_post_processed_file(&mut self, node: &Node) {
        let html_string = node.format_document_pretty();
        // let html_string = node.format_document();
        let resolved_public_path = self.file_input.resolved_public_path(&self.pipeline_spec.project);
        let output_path = self.file_input.to_output_file_path(&self.pipeline_spec.project);
        crate::common::path_utils::write_output_file_smart(&output_path, html_string.as_bytes());
        self.resolved_dependencies.emitted_files.insert(resolved_public_path);
    }
}

#[derive(Debug)]
pub enum PipelineError {
    PreProcessError {
        source_path: PathBuf,
        error: PreProcessError,
    }
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreProcessError { source_path, error } => {
                let error_message = crate::common::log::format_error(&error, Some(source_path), None);
                write!(f, "{error_message}")
            }
        }
    }
}

impl std::error::Error for PipelineError {}



