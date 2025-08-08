use macro_types::macro_tag::MacroTag;
use macro_types::lexical_env::MacroIO;

use macro_types::scope::BinderValue;
use web_compiler_types::CompilerRuntime;
use xml_ast::Node;

#[derive(Debug, Clone, Copy, Default)]
pub struct DefineMacroTag;

impl MacroTag for DefineMacroTag {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "define" }
    fn apply(
        &self,
        attributes: xml_ast::AttributeMap,
        _: xml_ast::Fragment,
        scope: &mut macro_types::lexical_env::ProcessScope,
        _: &Self::Runtime,
    ) -> MacroIO<xml_ast::Node> {
        let key_target = attributes.get("key");
        let value_target = attributes.get("value");
        match (key_target, value_target) {
            (Some(key_target), Some(value_target)) => {
                let key_target = key_target.as_str();
                let value_target = value_target.as_str();
                scope.binding_scope.insert(key_target, BinderValue::json_string(value_target));
            }
            _ => ()
        }
        MacroIO::wrap(Node::empty())
    }
}
