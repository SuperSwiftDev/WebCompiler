#![allow(unused)]
// use std::collections::{BTreeMap, HashMap};

use crate::{AttributeMap, Element, Fragment, Node, TagBuf};

mod pretty_html;

// ————————————————————————————————————————————————————————————————————————————
// SETTINGS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Default)]
pub struct FormatSettings {}

// ————————————————————————————————————————————————————————————————————————————
// INTERNAL HELPERS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatType { Inline, Block }

impl Default for FormatType {
    fn default() -> Self {
        FormatType::Block
    }
}

#[derive(Debug, Clone)]
struct FormatEnvironment {
    indent: usize,
    format_type: FormatType,
    escape_tokens: bool,
    settings: FormatSettings,
}

impl FormatEnvironment {
    pub fn new(settings: FormatSettings) -> Self {
        Self {
            indent: 0,
            format_type: FormatType::Block,
            escape_tokens: false,
            settings: settings,
        }
    }
    pub fn scope(&self, tag: &TagBuf) -> FormatEnvironment {
        let format_type = match self.format_type {
            FormatType::Block if crate::constants::is_inline_tag(tag) => FormatType::Inline,
            _ => self.format_type
        };
        let auto_indent: bool = match tag.as_normalized() {
            "html" => false,
            "head" => false,
            "body" => false,
            _ => format_type == FormatType::Block,
        };
        let escape_tokens = if self.escape_tokens {
            if tag.as_normalized() == "script" || tag.as_normalized() == "style" {
                false
            } else {
                true
            }
        } else {
            false
        };
        FormatEnvironment {
            indent: {
                if auto_indent {
                    self.indent + 1
                } else {
                    self.indent
                }
            },
            format_type: format_type,
            escape_tokens: escape_tokens,
            settings: self.settings.clone(),
        }
    }
    pub fn indent(self) -> FormatEnvironment {
        FormatEnvironment {
            indent: self.indent + 1,
            format_type: self.format_type,
            escape_tokens: self.escape_tokens,
            settings: self.settings.clone(),
        }
    }
    pub fn inline(self) -> FormatEnvironment {
        FormatEnvironment {
            indent: self.indent,
            format_type: FormatType::Inline,
            escape_tokens: self.escape_tokens,
            settings: self.settings.clone(),
        }
    }
    fn indent_spacing_string(&self) -> String {
        indent_spacing_string(self.indent)
    }
    fn is_in_inline_mode(&self) -> bool {
        self.format_type == FormatType::Inline
    }
    fn with_escape_tokens(self, escape_tokens: bool) -> Self {
        Self {
            indent: self.indent,
            format_type: self.format_type,
            escape_tokens: escape_tokens,
            settings: self.settings.clone(),
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// AST IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————

impl Node {
    pub fn format_document(&self) -> String {
        let doc_type = "<!DOCTYPE html>";
        let html = self.format(FormatSettings::default());
        format!("{doc_type}\n{html}")
    }
    pub fn format_document_pretty(&self) -> String {
        self.pretty_format()
    }
    pub fn format(&self, settings: FormatSettings) -> String {
        let environment = FormatEnvironment::new(settings);
        self.render_impl(&environment)
    }
    pub fn pretty_format(&self) -> String {
        let format_settings = FormatSettings::default();
        let string = self.format(format_settings);
        let pretty = pretty_html::prettify_html(&string).unwrap_or_else(|error| {
            eprintln!("PRETTY-HTML: {error}");
            string
        });
        pretty
    }
}
impl Element {
    pub fn format(&self, settings: FormatSettings) -> String {
        let environment = FormatEnvironment::new(settings);
        self.render_impl(&environment)
    }
    pub fn pretty_format(&self) -> String {
        let format_settings = FormatSettings::default();
        let string = self.format(format_settings);
        let pretty = pretty_html::prettify_html(&string).unwrap_or_else(|error| {
            eprintln!("PRETTY-HTML: {error}");
            string
        });
        pretty
    }
}
impl Fragment {
    pub fn format(&self, settings: FormatSettings) -> String {
        let environment = FormatEnvironment::new(settings);
        self.render_impl(&environment)
    }
    pub fn pretty_format(&self) -> String {
        let format_settings = FormatSettings::default();
        let string = self.format(format_settings);
        let pretty = pretty_html::prettify_html(&string).unwrap_or_else(|error| {
            eprintln!("PRETTY-HTML: {error}");
            string
        });
        pretty
    }
}


impl Node {
    fn render_impl(&self, environment: &FormatEnvironment) -> String {
        match self {
            Self::Text(text) => text.to_owned(),
            Self::Element(element) => element.render_impl(environment),
            Self::Fragment(fragment) => fragment.render_impl(environment),
        }
    }
}

impl Element {
    fn render_impl(&self, environment: &FormatEnvironment) -> String {
        let environment = environment.scope(&self.tag);
        // let level = environment.indent_spacing_string();
        let attributes = format_attributes(&self.attributes);
        if crate::constants::is_void_tag(&self.tag) && self.children.len() == 0 {
            format!(
                "<{tag}{attributes} />",
                tag=self.tag.as_original(),
            )
        } else {
            // let environment = environment.with_escape_tokens()
            let children = format_fragment(&self.children, &environment);
            let contents = {
                children
            };
            format!(
                "<{tag}{attributes}>{contents}</{tag}>",
                tag=self.tag.as_original(),
            )
        }
    }
}

impl Fragment {
    fn render_impl(&self, environment: &FormatEnvironment) -> String {
        format_fragment(self, environment)
    }
}


// ————————————————————————————————————————————————————————————————————————————
// INTERNAL UTILITIES
// ————————————————————————————————————————————————————————————————————————————

fn indent_spacing_string(level: usize) -> String {
    if level == 0 {
        String::from("")
    } else {
        std::iter::repeat(" ").take(level * 2).collect::<String>()
    }
}

fn format_fragment(nodes: &Fragment, environment: &FormatEnvironment) -> String {
    let xs = nodes
        .iter()
        .map(|child| {
            let environment = environment.clone();
            child.render_impl(&environment)
        })
        .collect::<Vec<_>>();
    if xs.is_empty() {
        String::new()
    } else {
        xs.join("")
    }
}

fn format_attributes(
    attributes: &AttributeMap,
) -> String {
    let attributes = attributes
        .into_iter()
        .map(|(key, value)| {
            // println!("{key:?}: {value:?}");
            // if value.is_empty() {
            //     return format!("{}", key);
            // }
            format!("{key}={value:?}")
        })
        .collect::<Vec<_>>();
    if attributes.is_empty() {
        String::new()
    } else {
        format!(" {}", attributes.join(" "))
    }
}

