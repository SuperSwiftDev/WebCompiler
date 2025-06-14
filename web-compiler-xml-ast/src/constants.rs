use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::TagBuf;
// use std::borrow::Cow;

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

pub fn is_inline_tag(tag: &TagBuf) -> bool {
    INLINE_TAGS.contains(tag.as_normalized())
}

pub fn is_header_tag(tag: &TagBuf) -> bool {
    HEADER_TAGS.contains(tag.as_normalized())
}

pub fn is_void_tag(tag: &TagBuf) -> bool {
    VOID_TAGS.contains(tag.as_normalized())
}


// pub(crate) static ROOT_HTML_TAG: Lazy<TagBuf> = Lazy::new(|| TagBuf::new("html"));

