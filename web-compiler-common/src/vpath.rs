//! Virtual path encoding for deferring relative path resolution.
//!
//! [`EncodedVirtualPath`] encodes a string of the form:
//!
//! ```text
//! @virtual:origin/path.html|./rel/path.css
//! ```
//!
//! - `origin`: relative path to the source file (from `project_root`)
//! - `rel`: the original relative reference as written (e.g. `./img/logo.png`)
//!
//! This encoding allows the build system to:
//!
//! - Tag unresolved paths uniformly across HTML, CSS, JS, etc.
//! - Defer resolution until the final output location of each file is known
//! - Prevent incorrect rewrites due to premature normalization
//!
//! Use [`EncodedVirtualPath::encode`] and [`EncodedVirtualPath::decode`] to round-trip.

use std::path::PathBuf;

use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncodedVirtualPath {
    pub origin: String,
    pub rel: String,
}

impl EncodedVirtualPath {
    pub fn encode(&self) -> String {
        format!(
            "@virtual:{}|{}",
            utf8_percent_encode(&self.origin, NON_ALPHANUMERIC),
            utf8_percent_encode(&self.rel, NON_ALPHANUMERIC)
        )
    }

    pub fn decode(input: &str) -> Option<Self> {
        input.strip_prefix("@virtual:").and_then(|rest| {
            let mut parts = rest.splitn(2, '|');
            let origin = percent_decode_str(parts.next()?).decode_utf8().ok()?.to_string();
            let rel = percent_decode_str(parts.next()?).decode_utf8().ok()?.to_string();
            Some(Self { origin, rel })
        })
    }

    pub fn is_virtual(input: &str) -> bool {
        input.starts_with("@virtual:")
    }

    pub fn resolved_target_path(&self) -> PathBuf {
        // PathBuf::from(&self.origin).join(PathBuf::from(&self.rel))
        let link_origin = PathBuf::from(path_clean::clean(&self.origin));
        let link_origin_dir = link_origin.parent().unwrap();
        let link_rel = PathBuf::from(&self.rel);
        let link_resolved = path_clean::clean(link_origin_dir.join(&link_rel));
        link_resolved
    }
}

// impl std::fmt::Display for EncodedVirtualPath {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "@virtual:{}|{}", self.origin, self.rel)
//     }
// }
