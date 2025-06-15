//! New Version

// use std::{collections::VecDeque, convert::Infallible, path::PathBuf};

// use macro_types::environment::{AccumulatedEffects, SourceContext};
// use io_types::Effectful;

// pub fn css_deep_processor(source_context: SourceContext) {
//     let mut effects = AccumulatedEffects::default();
//     let mut processed_files = VecDeque::<PathBuf>::new();
// }

// pub struct DeepCssPreProcessor<'a> {
//     effects: AccumulatedEffects,
//     source_context: SourceContext<'a>,
// }

// impl<'a> DeepCssPreProcessor<'a> {
//     pub fn new(source_context: SourceContext<'a>) -> Self {
//         Self { effects: Default::default(), source_context }
//     }
//     pub fn execute(mut self) -> Result<MacroIO<String>, Box<dyn std::error::Error>> {
//         use lightningcss::visitor::Visit;
//         let source_code = self.source_context.file_input().load_source_file()?;
//         let parser_options = lightningcss::stylesheet::ParserOptions::default();
//         let mut stylesheet = lightningcss::stylesheet::StyleSheet::parse(&source_code, parser_options).unwrap();
//         let mut visitor = CssPreprocessorVisitor {
//             effects: &mut self.effects,
//             source_context: self.source_context,
//         };
//         stylesheet.visit(&mut visitor).unwrap();
//         // - -
//         let printer_options = lightningcss::printer::PrinterOptions {
//             minify: false,
//             ..Default::default()
//         };
//         let res: lightningcss::stylesheet::ToCssResult = stylesheet
//             .to_css(printer_options)
//             .unwrap();
//         // - -
//         Ok(MacroIO::wrap(res.code).and_modify_context(|ctx| {
//             ctx.extend(self.effects);
//         }))
//     }
// }

// struct CssPreprocessorVisitor<'a> {
//     effects: &'a mut AccumulatedEffects,
//     source_context: SourceContext<'a>,
// }

// impl<'a, 'i> lightningcss::visitor::Visitor<'i> for CssPreprocessorVisitor<'a> {
//     type Error = Infallible;

//     fn visit_types(&self) -> lightningcss::visitor::VisitTypes {
//         lightningcss::visit_types!(URLS)
//     }

//     fn visit_url(
//         &mut self,
//         url: &mut lightningcss::values::url::Url<'i>,
//     ) -> Result<(), Self::Error> {
//         let url_string = url.url.to_string();
//         let dependency = self.source_context.file_input().with_dependency_relation(&url_string);
//         let encoded_url = dependency.encode();
//         self.effects.dependencies.insert(dependency);
//         url.url = encoded_url.into();
//         Ok(())
//     }
// }

