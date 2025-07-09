use std::path::{Path, PathBuf};

pub mod dsl;

use macro_types::{macro_tag::MacroTagSet, project::{FileInput, ProjectContext, ResolvedDependencies}, tag_rewrite_rule::TagRewriteRuleSet};
use web_compiler_types::{CompilationMode, CompilerRuntime};
use xml_ai_core::runtime::CompletedPrompt;

use crate::markup::OutputWriterMode;

#[derive(Debug, Clone)]
pub struct XmlAiDocument(xml_ai_core::frontend_ast::Document);

impl XmlAiDocument {
    pub fn load(file_path: impl AsRef<Path>) -> Self {
        let node = pre_process(file_path).unwrap();
        let document = xml_ai_core::frontend_ast::Document::from_node(node).unwrap();
        Self(document)
    }
    pub async fn run_prompt(&self, name: impl AsRef<str>) -> CompletedPrompt {
        self.0.evaluate(name).await
    }
}

fn macros() -> MacroTagSet<CompilerRuntime> {
    MacroTagSet::from_vec(vec![])
}

fn rules() -> TagRewriteRuleSet<CompilerRuntime> {
    TagRewriteRuleSet::from_vec(vec![])
}

fn pre_process(file_path: impl AsRef<Path>) -> Result<xml_ast::Node, ()> {
    let project_context = ProjectContext {
        project_root: file_path.as_ref().parent().unwrap().to_path_buf(),
        output_dir: PathBuf::from(".web-compiler/xml-ai-work")
    };
    let global_pipeline_spec = crate::markup::GlobalPipelineSpec {
        compilation_mode: CompilationMode::Dev,
        macros: macros(),
        rules: rules(),
        project: project_context.clone(),
        global_template: None,
    };
    let file_input = FileInput {
        source: file_path.as_ref().to_path_buf(),
        public: None,
    };
    let mut input_pipeline = crate::markup::SourcePipeline {
        file_input: file_input.clone(),
        pipeline_spec: global_pipeline_spec,
        local_template: None,
        all_input_rules: vec![ file_input ],
        resolved_dependencies: ResolvedDependencies::default(),
        site_tree_layout: Default::default(),
        output_writer_mode: OutputWriterMode::JustReturnNode,
    };
    input_pipeline.execute()
}

