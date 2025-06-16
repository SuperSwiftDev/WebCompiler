use macro_types::scope::{BinderValue, JsonBinderValue, MarkupBinderValue};
use once_cell::sync::Lazy;
use xml_ast::{AttributeMap, AttributeValueBuf, Node, TagBuf};
use std::collections::HashSet;
use std::path::PathBuf;

use macro_types::environment::{AccumulatedEffects, MacroRuntime, SourceContext, SourcePathResolver};
use macro_types::helpers::srcset::SrcsetCandidate;
use macro_types::project::{DependencyRelation, FileDependency, ResolvedDependencies, ResolvedDependency};

// ————————————————————————————————————————————————————————————————————————————
// CONSTANTS
// ————————————————————————————————————————————————————————————————————————————

pub static REQUIRES_REGULAR_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("a", "href"),
        ("area", "href"),
        ("link", "href"),
        ("img", "src"),
        ("video", "src"),
        ("video", "poster"),
        ("source", "src"),
        ("script", "src"),
        ("iframe", "src"),
        ("audio", "src"),
        ("track", "src"),
        ("embed", "src"),
        ("object", "data"),
        ("form", "action"),
        ("input", "formaction"),
        ("button", "formaction"),
        ("use", "href"),
        ("use", "xlink:href"),
        ("image", "href"),
        ("image", "xlink:href"),
    ])
});

pub static REQUIRES_SRC_SET_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("img", "srcset"),
        ("source", "srcset"),
    ])
});

pub static REQUIRES_DYNAMIC_SITE_LINK_DEPENDENCY_TRACKING: Lazy<HashSet<(&'static str, &'static str)>> = Lazy::new(|| {
    HashSet::from([
        ("a", "href"),
    ])
});

pub static TAG_MAY_REQUIRE_DEPENDENCY_TRACKING: Lazy<HashSet<&'static str>> = {
    fn tags_only() -> HashSet<&'static str> {
        let xs = REQUIRES_REGULAR_DEPENDENCY_TRACKING
            .iter()
            .chain(REQUIRES_SRC_SET_DEPENDENCY_TRACKING.iter())
            .map(|(x, _)| *x);
        let result: HashSet<&'static str> = HashSet::from_iter(xs);
        result
    }
    Lazy::new(|| { tags_only() })
};


// ————————————————————————————————————————————————————————————————————————————
// PRE-PROCESS LOGIC
// ————————————————————————————————————————————————————————————————————————————

pub fn virtualize_attribute_paths(
    tag: &TagBuf,
    attributes: &mut AttributeMap,
    effects: &mut AccumulatedEffects,
    source_context: SourceContext,
) {
    if !TAG_MAY_REQUIRE_DEPENDENCY_TRACKING.contains(tag.as_normalized()) {
        return 
    }
    // - -
    for (key, value) in attributes.iter_mut() {
        let key = key.as_str().to_ascii_lowercase();
        process_path_if_needed(&tag, &key, value.as_mut_string(), source_context, effects);
    }
}

fn process_path_if_needed(
    tag: &TagBuf,
    key: &str,
    value: &mut String,
    source_context: SourceContext,
    effects: &mut AccumulatedEffects,
) {
    // REGAULR
    if REQUIRES_REGULAR_DEPENDENCY_TRACKING.contains(&(tag.as_normalized(), &key)) {
        let dependency = source_context.file_input().with_dependency_relation(&value);
        let virtual_src = dependency.encode();
        effects.dependencies.insert(dependency);
        *value = virtual_src;
    }
    // SPECIAL
    else if REQUIRES_SRC_SET_DEPENDENCY_TRACKING.contains(&(tag.as_normalized(), &key)) {
        let source_sets = SrcsetCandidate::parse_srcset(value)
            .into_iter()
            .map(|SrcsetCandidate { url, descriptor }| {
                let dependency = source_context.file_input().with_dependency_relation(&url);
                let virtual_src = dependency.encode();
                effects.dependencies.insert(dependency);
                SrcsetCandidate {
                    url: virtual_src,
                    descriptor,
                }
            })
            .collect::<Vec<_>>();
        let rewritten_source_sets = SrcsetCandidate::format_srcset(&source_sets);
        *value = rewritten_source_sets;
    }
}


// ————————————————————————————————————————————————————————————————————————————
// POST-PROCESS LOGIC
// ————————————————————————————————————————————————————————————————————————————

pub fn resolve_virtual_path_attributes(
    tag: &TagBuf,
    attributes: &mut AttributeMap,
    resolver: SourcePathResolver,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let tag = tag.as_normalized();
    for ( key, value ) in attributes.iter_mut() {
        let key = key.as_str().to_ascii_lowercase();
        if REQUIRES_REGULAR_DEPENDENCY_TRACKING.contains(&(tag, &key)) {
            rewrite_path_mut(value.as_mut_string(), resolver, resolved_dependencies);
        }
        else if REQUIRES_SRC_SET_DEPENDENCY_TRACKING.contains(&(tag, &key)) {
            let source_sets = SrcsetCandidate::parse_srcset(value.as_str())
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
            *value.as_mut_string() = rewritten_source_sets;
        }
    }
}

fn rewrite_path_mut(
    href: &mut String,
    resolver: SourcePathResolver,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let _ = resolver;
    let _ = resolved_dependencies;
    let decoded_virtual_path = match DependencyRelation::decode(&href) {
        Some(x) => x,
        None => return
    };
    // - -
    if decoded_virtual_path.is_external_target() {
        *href = decoded_virtual_path.to;
        return
    }
    // - -
    let resolved = resolve_dependency_relation(&resolver, &decoded_virtual_path);
    let resolved = match resolved {
        Some(x) => x,
        None => {
            eprintln!("⚠️ TODO: resolve output path for target {href:?}");
            *href = decoded_virtual_path.to;
            return
        }
    };
    // - -
    let resolved_origin = resolver.host_context.file_input().resolved_public_path(resolver.project_context);
    let relative = pathdiff::diff_paths(&resolved, resolved_origin.parent().unwrap()).unwrap();
    // println!("{resolved:?} <~> {:?} => {relative:?}", resolved_origin.parent());
    // - -
    *href = relative.to_str().unwrap().to_string();
    // - -
    let resolved_dependency = ResolvedDependency {
        finalized: FileDependency {
            from: resolved_origin,
            to: relative,
        },
        original: decoded_virtual_path,
    };
    // let resolved_dependency = resolved_dependency.cleaned();
    resolved_dependencies.include_dependency(resolved_dependency);
}

fn resolve_dependency_relation(
    resolver: &SourcePathResolver,
    dependency: &DependencyRelation,
) -> Option<PathBuf> {
    // - -
    if let Some(input_rule) = resolver.lookup_input_rule(dependency) {
        let link_resolved = {
            input_rule.public
                .as_ref()
                .map(|x| x.to_path_buf())
                .unwrap_or_else(|| input_rule.source.clone())
        };
        Some(link_resolved)
    }
    // - -
    else if let Some(relation) = resolver.lookup_dependency(dependency) {
        let dependency = relation.as_file_dependency();
        let link_resolved = path_clean::clean(dependency.resolved_target_path());
        Some(link_resolved)
    }
    // - -
    else {
        None
    }
}

// ————————————————————————————————————————————————————————————————————————————
// RESOLVE ATTRIBUTE PATH EXPRESSIONS
// ————————————————————————————————————————————————————————————————————————————

pub fn resolve_attribute_path_expressions(
    attributes: &mut AttributeMap,
    scope: &mut macro_types::environment::LexicalEnvironment,
    runtime: &MacroRuntime,
) {
    attributes.map_mut(|_, value| {
        let rewrite = value
            .as_str()
            .trim()
            .strip_prefix("{{")
            .and_then(|value| {
                value.strip_suffix("}}")
            })
            .and_then(|target| {
                let result = scope.binding_scope.lookup(target);
                if result.is_none() {
                    let source_file = runtime.source_context();
                    let source_file = source_file.file_input().source_file();
                    eprintln!("⚠️ {source_file:?} failed to resolve binding `{target}`");
                }
                Some((target, result?))
            })
            .and_then(|(target, value)| {
                let result = value.try_cast_to_string();
                if result.is_none() {
                    let source_file = runtime.source_context();
                    let source_file = source_file.file_input().source_file();
                    eprintln!("⚠️ {source_file:?} failed to resolve binding `{target}` as string");
                }
                result
            });
        if let Some(rewrite) = rewrite {
            *value = AttributeValueBuf::literal(rewrite);
        }
    });
}
