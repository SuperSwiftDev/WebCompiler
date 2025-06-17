extern crate web_compiler_macro_types as macro_types;
extern crate web_compiler_io_types as io_types;


use std::convert::Infallible;
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::values::url::Url;
use lightningcss::visit_types;
use lightningcss::visitor::{Visit, VisitTypes, Visitor};

use macro_types::project::DependencyRelation;
use macro_types::environment::{AccumulatedEffects, MacroIO, SourceHostRef};
use io_types::Effectful;

// ————————————————————————————————————————————————————————————————————————————
// BASICS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifiedFlag {
    Default,
    Modified,
}

impl ModifiedFlag {
    pub fn mark_modified_mut(&mut self) {
        *self = ModifiedFlag::Modified;
    }
    pub fn union(&self, other: Self) -> Self {
        match (self, other) {
            (Self::Default, Self::Default) => Self::Default,
            (Self::Default, Self::Modified) => Self::Modified,
            (Self::Modified, Self::Default) => Self::Modified,
            (Self::Modified, Self::Modified) => Self::Modified,
        }
    }
    pub fn is_modified(&self) -> bool {
        match self {
            Self::Modified => true,
            Self::Default => false,
        }
    }
}

impl Default for ModifiedFlag {
    fn default() -> Self {
        ModifiedFlag::Default
    }
}

#[derive(Debug, Clone)]
pub struct Payload<Value> {
    pub value: Value,
    pub modified: ModifiedFlag,
}

// ————————————————————————————————————————————————————————————————————————————
// STYLE TAG PRE-PROCESSOR
// ————————————————————————————————————————————————————————————————————————————

pub struct CssPreprocessor<'a> {
    effects: AccumulatedEffects,
    source_context: SourceHostRef<'a>,
    modified: ModifiedFlag,
}

impl<'a> CssPreprocessor<'a> {
    pub fn new(source_context: SourceHostRef<'a>) -> Self {
        Self { effects: Default::default(), source_context, modified: ModifiedFlag::default() }
    }
    pub fn execute(mut self, source_code: &str) -> MacroIO<Payload<String>> {
        let mut stylesheet = StyleSheet::parse(source_code, ParserOptions::default()).unwrap();
        let mut visitor = CssPreprocessorVisitor {
            effects: &mut self.effects,
            source_context: self.source_context,
            modified: &mut self.modified,
        };
        stylesheet.visit(&mut visitor).unwrap();
        // - -
        let printer_options = PrinterOptions { minify: false, ..Default::default() };
        let res: lightningcss::stylesheet::ToCssResult = stylesheet
            .to_css(printer_options)
            .unwrap();
        // - -
        MacroIO::wrap(res.code)
            .and_modify_context(|ctx| {
                ctx.extend(self.effects);
            })
            .map(|result| {
                if self.modified.is_modified() {
                    Payload {
                        value: result,
                        modified: self.modified,
                    }
                } else {
                    Payload {
                        value: source_code.to_string(),
                        modified: self.modified,
                    }
                }
            })
    }
}

struct CssPreprocessorVisitor<'a> {
    effects: &'a mut AccumulatedEffects,
    source_context: SourceHostRef<'a>,
    modified: &'a mut ModifiedFlag,
}

impl<'a, 'i> Visitor<'i> for CssPreprocessorVisitor<'a> {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(URLS)
    }

    fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
        let url_string = url.url.to_string();
        let dependency = self.source_context.file_input().with_dependency_relation(&url_string);
        if dependency.is_external_target() {
            return Ok(())
        }
        let encoded_url = dependency.encode();
        self.effects.dependencies.insert(dependency);
        url.url = encoded_url.into();
        self.modified.mark_modified_mut();
        Ok(())
    }
}

// ————————————————————————————————————————————————————————————————————————————
// CSS POST-PROCESSOR
// ————————————————————————————————————————————————————————————————————————————

pub struct CssPostprocessor<'a> {
    modified: ModifiedFlag,
    environment: &'a (),
}

impl<'a> CssPostprocessor<'a> {
    pub fn new(environment: &'a ()) -> Self {
        Self { environment, modified: ModifiedFlag::default() }
    }
    pub fn execute(mut self, source_code: &str) -> Payload<String> {
        let mut stylesheet = StyleSheet::parse(source_code, ParserOptions::default()).unwrap();
        
        let mut visitor = CssPostprocessorVisitor {
            environment: self.environment,
            modified: &mut self.modified,
        };
        
        stylesheet.visit(&mut visitor ).unwrap();
        
        let res: lightningcss::stylesheet::ToCssResult = stylesheet.to_css(PrinterOptions { minify: false, ..Default::default() }).unwrap();

        if self.modified.is_modified() {
            Payload {
                value: res.code,
                modified: self.modified,
            }
        } else {
            Payload {
                value: source_code.to_string(),
                modified: self.modified,
            }
        }
    }
}

#[allow(unused)]
struct CssPostprocessorVisitor<'a> {
    // resolver_environment: ResolverTarget<'a>,
    environment: &'a (),
    // source: SourceHostRef<'a>,
    modified: &'a mut ModifiedFlag,
}

impl<'a, 'i> Visitor<'i> for CssPostprocessorVisitor<'a> {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(URLS)
    }

    fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
        let url_string = url.url.to_string();
        let decoded_virtual_path = DependencyRelation::decode(&url_string);
        let decoded_virtual_path = match decoded_virtual_path {
            Some(x) => x,
            None => {
                return Ok(())
            }
        };
        if decoded_virtual_path.is_external_target() {
            return Ok(())
        }
        let decoded_url = {
            decoded_virtual_path
                .as_file_dependency()
                .resolved_target_path()
                .to_str()
                .to_owned()
                .unwrap_or(decoded_virtual_path.to.as_str())
                .to_string()
        };
        if url.url.to_string() == decoded_url {
            return Ok(())
        }
        self.modified.mark_modified_mut();
        url.url = decoded_url.clone().into();
        Ok(())
    }
}


