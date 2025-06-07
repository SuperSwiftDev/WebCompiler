#![allow(unused)]
use std::collections::HashMap;
use std::path::Path;

use web_compiler_html_ast::{Element, Html, ParserMode, TagBuf};
use web_compiler_html_ast::transform::{Either, EffectPropagator, HtmlTransformer, IO};
use web_compiler_common::{InputRule, ProjectContext, SourceContext};

pub mod bind;
pub mod content;
pub mod enumerate;
pub mod include;
pub mod value;
pub mod attributes;
pub mod links;

mod environment;

pub use environment::*;


#[derive(Debug, Clone)]
pub struct PreProcessor {
    source_context: SourceContext,
}

impl PreProcessor {
    pub fn new(source_context: SourceContext) -> Self {
        Self { source_context }
    }
    pub fn source_context(&self) -> &SourceContext {
        &self.source_context
    }
    pub fn new_source_context(&self, source_context: SourceContext) -> Self {
        Self { source_context: source_context }
    }
    pub fn subprocess_html_file(
        &self,
        rel_src_path: impl AsRef<Path>,
        scope: &mut ScopeBindingEnv,
        parser_mode: ParserMode,
    ) -> Result<PreProcessIO<Html>, Box<dyn std::error::Error>> {
        let source_context = self.source_context();
        let resolved_path = source_context.source_dir().join(rel_src_path);
        let resolved_path = path_clean::clean(resolved_path);
        let source = std::fs::read_to_string(&resolved_path)?;
        let source_tree = Html::parse(&source, parser_mode);
        let new_source_context = SourceContext {
            project_context: source_context.project_context.clone(),
            input_rule: InputRule {
                source: resolved_path.clone(),
                public: None,
                template: None,
                subtemplate: None,
            }
        };
        let sub_visitor = self.new_source_context(new_source_context);
        let result = sub_visitor.enter(source_tree, scope);
        Ok(result)
    }
    pub fn process_template_context(
        &self,
        rel_src_path: impl AsRef<Path>,
        parser_mode: ParserMode,
        content: Html,
    ) -> Result<(Html, AccumulatedEffects), Box<dyn std::error::Error>> {
        let mut scope_binding_env = ScopeBindingEnv::default();
        scope_binding_env.define_binding(
            "content",
            BinderValue::Html(content),
        );
        // self.subprocess_html_file(rel_src_path, &mut scope_binding_env, parser_mode)
        //     .map(|x| x.collapse())
        // - -
        let source_context = self.source_context();
        let resolved_path = path_clean::clean(rel_src_path.as_ref());
        let source = std::fs::read_to_string(&resolved_path)?;
        let source_tree = Html::parse(&source, parser_mode);
        let new_source_context = SourceContext {
            project_context: source_context.project_context.clone(),
            input_rule: InputRule {
                source: resolved_path.clone(),
                public: None,
                template: None,
                subtemplate: None,
            }
        };
        let sub_visitor = self.new_source_context(new_source_context);
        let result = sub_visitor.enter(source_tree, &mut scope_binding_env);
        let result = result.collapse();
        Ok(result)
    }
    pub fn compile(&self, parser_mode: ParserMode) -> Result<(Html, AccumulatedEffects), Box<dyn std::error::Error>> {
        let source = std::fs::read_to_string(self.source_context.source_file())?;
        let source_tree = Html::parse(&source, parser_mode);
        let mut scope = ScopeBindingEnv::default();
        let result = web_compiler_html_ast::transform::evaluate_html_transformer(
            source_tree,
            self,
            &mut scope
        );
        Ok(result)
    }
}

impl HtmlTransformer for PreProcessor {
    type Output = Html;
    type Scope = ScopeBindingEnv;
    type Effect = AccumulatedEffects;

    fn transform_text(&self, text: String, _: &mut Self::Scope) -> IO<Self::Output, Self::Effect> {
        IO::wrap(Html::Text(text))
    }
    fn transform_fragment(&self, fragment: Vec<Self::Output>, _: &mut Self::Scope) -> IO<Self::Output, Self::Effect> {
        IO::wrap(Html::Fragment(fragment))
    }
    fn transform_element(
        &self,
        tag: TagBuf,
        mut attrs: HashMap<String, String>,
        children: Vec<Self::Output>,
        scope: &mut Self::Scope,
    ) -> IO<Self::Output, Self::Effect> {
        let mut context = AccumulatedEffects::default();
        attributes::resolve_attribute_bindings(&mut attrs, &scope);
        links::virtualize_local_attribute_paths(&tag, &mut attrs, &mut context, self.source_context());
        PreProcessIO::wrap(Html::Element(Element { tag, attrs, children }))
            .and_modify_context(|ctx| {
                ctx.merge_mut(context);
            })
    }
    fn manual_top_down_element_handler(&self, element: Element, scope: &mut Self::Scope) -> IO<Either<Self::Output, Element>, Self::Effect> {
        let Element { tag, attrs, children } = element;
        let output = match tag.as_normalized() {
            "bind" => bind::bind_element_handler(self, tag, attrs, children, scope),
            "content" => content::content_element_handler(self, tag, attrs, children, scope),
            "enumerate" => enumerate::enumerate_element_handler(self, tag, attrs, children, scope),
            "include" => include::include_element_handler(self, tag, attrs, children, scope),
            "value" => value::value_element_handler(self, tag, attrs, children, scope),
            _ => return IO::wrap(Either::Right(Element { tag, attrs, children }))
        };
        output.map(Either::Left)
    }
}




