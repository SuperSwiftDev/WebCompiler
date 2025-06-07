//! Spec-compliant parser and formatter for HTML `srcset` attributes.
//!
//! This module parses and formats the `srcset` attribute found in `<img>` and `<source>`
//! tags in HTML. It follows the [WHATWG HTML Standard](https://html.spec.whatwg.org/multipage/images.html#parse-a-srcset-attribute)
//! to robustly handle edge cases such as:
//!
//! - Quoted and unquoted URLs
//! - Pixel density descriptors (`1x`, `2x`, etc.)
//! - Width descriptors (`400w`, `800w`, etc.)
//! - Leading/trailing whitespace, malformed entries, and multiple commas
//!
//! ## Virtual Path Support
//!
//! In addition to standard URLs, this parser supports a custom **virtual path encoding**
//! format used by the static site compiler:
//!
//! ```text
//! @virtual:origin/file.html|./relative/asset.png
//! ```
//!
//! - `@virtual:` identifies the reference as unresolved
//! - `origin/file.html` is the source file (relative to project root)
//! - `./relative/asset.png` is the raw relative reference
//!
//! These references are used during initial parsing and are **resolved later** by a
//! [`PathResolver`](crate::PathResolver) once output layout is known.
//!
//! This deferred resolution ensures correct relative linking even when files are moved
//! or aliased in the output directory structure.
//!
//! Use [`EncodedVirtualPath`](crate::EncodedVirtualPath) to decode or resolve such paths.

/// Represents a single entry in an HTML `srcset` attribute.
///
/// Each `SrcsetCandidate` consists of:
/// - A `url` (possibly a virtual path starting with `@virtual:`)
/// - An optional `descriptor` like `1x`, `2x`, or `800w`
///
/// This struct is used to represent image resources for responsive loading.
///
/// Virtual paths are preserved in their encoded form and can be resolved
/// post-parse using a [`PathResolver`](crate::PathResolver).
#[derive(Debug, Clone, PartialEq)]
pub struct SrcsetCandidate {
    pub url: String,
    pub descriptor: Option<String>,
}

impl SrcsetCandidate {
    /// Spec-compliant srcset parser based on WHATWG HTML Standard.
    /// 
    /// Parses a `srcset` string into a list of [`SrcsetCandidate`]s.
    ///
    /// Follows the [WHATWG spec][spec] and supports both standard URLs and
    /// custom virtual paths of the form:
    ///
    /// ```text
    /// @virtual:origin/page.html|./img/logo.png 2x
    /// ```
    ///
    /// [spec]: https://html.spec.whatwg.org/multipage/images.html#parse-a-srcset-attribute
    ///
    /// # Arguments
    ///
    /// * `input` - A raw string from the `srcset` attribute.
    ///
    /// # Returns
    ///
    /// A `Vec<SrcsetCandidate>` with one entry per comma-separated image candidate.
    ///
    /// # Notes
    ///
    /// Virtual paths (`@virtual:`) are not resolved in this function. Use
    /// [`EncodedVirtualPath::decode`](crate::EncodedVirtualPath::decode)
    /// followed by [`PathResolver::resolve_virtual_path`](crate::PathResolver::resolve_virtual_path)
    /// to convert them to final relative output URLs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use my_module::SrcsetCandidate;
    ///
    /// let srcset = r#"img/a.png 1x, @virtual:page.html|./b.png 2x"#;
    /// let parsed = SrcsetCandidate::parse_srcset(srcset);
    ///
    /// assert_eq!(parsed.len(), 2);
    /// assert_eq!(parsed[1].url.starts_with("@virtual:"), true);
    /// ```
    pub fn parse_srcset(input: &str) -> Vec<Self> {
        let mut candidates = Vec::new();
        let mut input = input.trim();
        while !input.is_empty() {
            // 1. Skip leading whitespace
            input = input.trim_start();

            // 2. If input is empty after trimming, break
            if input.is_empty() {
                break;
            }

            // 3. Extract URL
            let (url, rest) = if input.starts_with('"') {
                // Quoted URL
                let closing_quote = input[1..].find('"');
                match closing_quote {
                    Some(pos) => {
                        let url = &input[1..=pos];
                        let rest = &input[pos + 2..];
                        (url.trim(), rest)
                    }
                    None => break, // malformed, stop parsing
                }
            } else {
                // Unquoted URL, ends at first whitespace or comma
                let mut url_end = 0;
                for (i, c) in input.char_indices() {
                    if c.is_whitespace() || c == ',' {
                        break;
                    }
                    url_end = i + c.len_utf8();
                }
                (&input[..url_end], &input[url_end..])
            };

            let rest = rest.trim_start();

            // 4. Extract descriptor if present (up to comma)
            let (descriptor, remainder) = if let Some(comma_index) = rest.find(',') {
                let desc = &rest[..comma_index].trim();
                (if desc.is_empty() { None } else { Some(desc.to_string()) }, &rest[comma_index + 1..])
            } else {
                // No comma, everything left might be descriptor
                let desc = rest.trim();
                (if desc.is_empty() { None } else { Some(desc.to_string()) }, "")
            };

            // 5. Save candidate
            if !url.is_empty() {
                candidates.push(SrcsetCandidate {
                    url: url.to_string(),
                    descriptor,
                });
            }

            input = remainder.trim_start();
        }

        candidates
    }

    /// Formats a list of [`SrcsetCandidate`]s into a valid `srcset` string.
    ///
    /// Each candidate is serialized as:
    /// ```text
    /// <url> [descriptor]
    /// ```
    ///
    /// Virtual paths are preserved in their encoded form unless they have
    /// been resolved externally.
    ///
    /// # Example
    ///
    /// ```rust
    /// use my_module::SrcsetCandidate;
    ///
    /// let candidates = vec![
    ///     SrcsetCandidate { url: "@virtual:page.html|./img/a.png".into(), descriptor: Some("1x".into()) },
    ///     SrcsetCandidate { url: "img/b.png".into(), descriptor: Some("2x".into()) },
    /// ];
    ///
    /// let result = SrcsetCandidate::format_srcset(&candidates);
    /// assert_eq!(result, "@virtual:page.html|./img/a.png 1x, img/b.png 2x");
    /// ```
    pub fn format_srcset(candidates: &[Self]) -> String {
        candidates
            .iter()
            .map(|c| match &c.descriptor {
                Some(desc) => format!("{} {}", c.url, desc),
                None => c.url.clone(),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}


// #[derive(Debug, Clone, PartialEq)]
// pub struct SrcsetCandidate {
//     pub url: String,
//     pub descriptor: Option<String>,
// }

// impl SrcsetCandidate {
//     pub fn parse_srcset(input: &str) -> Vec<Self> {
//         let mut input = input.trim();
//         let mut output = Vec::new();
    
//         while !input.is_empty() {
//             // 1. Skip leading whitespace
//             input = input.trim_start();
    
//             // 2. Extract URL
//             let mut url_end = 0;
//             let mut in_url = false;
//             for (i, c) in input.char_indices() {
//                 if c == ',' || c.is_whitespace() {
//                     break;
//                 }
//                 in_url = true;
//                 url_end = i + c.len_utf8();
//             }
    
//             if !in_url {
//                 break;
//             }
    
//             let url = &input[..url_end];
//             input = &input[url_end..];
    
//             // 3. Skip whitespace after URL
//             input = input.trim_start();
    
//             // 4. Parse descriptor (if any)
//             let mut descriptor = None;
//             if !input.is_empty() && !input.starts_with(',') {
//                 let mut desc_end = 0;
//                 for (i, c) in input.char_indices() {
//                     if c == ',' {
//                         break;
//                     }
//                     desc_end = i + c.len_utf8();
//                 }
    
//                 if desc_end > 0 {
//                     let desc = input[..desc_end].trim();
//                     if !desc.is_empty() {
//                         descriptor = Some(desc.to_string());
//                     }
//                     input = &input[desc_end..];
//                 }
//             }
    
//             output.push(SrcsetCandidate {
//                 url: url.to_string(),
//                 descriptor,
//             });
    
//             // 5. Skip over comma
//             if let Some(pos) = input.find(',') {
//                 input = &input[pos + 1..];
//             } else {
//                 break;
//             }
//         }
    
//         output
//     }
//     pub fn format_srcset(candidates: &[SrcsetCandidate]) -> String {
//         candidates
//             .iter()
//             .map(|c| {
//                 if let Some(desc) = &c.descriptor {
//                     format!("{} {}", c.url, desc)
//                 } else {
//                     c.url.clone()
//                 }
//             })
//             .collect::<Vec<_>>()
//             .join(", ")
//     }
// }

