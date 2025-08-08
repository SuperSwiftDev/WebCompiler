use std::str::FromStr;

use macro_types::environment::{MacroIO, ProcessScope, SourceHostRef};
use macro_types::tag_rewrite_rule::TagRewriteRule;
use xml_ast::{Element, Node, TagBuf};

use web_compiler_types::CompilerRuntime;


// ————————————————————————————————————————————————————————————————————————————
// TYPES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, Default)]
pub struct H1RewriteRule;

#[derive(Debug, Clone, Copy, Default)]
pub struct H2RewriteRule;

#[derive(Debug, Clone, Copy, Default)]
pub struct H3RewriteRule;

#[derive(Debug, Clone, Copy, Default)]
pub struct H4RewriteRule;

#[derive(Debug, Clone, Copy, Default)]
pub struct H5RewriteRule;
#[derive(Debug, Clone, Copy, Default)]
pub struct H6RewriteRule;

// ————————————————————————————————————————————————————————————————————————————
// TYPES — INTERNAL
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Baseline {
    H1, H2, H3, H4, H5, H6
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadingLevel {
    H1, H2, H3, H4, H5, H6
}

impl Baseline {
    // TODO: reduce LOC?
    pub fn apply(self, heading_level: HeadingLevel) -> HeadingLevel {
        match (self, heading_level) {
            (Self::H1, HeadingLevel::H1) => HeadingLevel::H1,
            (Self::H1, HeadingLevel::H2) => HeadingLevel::H2,
            (Self::H1, HeadingLevel::H3) => HeadingLevel::H3,
            (Self::H1, HeadingLevel::H4) => HeadingLevel::H4,
            (Self::H1, HeadingLevel::H5) => HeadingLevel::H5,
            (Self::H1, HeadingLevel::H6) => HeadingLevel::H6,
            // - -
            (Self::H2, HeadingLevel::H1) => HeadingLevel::H2,
            (Self::H2, HeadingLevel::H2) => HeadingLevel::H3,
            (Self::H2, HeadingLevel::H3) => HeadingLevel::H4,
            (Self::H2, HeadingLevel::H4) => HeadingLevel::H5,
            (Self::H2, HeadingLevel::H5) => HeadingLevel::H6,
            (Self::H2, HeadingLevel::H6) => HeadingLevel::H6,
            // - -
            (Self::H3, HeadingLevel::H1) => HeadingLevel::H3,
            (Self::H3, HeadingLevel::H2) => HeadingLevel::H4,
            (Self::H3, HeadingLevel::H3) => HeadingLevel::H5,
            (Self::H3, HeadingLevel::H4) => HeadingLevel::H6,
            (Self::H3, HeadingLevel::H5) => HeadingLevel::H6,
            (Self::H3, HeadingLevel::H6) => HeadingLevel::H6,
            // - -
            (Self::H4, HeadingLevel::H1) => HeadingLevel::H4,
            (Self::H4, HeadingLevel::H2) => HeadingLevel::H5,
            (Self::H4, HeadingLevel::H3) => HeadingLevel::H6,
            (Self::H4, HeadingLevel::H4) => HeadingLevel::H6,
            (Self::H4, HeadingLevel::H5) => HeadingLevel::H6,
            (Self::H4, HeadingLevel::H6) => HeadingLevel::H6,
            // - -
            (Self::H5, HeadingLevel::H1) => HeadingLevel::H5,
            (Self::H5, HeadingLevel::H2) => HeadingLevel::H6,
            (Self::H5, HeadingLevel::H3) => HeadingLevel::H6,
            (Self::H5, HeadingLevel::H4) => HeadingLevel::H6,
            (Self::H5, HeadingLevel::H5) => HeadingLevel::H6,
            (Self::H5, HeadingLevel::H6) => HeadingLevel::H6,
            // - -
            (Self::H6, HeadingLevel::H1) => HeadingLevel::H6,
            (Self::H6, HeadingLevel::H2) => HeadingLevel::H6,
            (Self::H6, HeadingLevel::H3) => HeadingLevel::H6,
            (Self::H6, HeadingLevel::H4) => HeadingLevel::H6,
            (Self::H6, HeadingLevel::H5) => HeadingLevel::H6,
            (Self::H6, HeadingLevel::H6) => HeadingLevel::H6,
        }
    }
}

impl HeadingLevel {
    pub fn try_from_tag(tag: &TagBuf) -> Result<Self, ()> {
        Self::from_str(tag.as_normalized())
    }
}

impl AsRef<str> for Baseline {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

impl AsRef<str> for HeadingLevel {
    fn as_ref(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

impl FromStr for Baseline {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            "h4" => Ok(Self::H4),
            "h5" => Ok(Self::H5),
            "h6" => Ok(Self::H6),
            // - -
            "H1" => Ok(Self::H1),
            "H2" => Ok(Self::H2),
            "H3" => Ok(Self::H3),
            "H4" => Ok(Self::H4),
            "H5" => Ok(Self::H5),
            "H6" => Ok(Self::H6),
            // - -
            _ => return Err(())
        }
    }
}

impl FromStr for HeadingLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            "h4" => Ok(Self::H4),
            "h5" => Ok(Self::H5),
            "h6" => Ok(Self::H6),
            // - -
            "H1" => Ok(Self::H1),
            "H2" => Ok(Self::H2),
            "H3" => Ok(Self::H3),
            "H4" => Ok(Self::H4),
            "H5" => Ok(Self::H5),
            "H6" => Ok(Self::H6),
            // - -
            _ => return Err(())
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————


fn apply(element: Element, scope: &mut ProcessScope) -> MacroIO<Node> {
    let baseline = scope.binding_scope
        .lookup("host")
        .and_then(|x| x.as_object())
        .and_then(|x| x.get("baseline"))
        .and_then(|x| x.as_string())
        .and_then(|x| Baseline::from_str(x).ok());
        // .and_then(|x| );
    // let baseline = scope.binding_scope
    //     .lookup("baseline")
    //     .and_then(|baseline| {
    //         baseline.try_cast_to_string()
    //     })
    //     .and_then(|baseline| {
    //         Baseline::from_str(baseline).ok()
    //     });
    let baseline = match baseline {
        Some(x) => x,
        None => return MacroIO::wrap(Node::Element(element)),
    };
    let heading_level = HeadingLevel::try_from_tag(&element.tag);
    let heading_level= match heading_level {
        Ok(x) => x,
        Err(()) => return MacroIO::wrap(Node::Element(element)),
    };
    let Element { tag: _, attributes, children } = element;
    let resolved_heading_level = baseline.apply(heading_level);
    let tag = TagBuf::new(resolved_heading_level.as_ref());
    let element = Element { tag, attributes, children };
    MacroIO::wrap(Node::Element(element))
}

impl TagRewriteRule for H1RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h1" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

impl TagRewriteRule for H2RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h2" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

impl TagRewriteRule for H3RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h3" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

impl TagRewriteRule for H4RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h4" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

impl TagRewriteRule for H5RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h5" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}

impl TagRewriteRule for H6RewriteRule {
    type Runtime = CompilerRuntime;
    fn tag_name(&self) -> &'static str { "h6" }
    fn pre_process( &self, element: Element, scope: &mut ProcessScope, _: &Self::Runtime) -> MacroIO<Node> {
        apply(element, scope)
    }
    fn post_process(&self, element: Element, _: &SourceHostRef) -> Node {
        Node::Element(element)
    }
}


