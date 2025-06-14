use std::path::PathBuf;

use web_compiler_core::pipeline::{GlobalPipelineSpec, SourcePipeline};
use web_compiler_macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};

fn main() {
    // println!("Hello, world!");
    process();
}

fn process() {
    let project_root = "samples";
    let output_dir = "output";
    let source_path = "page.html";
    std::env::set_current_dir(project_root).unwrap();
    let project_context = ProjectContext {
        project_root: PathBuf::from(project_root),
        output_dir: PathBuf::from(output_dir),
    };
    let file_input = FileInput {
        source: PathBuf::from(source_path),
        public: None,
    };
    let global_pipeline_spec = GlobalPipelineSpec {
        macros: web_compiler_core::macros::standard_macro_tag_set(),
        rules: web_compiler_core::rewrite_rules::standard_tag_rewrite_rule_set(),
        project: project_context,
        global_template: None,
    };
    let mut pipeline = SourcePipeline {
        pipeline_spec: global_pipeline_spec,
        local_template: None,
        all_input_rules: vec![ file_input.clone() ],
        resolved_dependencies: ResolvedDependencies::default(),
        file_input,
    };
    pipeline.execute();
    println!("{:#?}", pipeline.resolved_dependencies);
}
