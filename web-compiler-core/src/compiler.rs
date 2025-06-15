use std::path::PathBuf;

use macro_types::macro_tag::MacroTagSet;
use macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};
use macro_types::tag_rewrite_rule::TagRewriteRuleSet;

pub struct CompilerInput {
    pub source: FileInput,
    /// Will override the global template.
    pub local_template: Option<PathBuf>,
}

pub struct CompilerInputs {
    pub global_template: Option<PathBuf>,
    pub sources: Vec<CompilerInput>,
    pub project: ProjectContext,
}

pub struct CompilerSpec {
    pub macros: MacroTagSet,
    pub rules: TagRewriteRuleSet,
}

impl Default for CompilerSpec {
    fn default() -> Self {
        CompilerSpec {
            macros: crate::macros::standard_macro_tag_set(),
            rules: crate::rewrite_rules::standard_tag_rewrite_rule_set(),
        }
    }
}

pub struct CompilerPipeline {
    pub spec: CompilerSpec,
    pub inputs: CompilerInputs,
}

impl CompilerPipeline {
    pub fn execute(&self) {
        let _ = self.inputs.sources
            .iter()
            .map(|input| {
                let global_pipeline_spec = crate::pipeline::GlobalPipelineSpec {
                    macros: self.spec.macros.clone(),
                    rules: self.spec.rules.clone(),
                    project: self.inputs.project.clone(),
                    global_template: self.inputs.global_template.clone(),
                };
                let all_input_rules = self.inputs.sources
                    .iter()
                    .map(|x| x.source.clone())
                    .collect::<Vec<_>>();
                let mut input_pipeline = crate::pipeline::SourcePipeline {
                    file_input: input.source.clone(),
                    pipeline_spec: global_pipeline_spec,
                    local_template: input.local_template.clone(),
                    all_input_rules,
                    resolved_dependencies: ResolvedDependencies::default(),
                };
                input_pipeline.execute();
                input_pipeline.resolved_dependencies
            })
            .collect::<Vec<_>>();
    }
}
