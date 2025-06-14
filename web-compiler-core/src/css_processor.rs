#![allow(unused)]
use std::convert::Infallible;
use std::path::PathBuf;
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::traits::ToCss;
use lightningcss::rules::CssRule;
use lightningcss::values::length::LengthValue;
use lightningcss::values::url::Url;
use lightningcss::visit_types;
use lightningcss::visitor::{Visit, VisitTypes, Visitor};

use macro_types::project::DependencyRelation;
use macro_types::environment::{AccumulatedEffects, MacroIO, SourceContext, MacroRuntime};
use io_types::Effectful;


// ————————————————————————————————————————————————————————————————————————————
// STYLE TAG PRE-PROCESSOR
// ————————————————————————————————————————————————————————————————————————————

pub struct CssPreprocessor<'a> {
    effects: AccumulatedEffects,
    source_context: SourceContext<'a>,
}

impl<'a> CssPreprocessor<'a> {
    pub fn new(source_context: SourceContext<'a>) -> Self {
        Self { effects: Default::default(), source_context }
    }
    pub fn execute(mut self, source_code: &str) -> MacroIO<String> {
        let mut stylesheet = StyleSheet::parse(source_code, ParserOptions::default()).unwrap();
        let mut visitor = CssPreprocessorVisitor {
            effects: &mut self.effects,
            source_context: self.source_context,
        };
        stylesheet.visit(&mut visitor).unwrap();
        // - -
        let printer_options = PrinterOptions { minify: false, ..Default::default() };
        let res: lightningcss::stylesheet::ToCssResult = stylesheet
            .to_css(printer_options)
            .unwrap();
        // - -
        MacroIO::wrap(res.code).and_modify_context(|ctx| {
            ctx.extend(self.effects);
        })
    }
}

struct CssPreprocessorVisitor<'a> {
    effects: &'a mut AccumulatedEffects,
    source_context: SourceContext<'a>,
}

impl<'a, 'i> Visitor<'i> for CssPreprocessorVisitor<'a> {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(URLS)
    }

    fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
        let url_string = url.url.to_string();
        let dependency = self.source_context.file_input().with_dependency_relation(&url_string);
        let encoded_url = dependency.encode();
        self.effects.dependencies.insert(dependency);
        url.url = encoded_url.into();
        Ok(())
    }
}

// ————————————————————————————————————————————————————————————————————————————
// CSS POST-PROCESSOR
// ————————————————————————————————————————————————————————————————————————————

pub struct CssPostprocessor<'a> {
    environment: &'a (),
}

impl<'a> CssPostprocessor<'a> {
    pub fn new(environment: &'a ()) -> Self {
        Self { environment }
    }
    pub fn execute(self, source_code: &str) -> String {
        let mut stylesheet = StyleSheet::parse(source_code, ParserOptions::default()).unwrap();
        
        let mut visitor = CssPostprocessorVisitor {
            environment: self.environment,
        };
        
        stylesheet.visit(&mut visitor ).unwrap();
        
        let res: lightningcss::stylesheet::ToCssResult = stylesheet.to_css(PrinterOptions { minify: false, ..Default::default() }).unwrap();

        res.code
    }
}

struct CssPostprocessorVisitor<'a> {
    environment: &'a (),
}

impl<'a, 'i> Visitor<'i> for CssPostprocessorVisitor<'a> {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(URLS)
    }

    fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
        // println!("resolve_virtual_path: {:?}", url.url);
        let url_string = url.url.to_string();
        let decoded_url = DependencyRelation::decode(&url_string)
            .map(|relation| {
                if relation.is_external_target() {
                    return relation.to
                }
                let path = relation.as_file_dependency().resolved_target_path();
                path.to_str()
                    .to_owned()
                    .unwrap_or(relation.to.as_str())
                    .to_string()
            })
            .unwrap_or_else(|| url_string.clone());
        url.url = decoded_url.into();
        Ok(())
    }
}



