//! Types for defining the overall compiler.
use std::collections::HashSet;

use macro_types::environment::{Featureset, SourceHostRef};
use macro_types::breadcrumbs::SiteTreeLayout;
use macro_types::project::{FileInput, ProjectContext, ResolvedDependencies};
use web_compiler_types::{CompilationMode, CompilerFeatureset, CompilerPipeline, CompilerRuntime};

use crate::markup::OutputWriterMode;


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
    let compilation_mode = compiler_pipeline.inputs.compilation_mode;
    let all_input_rules = compiler_pipeline.inputs.sources
        .iter()
        .map(|x| x.source.clone())
        .collect::<Vec<_>>();
    let site_tree_layout = SiteTreeLayout::compute(&all_input_rules, &compiler_pipeline.inputs.project);
    let resolved_dependencies = compiler_pipeline.inputs.sources
        .iter()
        .map(|input| {
            let global_pipeline_spec = crate::markup::GlobalPipelineSpec {
                compilation_mode: compiler_pipeline.inputs.compilation_mode,
                macros: compiler_pipeline.featureset.macros().to_owned(),
                rules: compiler_pipeline.featureset.rules().to_owned(),
                project: compiler_pipeline.inputs.project.clone(),
                global_template: compiler_pipeline.inputs.global_template.clone(),
            };
            let mut input_pipeline = crate::markup::SourcePipeline {
                file_input: input.source.clone(),
                pipeline_spec: global_pipeline_spec,
                local_template: input.local_template.clone(),
                all_input_rules: all_input_rules.clone(),
                resolved_dependencies: ResolvedDependencies::default(),
                site_tree_layout: site_tree_layout.clone(),
                output_writer_mode: OutputWriterMode::WriteFile,
            };
            let _ = input_pipeline.execute();
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
    let (css_files, mut remaining) = remaining
        .into_iter()
        .partition::<Vec<_>, _>(|x| {
            x.source.extension() == Some("css".as_ref())
        });
    // println!("css_files: {css_files:#?}");
    compile_css(&css_files, &compiler_pipeline.inputs.project, compilation_mode, &mut remaining);
    // println!("remaining: {remaining:#?}");
    emit_assets(&remaining, &compiler_pipeline.inputs.project, compilation_mode);
}

fn compile_css(css_files: &[FileInput], project_context: &ProjectContext, compilation_mode: CompilationMode, remaining: &mut Vec<FileInput>) {
    let _ = compilation_mode;
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
        // let environment = &();
        let css_postprocessor = css::CssPostprocessor::new(source_context);
        let ( pre_processed, effects ) = css_preprocessor.execute(&css_source).collapse();
        // let _ = effects; // TODO
        let assets = effects.dependencies
            .iter()
            .filter(|x| !x.is_external_target())
            .map(|x| x.as_file_dependency());
        for asset in assets {
            remaining.push(FileInput {
                source: asset.resolved_target_path(),
                public: None,
            });
        }
        let post_processed = css_postprocessor.execute(&pre_processed.value);
        let output_path = css_file.to_output_file_path(project_context);
        let is_modified = pre_processed.modified.union(post_processed.modified).is_modified();
        let write_or_symlink_output = crate::common::path_utils::WriteOrSymlinkOutput {
            output_file: output_path.as_path(),
            source_file: css_file.source_file(),
            contents: post_processed.value.as_bytes(),
        };
        let _ = is_modified; // TODO: MAYBE USE THIS
        let _ = write_or_symlink_output; // TODO: DISGARD THIS?
        crate::common::path_utils::write_output_file_smart(output_path.as_path(), post_processed.value.as_bytes());
        // if compilation_mode.is_production() {
        // } else if is_modified  {
        //     write_or_symlink_output.execute();
        // } else {
        //     write_or_symlink_output.write_symlink().unwrap_or_else(|_| {
        //         write_or_symlink_output.execute();
        //     });
        // }
    }
}

fn emit_assets(asset_files: &[FileInput], project_context: &ProjectContext, compilation_mode: CompilationMode) {
    for asset_file in asset_files {
        let source_file = asset_file.source_file();
        let output_path = asset_file.to_output_file_path(project_context);
        if compilation_mode.is_production() {
            let output_data = std::fs::read(source_file).unwrap();
            crate::common::path_utils::write_output_file_smart(&output_path, output_data);
        } else {
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
}


