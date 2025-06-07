#![allow(unused)]
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::Html;
use crate::Element;
use crate::TagBuf;
use crate::TagIdentifier;

mod pretty_html;

// ————————————————————————————————————————————————————————————————————————————
// PRETTY PRINTER
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct PrettyPrinter {}

#[derive(Debug, Clone)]
pub struct Environment {
    indent: usize,
    format_type: FormatType,
    escape_tokens: bool,
}

impl Environment {
    pub fn scope(&self, tag: &TagBuf) -> Environment {
        let format_type = match self.format_type {
            FormatType::Block if crate::is_inline_tag(tag) => FormatType::Inline,
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
        Environment {
            indent: {
                if auto_indent {
                    self.indent + 1
                } else {
                    self.indent
                }
            },
            format_type: format_type,
            escape_tokens: escape_tokens,
        }
    }
    pub fn indent(self) -> Environment {
        Environment { indent: self.indent + 1, format_type: self.format_type, escape_tokens: self.escape_tokens }
    }
    pub fn inline(self) -> Environment {
        Environment {
            indent: self.indent,
            format_type: FormatType::Inline,
            escape_tokens: self.escape_tokens,
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
            escape_tokens: escape_tokens
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatType { Inline, Block }

impl Default for FormatType {
    fn default() -> Self {
        FormatType::Block
    }
}


impl Default for Environment {
    fn default() -> Self {
        Environment {
            indent: 0,
            format_type: FormatType::default(),
            escape_tokens: true
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————

impl Html {
    pub fn render_html_string(&self, pretty_print: bool, ensure_doc_tag: bool) -> String {
        if pretty_print {
            self.pretty_html_string() // WILL IMPLICITELY INCLUDE DOC TAG
        } else {
            let html_string = self.render_html_string_impl(&Default::default());
            if ensure_doc_tag {
                let doctype = "<!DOCTYPE html>";
                format!("{doctype}\n{html_string}",)
            } else {
                html_string
            }
        }
    }
    fn pretty_html_string(&self) -> String {
        let string = self.render_html_string_impl(&Default::default());
        let pretty = pretty_html::prettify_html(&string).unwrap_or_else(|error| {
            eprintln!("PRETTY-HTML: {error}");
            string
        });
        pretty
    }
    fn render_html_string_impl(&self, environment: &Environment) -> String {
        match self {
            Self::Element(element) => element.render_html_string_impl(environment),
            Self::Fragment(nodes) => format_fragment(&nodes, environment),
            Self::Text(text) => {
                if environment.escape_tokens {
                    escape_html(text)
                } else {
                    text.to_owned()
                }
            },
        }
    }
}

impl Element {
    fn render_html_string_impl(&self, environment: &Environment) -> String {
        let environment = environment.scope(&self.tag);
        let level = environment.indent_spacing_string();
        let attributes = format_attributes(&self.attrs);
        if crate::is_void_tag(&self.tag) && self.children.len() == 0 {
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

fn format_fragment(nodes: &[Html], environment: &Environment) -> String {
    let xs = nodes
        .iter()
        .map(|child| {
            let environment = environment.clone();
            child.render_html_string_impl(&environment)
        })
        .collect::<Vec<_>>();
    if xs.is_empty() {
        String::new()
    } else {
        xs.join("")
    }
}

fn format_attributes(attributes: &HashMap<String, String>) -> String {
    let attributes = attributes
        .clone()
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    let attributes = attributes
        .iter()
        .map(|(key, value)| format!("{key}={value:?}"))
        .collect::<Vec<_>>();
    if attributes.is_empty() {
        String::new()
    } else {
        format!(" {}", attributes.join(" "))
    }
}

fn indent_spacing_string(level: usize) -> String {
    if level == 0 {
        String::from("")
    } else {
        std::iter::repeat(" ").take(level * 2).collect::<String>()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// INTERNAL
// ————————————————————————————————————————————————————————————————————————————

fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            c if c as u32 > 127 => {
                // Optional: numeric entity for non-ASCII
                use std::fmt::Write;
                write!(escaped, "&#x{:X};", c as u32).unwrap();
            }
            c => escaped.push(c),
        }
    }

    escaped
}

