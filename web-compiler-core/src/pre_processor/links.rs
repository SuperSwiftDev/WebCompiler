use std::collections::HashMap;
use web_compiler_html_ast::TagBuf;
use web_compiler_common::EncodedVirtualPath;
use web_compiler_common::SourceContext;
use web_compiler_common::srcset::SrcsetCandidate;

use crate::common::constants::REQUIRES_REGULAR_DEPENDENCY_TRACKING;
use crate::common::constants::REQUIRES_SRC_SET_DEPENDENCY_TRACKING;
use crate::common::constants::TAG_MAY_REQUIRE_DEPENDENCY_TRACKING;
use super::AccumulatedEffects;

pub fn virtualize_local_attribute_paths(
    tag: &TagBuf,
    attributes: &mut HashMap<String, String>,
    state_context: &mut AccumulatedEffects,
    source_context: &SourceContext,
) {
    if !TAG_MAY_REQUIRE_DEPENDENCY_TRACKING.contains(tag.as_normalized()) {
        return 
    }
    // - -
    for (key, value) in attributes.iter_mut() {
        let key = key.to_lowercase();
        process_path_if_needed(&tag, &key, value, &source_context, state_context);
    }
}

fn process_path_if_needed(
    tag: &TagBuf,
    key: &str,
    value: &mut String,
    source_context: &SourceContext,
    state_context: &mut AccumulatedEffects,
) {
    // REGAULR
    if REQUIRES_REGULAR_DEPENDENCY_TRACKING.contains(&(tag.as_normalized(), &key)) {
        let virtual_src = to_encoded_virtual_path(&value, source_context);
        let dependency = source_context.new_relative_source_file_dependency(&value);
        state_context.insert_file_dependency(dependency);
        *value = virtual_src.encode();
    }
    // SPECIAL
    else if REQUIRES_SRC_SET_DEPENDENCY_TRACKING.contains(&(tag.as_normalized(), &key)) {
        let source_sets = SrcsetCandidate::parse_srcset(value)
            .into_iter()
            .map(|SrcsetCandidate { url, descriptor }| {
                let virtual_src = to_encoded_virtual_path(&url, source_context);
                let virtual_src = virtual_src.encode();
                let dependency = source_context.new_relative_source_file_dependency(&url);
                state_context.insert_file_dependency(dependency);
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

fn to_encoded_virtual_path(src_value: &str, source_context: &SourceContext) -> EncodedVirtualPath {
    EncodedVirtualPath {
        origin: {
            let path = source_context.source_file().to_path_buf();
            path.to_str().unwrap().to_string()
        },
        rel: src_value.to_string(),
    }
}

