use crate::scope::{BinderValue, JsonBinderValue, MarkupBinderValue, BindingScope};

/// Individual segments in a path expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    Field(String),         // `.field`
    Attribute(String),     // `@attribute`
    Children,              // `~`
}

/// Full path expression with root and optional segments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathExpression {
    pub root: String,
    pub segments: Vec<PathSegment>,
}

impl PathExpression {
    /// Parses a path expression string like `user.name`, `tab@label`, or `section~`.
    pub fn parse(input: &str) -> Result<Self, String> {
        let rest = input;

        let (root, tail_start) = match rest.find(|c: char| c == '.' || c == '@' || c == '~') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, ""),
        };

        if !is_valid_identifier(root) {
            return Err(format!("Invalid root identifier: `{}`", root));
        }

        let mut segments = Vec::new();
        let mut cursor = tail_start;

        while !cursor.is_empty() {
            if cursor.starts_with('.') {
                cursor = &cursor[1..];
                let next = cursor.find(|c: char| c == '.' || c == '@' || c == '~')
                    .unwrap_or(cursor.len());
                let field = &cursor[..next];
                if !is_valid_identifier(field) {
                    return Err(format!("Invalid field name after dot: `{}`", field));
                }
                segments.push(PathSegment::Field(field.to_string()));
                cursor = &cursor[next..];
            } else if cursor.starts_with('@') {
                cursor = &cursor[1..];
                if !cursor.is_empty() {
                    if let Some((attr, _)) = cursor.split_once(|c: char| c == '.' || c == '@' || c == '~') {
                        return Err(format!("Unexpected continuation after attribute access: `@{}`", attr));
                    }
                    if !is_valid_identifier(cursor) {
                        return Err(format!("Invalid attribute name: `{}`", cursor));
                    }
                    segments.push(PathSegment::Attribute(cursor.to_string()));
                    cursor = "";
                } else {
                    return Err("Missing attribute name after `@`".into());
                }
            } else if cursor.starts_with('~') {
                cursor = &cursor[1..];
                if !cursor.is_empty() {
                    return Err("`~` must appear at end of path".into());
                }
                segments.push(PathSegment::Children);
                break;
            } else {
                return Err(format!("Unexpected character in path: `{}`", cursor));
            }
        }

        Ok(PathExpression {
            root: root.to_string(),
            segments,
        })
    }

    /// Evaluates the expression against a given binding scope
    pub fn evaluate(&self, scope: &BindingScope) -> Option<BinderValue> {
        let base = scope.lookup(&self.root)?.clone();

        match base {
            BinderValue::Json(mut json_val) => {
                for segment in &self.segments {
                    match segment {
                        PathSegment::Field(name) => {
                            match json_val {
                                JsonBinderValue::Object(ref map) => {
                                    json_val = map.get(name)?.clone();
                                }
                                _ => return None,
                            }
                        }
                        PathSegment::Attribute(_) | PathSegment::Children => {
                            return None; // Invalid on JSON
                        }
                    }
                }
                Some(BinderValue::Json(json_val))
            }

            BinderValue::Markup(markup_val) => {
                if self.segments.is_empty() {
                    return Some(BinderValue::Markup(markup_val));
                }

                match &self.segments[..] {
                    [PathSegment::Attribute(attr)] => {
                        markup_val.0
                            .lookup_element_attribute(attr)
                            .map(|val| {
                                BinderValue::Json(JsonBinderValue::String(val.to_string()))
                            })
                    }
                    [PathSegment::Children] => {
                        markup_val.0.as_element()
                            .map(|element| element.children.clone())
                            .map(|fragment| {
                                BinderValue::Markup(MarkupBinderValue(xml_ast::Node::Fragment(fragment)))
                            })
                    }
                    _ => None, // Chaining on attributes/children not allowed
                }
            }
        }
    }
}

fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}
