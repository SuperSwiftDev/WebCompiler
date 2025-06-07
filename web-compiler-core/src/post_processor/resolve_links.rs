use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use web_compiler_common::is_external_url;
use web_compiler_common::EncodedVirtualPath;
use web_compiler_common::FileDependency;
use web_compiler_common::InputRule;
use web_compiler_common::ProjectContext;
use web_compiler_common::SourceContext;
use web_compiler_html_ast::TagBuf;
use web_compiler_common::PathResolver;
use web_compiler_common::srcset::SrcsetCandidate;
use crate::common::constants::REQUIRES_REGULAR_DEPENDENCY_TRACKING;
use crate::common::constants::REQUIRES_SRC_SET_DEPENDENCY_TRACKING;

pub fn resolve_virtual_path_attributes(
    tag: &TagBuf,
    attributes: &mut HashMap<String, String>,
    resolver: &PathResolver,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let tag = tag.as_normalized();
    for ( key, value ) in attributes.iter_mut() {
        let key = key.to_lowercase();
        if REQUIRES_REGULAR_DEPENDENCY_TRACKING.contains(&(tag, &key)) {
            rewrite_path_mut(value, resolver, resolved_dependencies);
        }
        else if REQUIRES_SRC_SET_DEPENDENCY_TRACKING.contains(&(tag, &key)) {
            let source_sets = SrcsetCandidate::parse_srcset(value)
                .into_iter()
                .map(|SrcsetCandidate { mut url, descriptor }| {
                    rewrite_path_mut(&mut url, resolver, resolved_dependencies);
                    SrcsetCandidate {
                        url,
                        descriptor: descriptor,
                    }
                })
                .collect::<Vec<_>>();
            let rewritten_source_sets = SrcsetCandidate::format_srcset(&source_sets);
            *value = rewritten_source_sets;
        }
    }
}

fn rewrite_path_mut(
    href: &mut String,
    resolver: &PathResolver,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let decoded_virtual_path = EncodedVirtualPath::decode(&href).unwrap();
    let resolved_origin = resolver.host_context.input_rule.resolved_target_path(resolver.project_context());
    let is_external = web_compiler_common::is_external_url(&decoded_virtual_path.rel);
    *href = decoded_virtual_path.rel.clone();
    if is_external {
        return
    }

    let mut debug_parts = Vec::<String>::new();
    let resolved = resolve_ref(&resolver, ResolveRefTask {
        link: decoded_virtual_path.clone(),
        resolved_origin: resolved_origin.clone(),
    });
    let resolved = match resolved {
        Some(x) => x,
        None => {
            let debug_parts = debug_parts
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts}");
            *href = decoded_virtual_path.rel.clone();
            return
        }
    };

    // - DEBUG -
    debug_parts.extend(vec![
        format!("(resolved: {resolved:?})"),
        format!("(from host: {:?})", resolved_origin),
    ]);
    let debug_parts = debug_parts
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let relative = pathdiff::diff_paths(&resolved, resolved_origin.parent().unwrap()).unwrap();

    // eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts} => {relative:?}");
    
    
    // - -
    // eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts}: {resolver:#?}");

    // - FINALIZE -
    // *href = decoded_virtual_path.rel.clone();
    *href = relative.to_str().unwrap().to_string();

    let resolved_dependency = ResolvedDependency {
        finalized: FileDependency {
            origin: resolved_origin,
            target: relative,
        },
        original: decoded_virtual_path,
    };

    resolved_dependencies.include_dependency(resolved_dependency);
}

struct ResolveRefTask {
    /// Figure out the resolved output that this points to.
    pub link: EncodedVirtualPath,
    /// Resolve the (resolved) link relative to this host location
    pub resolved_origin: PathBuf,
}

fn resolve_ref(
    resolver: &PathResolver,
    ResolveRefTask {
        link,
        resolved_origin,
    }: ResolveRefTask
) -> Option<PathBuf> {
    // - -
    if let Some(input_rule) = lookup_input_rule(resolver, &link) {
        let link_resolved = {
            input_rule.public
                .as_ref()
                .map(|x| x.to_path_buf())
                .unwrap_or_else(|| input_rule.source.clone())
        };
        Some(link_resolved)
    }
    // - -
    else if let Some(file_dependency) = lookup_file_dependency(resolver, &link) {
        let link_resolved = path_clean::clean(file_dependency.resolved_target());
        Some(link_resolved)
    }
    // - -
    else {
        None
    }
}

fn lookup_input_rule<'a, 'b>(resolver: &'a PathResolver, link: &'b EncodedVirtualPath) -> Option<&'a InputRule> {
    let link_origin = PathBuf::from(&link.origin);
    let link_origin_dir = link_origin.parent().unwrap();
    let link_rel = PathBuf::from(&link.rel);
    let link_resolved = path_clean::clean(link_origin_dir.join(&link_rel));

    resolver.inputs
        .iter()
        .find(|input| {
            path_clean::clean(&input.source) == link_resolved
        })
}

fn lookup_file_dependency<'a, 'b>(resolver: &'a PathResolver, link: &'b EncodedVirtualPath) -> Option<&'a FileDependency> {
    let link_resolved = path_clean::clean(link.resolved_target_path());

    resolver.dependencies
        .iter()
        .find(|input| {
            path_clean::clean(input.resolved_target()) == link_resolved
        })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedDependency {
    pub finalized: FileDependency,
    pub original: EncodedVirtualPath,
}

impl ResolvedDependency {
    
}

#[derive(Debug, Clone, Default)]
pub struct ResolvedDependencies {
    pub dependencies: HashSet<ResolvedDependency>,
    pub emitted_files: HashSet<PathBuf>,
}

impl ResolvedDependencies {
    pub fn extend(&mut self, other: Self) {
        self.dependencies.extend(other.dependencies);
        self.emitted_files.extend(other.emitted_files);
    }
    pub fn include_dependency(&mut self, dependency: ResolvedDependency) {
        self.dependencies.insert(dependency);
    }
    pub fn include_emitted_file(&mut self, resolved_target: impl Into<PathBuf>) {
        let _ = self.emitted_files.insert(resolved_target.into());
    }
}
