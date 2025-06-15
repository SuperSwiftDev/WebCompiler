use std::path::PathBuf;

use web_compiler_core::pipeline::{GlobalPipelineSpec, SourcePipeline};
use web_compiler_macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};

fn main() {
    // println!("Hello, world!");
    process();
}

fn process() {
    let project_root = "demos/basic";
    std::env::set_current_dir(project_root).unwrap();
    let output_dir = "output";
    let template_path = "templates/global.html";
    let input_pattern = String::from("pages/**/*.html");
    let project_context = ProjectContext {
        project_root: PathBuf::from(project_root),
        output_dir: PathBuf::from(output_dir),
    };
    let sources = web_compiler_core::common::path_utils::resolve_file_path_paterns(&[input_pattern])
        .unwrap()
        .into_iter()
        .map(|input_path| {
            // let source_path = "pages/index.html";
            let file_input = FileInput {
                source: input_path,
                public: None,
            };
            let global_pipeline_spec = GlobalPipelineSpec {
                macros: web_compiler_core::macros::standard_macro_tag_set(),
                rules: web_compiler_core::rewrite_rules::standard_tag_rewrite_rule_set(),
                project: project_context.clone(),
                global_template: Some(PathBuf::from(template_path)),
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
        })
        .collect::<Vec<_>>();
}
