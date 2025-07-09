use std::{ops::Not, rc::Rc, str::FromStr};

pub mod attribute_types;


// ————————————————————————————————————————————————————————————————————————————
// CONTEXT
// ————————————————————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub struct XmlDslContext {}

// ————————————————————————————————————————————————————————————————————————————
// ERROR HANDLING
// ————————————————————————————————————————————————————————————————————————————

pub trait DslFormatError: std::error::Error + std::fmt::Debug {
    fn singleton(&self) -> DslFormatErrorList;
}

#[derive(Debug, Clone)]
pub struct DslFormatErrorList {
    pub errors: Vec<Rc<dyn DslFormatError>>,
}

impl DslFormatErrorList {
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn with_capacity(len: usize) -> Self {
        Self { errors: Vec::<_>::with_capacity(len) }
    }
    pub fn new(item: Rc<dyn DslFormatError>) -> Self {
        let errors = vec![ item ];
        Self { errors }
    }
    pub fn union(mut self, other: Self) -> Self {
        self.errors.extend(other.errors);
        self
    }
    pub fn push<T: DslFormatError + 'static>(&mut self, new: Rc<T>) {
        self.errors.push(new);
    }
    pub fn extend(&mut self, other: Self) {
        self.errors.extend(other.errors);
    }
    pub fn join<T: DslFormatError + 'static>(mut self, next: Rc<T>) {
        self.errors.push(next);
    }
    pub fn joined(&self, separator: impl AsRef<str>) -> String {
        self.errors
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(separator.as_ref())
    }
}

impl std::fmt::Display for DslFormatErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.joined(" ∙ ");
        write!(f, "{}", items)
    }
}

impl<T: DslFormatError> From<T> for DslFormatErrorList {
    fn from(value: T) -> Self {
        value.singleton()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// BASICS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    Assistant,
    User,
}

impl Role {
    pub fn is_system(&self) -> bool {
        match self {
            Self::System => true,
            _ => false,
        }
    }
    pub fn is_assistant(&self) -> bool {
        match self {
            Self::Assistant => true,
            _ => false,
        }
    }
    pub fn is_user(&self) -> bool {
        match self {
            Self::User => true,
            _ => false,
        }
    }
}

impl FromStr for Role {
    type Err = ParseRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "system" => Ok(Role::System),
            "assistant" => Ok(Role::Assistant),
            "user" => Ok(Role::User),
            _ => Err(ParseRoleError { given: s.to_string() })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseRoleError {
    pub given: String,
}

impl std::fmt::Display for ParseRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unrecognized role {:?}", self.given)
    }
}

impl std::error::Error for ParseRoleError {}
impl DslFormatError for ParseRoleError {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}


// ————————————————————————————————————————————————————————————————————————————
// MESSAGE ATTRIBUTES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct MessageAttributes {
    pub role: Role,
}

impl MessageAttributes {
    fn from_attribute_map(attributes: xml_ast::AttributeMap) -> Result<Self, DslFormatErrorList> {
        let role = attributes
            .get("role")
            .ok_or_else(|| {
                InvalidMessageAttributes
            })?;
        let role = Role::from_str(role.as_str())?;
        Ok(Self { role })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageAttributes;

impl std::fmt::Display for InvalidMessageAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message attributes")
    }
}

impl std::error::Error for InvalidMessageAttributes {}
impl DslFormatError for InvalidMessageAttributes {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// MESSAGE CONTENT NODE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum ContentNode {
    Text(String),
}

impl ContentNode {
    pub fn text_contents(&self) -> String {
        match self {
            Self::Text(text) => text.to_owned(),
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// MESSAGE CONTENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct MessageContent {
    pub content: String,
}

impl MessageContent {
    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into() }
    }
    fn from_fragment(node: xml_ast::Fragment) -> Result<Self, DslFormatErrorList> {
        let content = node
            .extract_text_strict()
            .map_err(|_| InvalidMessageContent)?
            .join("");
        Ok(Self { content })
    }
    pub fn to_string(&self) -> String {
        self.content.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageContent;

impl std::fmt::Display for InvalidMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message content")
    }
}

impl std::error::Error for InvalidMessageContent {}
impl DslFormatError for InvalidMessageContent {
    fn singleton(&self) -> DslFormatErrorList {
        DslFormatErrorList::new(Rc::new(self.clone()))
    }
}

// ————————————————————————————————————————————————————————————————————————————
// MESSAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct MessageElement {
    pub attributes: MessageAttributes,
    pub content: MessageContent,
}

impl MessageElement {
    fn tag_type() -> xml_ast::TagBuf {
        xml_ast::TagBuf::new("message")
    }
    fn matches(tag: &xml_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
    fn from_element(element: xml_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidMessage)))
        }
        let attributes = MessageAttributes::from_attribute_map(element.attributes)?;
        let content = MessageContent::from_fragment(element.children)?;
        Ok(Self {
            attributes,
            content,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMessage;

impl std::fmt::Display for InvalidMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message format")
    }
}

impl std::error::Error for InvalidMessage {}
impl DslFormatError for InvalidMessage {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}


// ————————————————————————————————————————————————————————————————————————————
// MESSAGE BREAKPOINT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct MessageBreakpointElement {
    pub attributes: MessageAttributes,
    pub content: MessageContent,
}

impl MessageBreakpointElement {
    fn tag_type() -> xml_ast::TagBuf {
        xml_ast::TagBuf::new("message-breakpoint")
    }
    fn matches(tag: &xml_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
    fn from_element(element: xml_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidMessageBreakpoint)))
        }
        let attributes = MessageAttributes::from_attribute_map(element.attributes)?;
        let content = MessageContent::from_fragment(element.children)?;
        Ok(Self {
            attributes,
            content,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageBreakpoint;

impl std::fmt::Display for InvalidMessageBreakpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message format")
    }
}

impl std::error::Error for InvalidMessageBreakpoint {}
impl DslFormatError for InvalidMessageBreakpoint {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}


// ————————————————————————————————————————————————————————————————————————————
// PROMPT ATTRIBUTES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct PromptAttributes {
    pub name: String,
    pub model: Option<attribute_types::Model>,
    pub stream: Option<attribute_types::Stream>,
    pub temperature: Option<attribute_types::Temperature>,
    pub n: Option<attribute_types::N>,
    pub max_tokens: Option<attribute_types::MaxTokens>,
    pub top_p: Option<attribute_types::TopP>,
    pub frequency_penalty: Option<attribute_types::FrequencyPenalty>,
    pub presence_penalty: Option<attribute_types::PresencePenalty>,
    pub logprobs: Option<attribute_types::Logprobs>,
    pub top_logprobs: Option<attribute_types::TopLogprobs>,
    pub response_format: Option<attribute_types::ResponseFormat>,
}

impl PromptAttributes {
    fn from_attribute_map(attributes: xml_ast::AttributeMap) -> Result<Self, DslFormatErrorList> {
        let name = attributes
            .get("name")
            .map(|x| x.as_str().to_string())
            .ok_or_else(|| InvalidPromptAttributes::MissingName)?;

        let model= attributes
            .get("model")
            .map(|x| {
                attribute_types::Model::from_str(x.as_str())
            });
        let stream= attributes
            .get("stream")
            .map(|x| {
                attribute_types::Stream::from_str(x.as_str())
            });
        let temperature= attributes
            .get("temperature")
            .map(|x| {
                attribute_types::Temperature::from_str(x.as_str())
            });
        let n= attributes
            .get("n")
            .map(|x| {
                attribute_types::N::from_str(x.as_str())
            });
        let max_tokens= attributes
            .get("max-tokens")
            .map(|x| {
                attribute_types::MaxTokens::from_str(x.as_str())
            });
        let top_p= attributes
            .get("top-p")
            .map(|x| {
                attribute_types::TopP::from_str(x.as_str())
            });
        let frequency_penalty= attributes
            .get("frequency-penalty")
            .map(|x| {
                attribute_types::FrequencyPenalty::from_str(x.as_str())
            });
        let presence_penalty= attributes
            .get("presence-penalty")
            .map(|x| {
                attribute_types::PresencePenalty::from_str(x.as_str())
            });
        let logprobs= attributes
            .get("logprobs")
            .map(|x| {
                attribute_types::Logprobs::from_str(x.as_str())
            });
        let top_logprobs= attributes
            .get("top-logprobs")
            .map(|x| {
                attribute_types::TopLogprobs::from_str(x.as_str())
            });
        let response_format= attributes
            .get("response-format")
            .map(|x| {
                attribute_types::ResponseFormat::from_str(x.as_str())
            });
        
        let model = match model {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidModelAttribute(x)
                })?
            }),
        };
        let stream = match stream {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidStreamAttribute(x)
                })?
            }),
        };
        let temperature = match temperature {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidTemperatureAttribute(x)
                })?
            }),
        };
        let n = match n {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidNAttribute(x)
                })?
            }),
        };
        let max_tokens = match max_tokens {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidMaxTokensAttribute(x)
                })?
            }),
        };
        let top_p = match top_p {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidTopPAttribute(x)
                })?
            }),
        };
        let frequency_penalty = match frequency_penalty {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidFrequencyPenaltyAttribute(x)
                })?
            }),
        };
        let presence_penalty = match presence_penalty {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidPresencePenaltyAttribute(x)
                })?
            }),
        };
        let logprobs = match logprobs {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidLogprobsAttribute(x)
                })?
            }),
        };
        let top_logprobs = match top_logprobs {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidTopLogprobsAttribute(x)
                })?
            }),
        };
        let response_format = match response_format {
            None => None,
            Some(x) => Some({
                x.map_err(|x| {
                    InvalidPromptAttributes::InvalidResponseFormatAttribute(x)
                })?
            }),
        };

        Ok(Self {
            name,
            model,
            stream,
            temperature,
            n,
            max_tokens,
            top_p,
            frequency_penalty,
            presence_penalty,
            logprobs,
            top_logprobs,
            response_format,
        })
    }
}

#[derive(Debug, Clone)]
pub enum InvalidPromptAttributes {
    MissingName,
    InvalidModelAttribute(attribute_types::InvalidModelAttribute),
    InvalidStreamAttribute(attribute_types::InvalidStreamAttribute),
    InvalidTemperatureAttribute(attribute_types::InvalidTemperatureAttribute),
    InvalidNAttribute(attribute_types::InvalidNAttribute),
    InvalidMaxTokensAttribute(attribute_types::InvalidMaxTokensAttribute),
    InvalidTopPAttribute(attribute_types::InvalidTopPAttribute),
    InvalidFrequencyPenaltyAttribute(attribute_types::InvalidFrequencyPenaltyAttribute),
    InvalidPresencePenaltyAttribute(attribute_types::InvalidPresencePenaltyAttribute),
    InvalidLogprobsAttribute(attribute_types::InvalidLogprobsAttribute),
    InvalidTopLogprobsAttribute(attribute_types::InvalidTopLogprobsAttribute),
    InvalidResponseFormatAttribute(attribute_types::InvalidResponseFormatAttribute),
}

impl std::fmt::Display for InvalidPromptAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingName => write!(f, "missing prompt name"),
            Self::InvalidModelAttribute(x) => x.fmt(f),
            Self::InvalidStreamAttribute(x) => x.fmt(f),
            Self::InvalidTemperatureAttribute(x) => x.fmt(f),
            Self::InvalidNAttribute(x) => x.fmt(f),
            Self::InvalidMaxTokensAttribute(x) => x.fmt(f),
            Self::InvalidTopPAttribute(x) => x.fmt(f),
            Self::InvalidFrequencyPenaltyAttribute(x) => x.fmt(f),
            Self::InvalidPresencePenaltyAttribute(x) => x.fmt(f),
            Self::InvalidLogprobsAttribute(x) => x.fmt(f),
            Self::InvalidTopLogprobsAttribute(x) => x.fmt(f),
            Self::InvalidResponseFormatAttribute(x) => x.fmt(f),
        }
    }
}

impl std::error::Error for InvalidPromptAttributes {}
impl DslFormatError for InvalidPromptAttributes {
    fn singleton(&self) -> DslFormatErrorList {
        DslFormatErrorList::new(Rc::new(self.clone()))
    }
}



// ————————————————————————————————————————————————————————————————————————————
// PROMPT CHILD NODES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum PromptChild {
    Message(MessageElement),
    MessageBreakpoint(MessageBreakpointElement),
}

impl PromptChild {
    pub fn as_message(&self) -> Option<&MessageElement> {
        match self {
            Self::Message(x) => Some(x),
            _ => None,
        }
    }
    pub fn as_message_breakpoint(&self) -> Option<&MessageBreakpointElement> {
        match self {
            Self::MessageBreakpoint(x) => Some(x),
            _ => None,
        }
    }
}

impl PromptChild {
    pub fn from_element(element: xml_ast::Element) -> Result<Self, DslFormatErrorList> {
        if MessageElement::matches(&element.tag) {
            return MessageElement::from_element(element).map(PromptChild::Message)
        }
        if MessageBreakpointElement::matches(&element.tag) {
            return MessageBreakpointElement::from_element(element).map(PromptChild::MessageBreakpoint)
        }
        Err(DslFormatErrorList::new(Rc::new(InvalidPromptChild)))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPromptChild;

impl std::fmt::Display for InvalidPromptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt child")
    }
}

impl std::error::Error for InvalidPromptChild {}
impl DslFormatError for InvalidPromptChild {
    fn singleton(&self) -> DslFormatErrorList {
        DslFormatErrorList::new(Rc::new(self.clone()))
    }
}

// ————————————————————————————————————————————————————————————————————————————
// PROMPT TAG
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct PromptElement {
    pub attributes: PromptAttributes,
    pub children: Vec<PromptChild>,
}

impl PromptElement {
    pub fn name(&self) -> &str {
        &self.attributes.name
    }
}

impl PromptElement {
    fn tag_type() -> xml_ast::TagBuf {
        xml_ast::TagBuf::new("prompt")
    }
    fn matches(tag: &xml_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
    fn from_element(element: xml_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidPrompt)))
        }
        let attributes = PromptAttributes::from_attribute_map(element.clone().attributes)?;
        let mut errors = DslFormatErrorList::with_capacity(element.children.len());
        let mut items = Vec::<PromptChild>::with_capacity(element.children.len());
        for child in element.extract_child_elements() {
            match PromptChild::from_element(child) {
                Ok(item) => {
                    items.push(item);
                }
                Err(error) => {
                    errors.extend(error);
                }
            }
        }
        Ok(PromptElement { attributes, children: items })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPrompt;

impl std::fmt::Display for InvalidPrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt format")
    }
}

impl std::error::Error for InvalidPrompt {}
impl DslFormatError for InvalidPrompt {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT ITEM
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum DocumentItem {
    Prompt(PromptElement),
}

impl DocumentItem {
    pub fn as_prompt(&self) -> Option<&PromptElement> {
        match self {
            Self::Prompt(x) => Some(x),
        }
    }
}

impl DocumentItem {
    fn from_element(element: xml_ast::Element) -> Result<Self, DslFormatErrorList> {
        if PromptElement::matches(&element.tag) {
            return PromptElement::from_element(element).map(DocumentItem::Prompt)
        }
        Err(DslFormatErrorList::new(Rc::new(InvalidDocumentItem)))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDocumentItem;

impl std::fmt::Display for InvalidDocumentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid document item")
    }
}

impl std::error::Error for InvalidDocumentItem {}
impl DslFormatError for InvalidDocumentItem {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct Document {
    pub items: Vec<DocumentItem>,
}

impl Document {
    pub fn lookup_prompt_where(&self, predicate: impl Fn(&PromptElement) -> bool) -> Option<&PromptElement> {
        self.items
            .iter()
            .filter_map(|x| x.as_prompt())
            .find(|x| predicate(x))
    }
    pub fn lookup_prompt(&self, name: impl AsRef<str>) -> Option<&PromptElement> {
        self.lookup_prompt_where(|x| x.name() == name.as_ref())
    }
}

impl Document {
    pub fn from_fragment(fragment: xml_ast::Fragment) -> Result<Self, DslFormatErrorList> {
        let elements = fragment.extract_elements();
        let mut items = Vec::<DocumentItem>::with_capacity(elements.len());
        let mut errors = DslFormatErrorList::with_capacity(elements.len());
        for element in elements {
            match DocumentItem::from_element(element) {
                Ok(item) => {
                    items.push(item);
                },
                Err(error) => {
                    errors.extend(error);
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors)
        }
        Ok(Self { items })
    }
    pub fn from_node(node: xml_ast::Node) -> Result<Self, DslFormatErrorList> {
        match node {
            xml_ast::Node::Element(x) => Ok(Self { items: vec![
                DocumentItem::from_element(x)?
            ]}),
            xml_ast::Node::Fragment(fragment) => Self::from_fragment(fragment),
            _ => Err(DslFormatErrorList::new(Rc::new(InvalidDocument))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDocument;

impl std::fmt::Display for InvalidDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid document")
    }
}

impl std::error::Error for InvalidDocument {}
impl DslFormatError for InvalidDocument {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// LOAD
// ————————————————————————————————————————————————————————————————————————————

// pub fn parse_load(source: impl AsRef<str>) -> Result<Document, DslFormatErrorList> {
//     let xml_ast::ParserPayload {output, errors} = xml_ast::parse_fragment_str(source);
//     if !errors.is_empty() {
//         return Err(DslFormatErrorList::new(Rc::new(ParserErrors { errors })))
//     }
//     Document::from_node(output)
// }

