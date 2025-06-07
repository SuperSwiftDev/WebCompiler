use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::{TagBuf, TagIdentifier};
use std::borrow::Cow;

static INLINE_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // — Inline Textual
        "a", "abbr", "b", "bdi", "bdo", "br", "cite", "code", "data", "dfn", "em", "i", "kbd",
        "mark", "q", "rp", "rt", "ruby", "s", "samp", "small", "span", "strong", "sub", "sup",
        "time", "u", "var", "wbr",

        // — Embedded Content
        "audio", "canvas", "embed", "iframe", "img", "math", "object", "picture", "svg", "video",

        // — Interactive Content
        "button", "input", "label", "select", "textarea",

        // — Script/Template/etc.
        "script", "noscript", "template", "slot", "output",
    ]
    .into_iter()
    .collect()
});

static HEADER_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    ["h1", "h2", "h3", "h4", "h5", "h6"].into_iter().collect()
});

static VOID_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "area", "base", "br", "col", "embed", "hr", "img", "input",
        "link", "meta", "source", "track", "wbr",
    ]
    .into_iter()
    .collect()
});

pub fn is_inline_tag(tag: impl TagIdentifier) -> bool {
    match tag.as_normalized() {
        Cow::Borrowed(s) => INLINE_TAGS.contains(s),
        Cow::Owned(s) => INLINE_TAGS.contains(s.as_str()), // fallback, should be rare
    }
}

pub fn is_header_tag(tag: impl TagIdentifier) -> bool {
    match tag.as_normalized() {
        Cow::Borrowed(s) => HEADER_TAGS.contains(s),
        Cow::Owned(s) => HEADER_TAGS.contains(s.as_str()),
    }
}

pub fn is_void_tag(tag: impl TagIdentifier) -> bool {
    match tag.as_normalized() {
        Cow::Borrowed(s) => VOID_TAGS.contains(s),
        Cow::Owned(s) => VOID_TAGS.contains(s.as_str()),
    }
}


pub(crate) static ROOT_HTML_TAG: Lazy<TagBuf> = Lazy::new(|| TagBuf::new("html"));
