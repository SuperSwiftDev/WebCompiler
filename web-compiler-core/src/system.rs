//! Types for defining the overall compiler.
use std::collections::HashSet;

use macro_types::environment::{Featureset, SourceHostRef};
use macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};
use web_compiler_types::{CompilerFeatureset, CompilerPipeline, CompilerRuntime};

pub fn web_publishing_compiler_featureset() -> CompilerFeatureset {
    CompilerFeatureset {
        macros: crate::markup::macros::standard_macro_tag_set(),
        rules: crate::markup::rewrites::standard_tag_rewrite_rule_set(),
    }
}

pub fn web_publishing_compiler_runtime(project: ProjectContext, source_file: FileInput) -> CompilerRuntime {
    CompilerRuntime {
        project,
        source_file,
        featureset: web_publishing_compiler_featureset(),
    }
}

pub fn execute_compiler_pipeline(compiler_pipeline: CompilerPipeline) {
    let resolved_dependencies = compiler_pipeline.inputs.sources
        .iter()
        .map(|input| {
            let global_pipeline_spec = crate::markup::GlobalPipelineSpec {
                macros: compiler_pipeline.featureset.macros().to_owned(),
                rules: compiler_pipeline.featureset.rules().to_owned(),
                project: compiler_pipeline.inputs.project.clone(),
                global_template: compiler_pipeline.inputs.global_template.clone(),
            };
            let all_input_rules = compiler_pipeline.inputs.sources
                .iter()
                .map(|x| x.source.clone())
                .collect::<Vec<_>>();
            let mut input_pipeline = crate::markup::SourcePipeline {
                file_input: input.source.clone(),
                pipeline_spec: global_pipeline_spec,
                local_template: input.local_template.clone(),
                all_input_rules,
                resolved_dependencies: ResolvedDependencies::default(),
            };
            input_pipeline.execute();
            input_pipeline.resolved_dependencies
        })
        .fold(ResolvedDependencies::default(), |mut acc, item| {
            acc.extend(item);
            acc
        });
    // - -
    // println!("resolved_dependencies: {resolved_dependencies:#?}");
    let remaining = resolved_dependencies.dependency_relations
        .iter()
        .filter(|dep| {
            let target = dep.finalized.resolved_target_path();
            let is_emitted = resolved_dependencies.emitted_files.contains(&target);
            let is_html_file = target.extension() == Some("html".as_ref());
            !is_emitted && !is_html_file
        })
        .map(|dep| {
            let source_path = dep.original.as_file_dependency().resolved_target_path();
            let public_path = dep.finalized.resolved_target_path();
            FileInput {
                source: source_path,
                public: Some(public_path),
            }
        })
        .map(|x| x.cleaned())
        .collect::<HashSet<_>>();
    // println!("remaining: {remaining:#?}");
    let (css_files, remaining) = remaining
        .into_iter()
        .partition::<Vec<_>, _>(|x| {
            x.source.extension() == Some("css".as_ref())
        });
    // println!("css_files: {css_files:#?}");
    compile_css(&css_files, &compiler_pipeline.inputs.project);
    // println!("remaining: {remaining:#?}");
    emit_assets(&remaining, &compiler_pipeline.inputs.project);
}

fn compile_css(css_files: &[FileInput], project_context: &ProjectContext) {
    // let mut resolved_dependencies = ResolvedDependencies::default();
    for css_file in css_files {
        let css_source = css_file.load_source_file();
        let css_source = match css_source {
            Ok(x) => x,
            Err(_) => {
                eprintln!("⚠️ missing css file {:?}", css_file.source_file());
                continue;
            }
        };
        let source_context = SourceHostRef {
            project_context: &project_context,
            file_input: css_file,
        };
        let css_preprocessor = css::CssPreprocessor::new(source_context);
        let environment = &();
        let css_postprocessor = css::CssPostprocessor::new(environment);
        let ( pre_processed, effects ) = css_preprocessor.execute(&css_source).collapse();
        let _ = effects; // TODO
        let post_processed = css_postprocessor.execute(&pre_processed.value);
        let output_path = css_file.to_output_file_path(project_context);
        let is_modified = pre_processed.modified.union(post_processed.modified).is_modified();
        let write_or_symlink_output = crate::common::path_utils::WriteOrSymlinkOutput {
            output_file: output_path.as_path(),
            source_file: css_file.source_file(),
            contents: post_processed.value.as_bytes(),
        };
        if is_modified {
            write_or_symlink_output.execute();
        } else {
            write_or_symlink_output.write_symlink().unwrap_or_else(|_| {
                write_or_symlink_output.execute();
            });
        }
    }
}

fn emit_assets(asset_files: &[FileInput], project_context: &ProjectContext) {
    for asset_file in asset_files {
        let source_file = asset_file.source_file();
        let output_path = asset_file.to_output_file_path(project_context);
        let status = crate::common::symlink::create_relative_symlink(source_file, &output_path);
        match status {
            Ok(status) => {
                if status.is_updated() {
                    println!("> linking {:?} ⬌ {:?}", source_file, output_path);
                }
            }
            Err(error) => {
                eprintln!("Failed to create symlink {:?} → {:?}: {error}", source_file, output_path);
            }
        }
    }
}


