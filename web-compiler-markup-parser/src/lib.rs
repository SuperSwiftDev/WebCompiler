extern crate markup5ever_rcdom as rcdom;

pub trait HtmlTreeBuilder {
    type Output: Clone;
    fn text_node(&mut self, text: String) -> Self::Output;
    fn element_node(&mut self, name: String, attributes: Vec<(String, String)>, children: Vec<Self::Output>) -> Self::Output;
    fn fragment_node(&mut self, fragment: Vec<Self::Output>) -> Self::Output;
    fn comment_node(&mut self, text: String) -> Self::Output;
}

pub struct ParserPayload<Output> {
    pub output: Output,
    pub errors: Vec<String>,
}

pub fn parse_document_str<Builder: HtmlTreeBuilder>(source: impl AsRef<str>, builder: &mut Builder) -> ParserPayload<Vec<Builder::Output>> {
    use html5ever::tendril::TendrilSink;
    let mut input_reader = std::io::Cursor::new(source.as_ref());
    let dom = html5ever::parse_document(rcdom::RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut input_reader)
        .unwrap();
    let errors = dom.errors
        .borrow()
        .iter()
        .map(|error| {
            error.to_string()
        })
        .collect::<Vec<_>>();
    ParserPayload {
        errors,
        output: to_output_format(&dom.document, builder),
    }
}

pub fn parse_fragment_str<Builder: HtmlTreeBuilder>(source: impl AsRef<str>, builder: &mut Builder) -> ParserPayload<Vec<Builder::Output>> {
    use html5ever::tendril::TendrilSink;
    let mut input_reader = std::io::Cursor::new(source.as_ref());
    let dom: rcdom::RcDom = html5ever::parse_fragment(
        rcdom::RcDom::default(),
        Default::default(),
        markup5ever::QualName::new(None, html5ever::ns!(html), html5ever::local_name!("")),
        vec![]
    )
    .from_utf8()
    .read_from(&mut input_reader)
    .unwrap();

    let errors = dom.errors
        .borrow()
        .iter()
        .map(|error| {
            error.to_string()
        })
        .collect::<Vec<_>>();
    ParserPayload {
        errors,
        output: to_output_format(&dom.document, builder),
    }
}

fn to_output_format<Builder: HtmlTreeBuilder>(node: &rcdom::Node, builder: &mut Builder) -> Vec<Builder::Output> {
    let children = node.children
        .borrow()
        .iter()
        .flat_map(|node| {
            to_output_format(node, builder)
        })
        .collect::<Vec<_>>();
    match &node.data {
        rcdom::NodeData::Document => children,
        rcdom::NodeData::Doctype { .. } => children,
        rcdom::NodeData::Text { contents } => {
            // use html5ever::tendril::TendrilSink;

            let contents = contents.borrow().to_string();
            vec![
                builder.text_node(contents)
            ]
        },
        rcdom::NodeData::Comment { contents } => {
            let contents = contents.escape_default().to_string();
            vec![
                builder.comment_node(contents)
            ]
        },
        rcdom::NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point: _,
        } => {
            let name = name.local.to_string();
            if name.to_lowercase() == "html" {
                return children
            }
            let attrs = attrs
                .borrow()
                .iter()
                .map(|x| {
                    let key = format!("{}", x.name.local);
                    let value = format!("{}", x.value);
                    ( key, value )
                })
                .collect::<Vec<_>>();
            let mut children = children;
            let template_contents = template_contents
                .borrow()
                .as_ref()
                .map(|x| {
                    to_output_format(x, builder)
                })
                .unwrap_or_default();
            children.extend(template_contents);
            vec![
                builder.element_node(name, attrs, children)
            ]
        },
        rcdom::NodeData::ProcessingInstruction { .. } => unimplemented!(),
    }
}

