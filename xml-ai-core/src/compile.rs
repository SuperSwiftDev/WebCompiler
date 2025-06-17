use std::rc::Rc;

use xml_ast::Node;

pub fn parse_compile(source: impl AsRef<str>) -> Result<Node, crate::dsl::DslFormatErrorList> {
    let xml_ast::ParserPayload {output, errors} = xml_ast::parse_fragment_str(source);
    if !errors.is_empty() {
        return Err(crate::dsl::DslFormatErrorList::new(Rc::new(ParserErrors { errors })))
    }
    let _ = output;
    unimplemented!("TODO")
}

#[derive(Debug, Clone)]
pub struct ParserErrors {
    errors: Vec<String>,
}

impl ParserErrors {
    pub fn joined(&self, separator: impl AsRef<str>) -> String {
        self.errors
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(separator.as_ref())
    }
}

impl std::fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.joined(" ∙ ");
        write!(f, "[parser errors] {items}")
    }
}

impl std::error::Error for ParserErrors {}

impl crate::dsl::DslFormatError for ParserErrors {
    fn singleton(&self) -> crate::dsl::DslFormatErrorList { crate::dsl::DslFormatErrorList::new(Rc::new(self.clone())) }
}

