use macro_types::{environment::{AccumulatedEffects, PathResolver, SourceContext}, helpers::srcset::SrcsetCandidate, project::{DependencyRelation, ResolvedDependencies}};
use once_cell::sync::Lazy;
use xml_ast::{AttributeMap, TagBuf};
use std::collections::HashSet;

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
    resolver: PathResolver,
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
    resolver: PathResolver,
    resolved_dependencies: &mut ResolvedDependencies,
) {
    let _ = resolver;
    let _ = resolved_dependencies;
    let decoded_virtual_path = match DependencyRelation::decode(&href) {
        Some(x) => x,
        None => return
    };
    // - -
    // let resolved_dependency = ResolvedDependency {
    //     finalized: FileDependency {
    //         origin: resolved_origin,
    //         target: relative,
    //     },
    //     original: decoded_virtual_path,
    // };
    // resolved_dependencies.include_dependency(resolved_dependency);
    // - -
    *href = decoded_virtual_path.to;
}

// fn rewrite_path_mut(
//     href: &mut String,
//     resolver: &PathResolver,
//     resolved_dependencies: &mut ResolvedDependencies,
// ) {
//     let decoded_virtual_path = EncodedVirtualPath::decode(&href).unwrap();
//     let resolved_origin = resolver.host_context.input_rule.resolved_target_path(resolver.project_context());
//     let is_external = web_compiler_common::is_external_url(&decoded_virtual_path.rel);
//     *href = decoded_virtual_path.rel.clone();
//     if is_external {
//         return
//     }

//     let mut debug_parts = Vec::<String>::new();
//     let resolved = resolve_ref(&resolver, ResolveRefTask {
//         link: decoded_virtual_path.clone(),
//         resolved_origin: resolved_origin.clone(),
//     });
//     let resolved = match resolved {
//         Some(x) => x,
//         None => {
//             let debug_parts = debug_parts
//                 .into_iter()
//                 .filter(|x| !x.is_empty())
//                 .collect::<Vec<_>>()
//                 .join(" ");
//             eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts}");
//             *href = decoded_virtual_path.rel.clone();
//             return
//         }
//     };

//     // - DEBUG -
//     debug_parts.extend(vec![
//         format!("(resolved: {resolved:?})"),
//         format!("(from host: {:?})", resolved_origin),
//     ]);
//     let debug_parts = debug_parts
//         .into_iter()
//         .filter(|x| !x.is_empty())
//         .collect::<Vec<_>>()
//         .join(" ");

//     let relative = pathdiff::diff_paths(&resolved, resolved_origin.parent().unwrap()).unwrap();

//     // eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts} => {relative:?}");
    
    
//     // - -
//     // eprintln!("⚠️ TODO: resolve output path for target {href:?} {debug_parts}: {resolver:#?}");

//     // - FINALIZE -
//     // *href = decoded_virtual_path.rel.clone();
//     *href = relative.to_str().unwrap().to_string();

//     let resolved_dependency = ResolvedDependency {
//         finalized: FileDependency {
//             origin: resolved_origin,
//             target: relative,
//         },
//         original: decoded_virtual_path,
//     };

//     resolved_dependencies.include_dependency(resolved_dependency);
// }

// struct ResolveRefTask {
//     /// Figure out the resolved output that this points to.
//     pub link: EncodedVirtualPath,
//     /// Resolve the (resolved) link relative to this host location
//     pub resolved_origin: PathBuf,
// }
