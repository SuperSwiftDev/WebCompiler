// use macro_types::scope::MarkupBinderValue;
use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;

use web_compiler_types::CompilerRuntime;
// use xml_ast::{Element, Node};

#[derive(Debug, Clone, Copy, Default)]
pub struct RewriteMacroTag;

impl MacroTag for RewriteMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "rewrite" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        children: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        _: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let _ = attributes;
        let _ = children;
        let _ = scope;
        // let title_element = Element::new("title").with_children(children);
        // if let Some(bind_title) = attributes.get("bind:title") {
        //     scope.binding_scope.insert("title", BinderValue::json_string(bind_title.clone().as_str()));
        // }
        // return MacroIO::wrap(Node::empty()).and_modify_context(|ctx| {
        //     ctx.hoisted.push(BinderValue::Markup(MarkupBinderValue(Node::Element(title_element))));
        // });
        unimplemented!("TODO")
    }
}
