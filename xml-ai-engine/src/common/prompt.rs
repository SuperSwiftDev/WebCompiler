use std::str::FromStr;
use super::{Integer, Number};

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE TYPES
// ————————————————————————————————————————————————————————————————————————————


#[derive(Debug, Clone)]
pub struct Model(pub String);
#[derive(Debug, Clone)]
pub struct Stream(pub String);
#[derive(Debug, Clone)]
pub struct Temperature(pub Number);
#[derive(Debug, Clone)]
pub struct N(pub Integer);
#[derive(Debug, Clone)]
pub struct MaxTokens(pub Integer);
#[derive(Debug, Clone)]
pub struct TopP(pub Number);
#[derive(Debug, Clone)]
pub struct FrequencyPenalty(pub Number);
#[derive(Debug, Clone)]
pub struct PresencePenalty(pub Number);
#[derive(Debug, Clone)]
pub struct Logprobs(pub bool);
#[derive(Debug, Clone)]
pub struct TopLogprobs(pub Integer);
#[derive(Debug, Clone)]
pub struct ResponseFormat(pub ResponseFormatType);

impl Model {
    // ... TODO ...
}
impl Stream {
    // ... TODO ...
}
impl Temperature {
    // ... TODO ...
}
impl N {
    // ... TODO ...
}
impl MaxTokens {
    // ... TODO ...
}
impl TopP {
    // ... TODO ...
}
impl FrequencyPenalty {
    // ... TODO ...
}
impl PresencePenalty {
    // ... TODO ...
}
impl Logprobs {
    // ... TODO ...
}
impl TopLogprobs {
    // ... TODO ...
}
impl ResponseFormat {
    // ... TODO ...
}

impl FromStr for Model {
    type Err = InvalidModelAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl FromStr for Stream {
    type Err = InvalidStreamAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl FromStr for Temperature {
    type Err = InvalidTemperatureAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Number::from_str(s).map_err(|_| InvalidTemperatureAttribute)?))
    }
}
impl FromStr for N {
    type Err = InvalidNAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Integer::from_str(s).map_err(|_| InvalidNAttribute)?))
    }
}
impl FromStr for MaxTokens {
    type Err = InvalidMaxTokensAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Integer::from_str(s).map_err(|_| InvalidMaxTokensAttribute)?))
    }
}
impl FromStr for TopP {
    type Err = InvalidTopPAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Number::from_str(s).map_err(|_| InvalidTopPAttribute)?))
    }
}
impl FromStr for FrequencyPenalty {
    type Err = InvalidFrequencyPenaltyAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Number::from_str(s).map_err(|_| InvalidFrequencyPenaltyAttribute)?))
    }
}
impl FromStr for PresencePenalty {
    type Err = InvalidPresencePenaltyAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Number::from_str(s).map_err(|_| InvalidPresencePenaltyAttribute)?))
    }
}
impl FromStr for Logprobs {
    type Err = InvalidLogprobsAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(bool::from_str(s).map_err(|_| InvalidLogprobsAttribute)?))
    }
}
impl FromStr for TopLogprobs {
    type Err = InvalidTopLogprobsAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Integer::from_str(s).map_err(|_| InvalidTopLogprobsAttribute)?))
    }
}
impl FromStr for ResponseFormat {
    type Err = InvalidResponseFormatAttribute;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(ResponseFormatType::from_str(s).map_err(|_| InvalidResponseFormatAttribute)?))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidModelAttribute;
#[derive(Debug, Clone)]
pub struct InvalidStreamAttribute;
#[derive(Debug, Clone)]
pub struct InvalidTemperatureAttribute;
#[derive(Debug, Clone)]
pub struct InvalidNAttribute;
#[derive(Debug, Clone)]
pub struct InvalidMaxTokensAttribute;
#[derive(Debug, Clone)]
pub struct InvalidTopPAttribute;
#[derive(Debug, Clone)]
pub struct InvalidFrequencyPenaltyAttribute;
#[derive(Debug, Clone)]
pub struct InvalidPresencePenaltyAttribute;
#[derive(Debug, Clone)]
pub struct InvalidLogprobsAttribute;
#[derive(Debug, Clone)]
pub struct InvalidTopLogprobsAttribute;
#[derive(Debug, Clone)]
pub struct InvalidResponseFormatAttribute;

impl std::fmt::Display for InvalidModelAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidModelAttribute")
    }
}
impl std::fmt::Display for InvalidStreamAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidStreamAttribute")
    }
}
impl std::fmt::Display for InvalidTemperatureAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTemperatureAttribute")
    }
}
impl std::fmt::Display for InvalidNAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidNAttribute")
    }
}
impl std::fmt::Display for InvalidMaxTokensAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidMaxTokensAttribute")
    }
}
impl std::fmt::Display for InvalidTopPAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTopPAttribute")
    }
}
impl std::fmt::Display for InvalidFrequencyPenaltyAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidFrequencyPenaltyAttribute")
    }
}
impl std::fmt::Display for InvalidPresencePenaltyAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidPresencePenaltyAttribute")
    }
}
impl std::fmt::Display for InvalidLogprobsAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidLogprobsAttribute")
    }
}
impl std::fmt::Display for InvalidTopLogprobsAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTopLogprobsAttribute")
    }
}
impl std::fmt::Display for InvalidResponseFormatAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidResponseFormatAttribute")
    }
}

impl std::error::Error for InvalidModelAttribute {}
impl std::error::Error for InvalidStreamAttribute {}
impl std::error::Error for InvalidTemperatureAttribute {}
impl std::error::Error for InvalidNAttribute {}
impl std::error::Error for InvalidMaxTokensAttribute {}
impl std::error::Error for InvalidTopPAttribute {}
impl std::error::Error for InvalidFrequencyPenaltyAttribute {}
impl std::error::Error for InvalidPresencePenaltyAttribute {}
impl std::error::Error for InvalidLogprobsAttribute {}
impl std::error::Error for InvalidTopLogprobsAttribute {}
impl std::error::Error for InvalidResponseFormatAttribute {}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE TYPES - SPECIAL
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum ResponseFormatType {
    JsonObject,
    Text,
}

#[derive(Debug, Clone)]
pub struct ParseErrorResponseFormatType;

impl std::fmt::Display for ParseErrorResponseFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid response format type")
    }
}

impl std::error::Error for ParseErrorResponseFormatType {}

impl FromStr for ResponseFormatType {
    type Err = ParseErrorResponseFormatType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s
            .trim()
            .replace("-", "")
            .replace("_", "")
            .to_lowercase();
        match cleaned.as_str() {
            "jsonobject" => Ok(Self::JsonObject),
            "text" => Ok(Self::Text),
            _ => Err(ParseErrorResponseFormatType),
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE TYPES - ERROR TYPES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum InvalidAttribute {
    Name,
    Model,
    Stream,
    Temperature,
    N,
    MaxTokens,
    TopP,
    FrequencyPenalty,
    PresencePenalty,
    Logprobs,
    TopLogprobs,
    ResponseFormat,
}

impl std::fmt::Display for InvalidAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name => write!(f, "invalid `Name` attribute"),
            Self::Model => write!(f, "invalid `Model` attribute"),
            Self::Stream => write!(f, "invalid `Stream` attribute"),
            Self::Temperature => write!(f, "invalid `Temperature` attribute"),
            Self::N => write!(f, "invalid `N` attribute"),
            Self::MaxTokens => write!(f, "invalid `MaxTokens` attribute"),
            Self::TopP => write!(f, "invalid `TopP` attribute"),
            Self::FrequencyPenalty => write!(f, "invalid `FrequencyPenalty` attribute"),
            Self::PresencePenalty => write!(f, "invalid `PresencePenalty` attribute"),
            Self::Logprobs => write!(f, "invalid `Logprobs` attribute"),
            Self::TopLogprobs => write!(f, "invalid `TopLogprobs` attribute"),
            Self::ResponseFormat => write!(f, "invalid `ResponseFormat` attribute"),
        }
    }
}


// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTES PRODUCT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum PromptAttributeEntry {
    Name(String),
    Model(Model),
    Stream(Stream),
    Temperature(Temperature),
    N(N),
    MaxTokens(MaxTokens),
    TopP(TopP),
    FrequencyPenalty(FrequencyPenalty),
    PresencePenalty(PresencePenalty),
    Logprobs(Logprobs),
    TopLogprobs(TopLogprobs),
    ResponseFormat(ResponseFormat),
}

impl PromptAttributeEntry {
    pub fn try_from(key: impl AsRef<str>, value: impl AsRef<str>) -> Option<Result<Self, InvalidAttribute>> {
        match key.as_ref() {
            "name" => {
                Some(Ok(Self::Name(value.as_ref().to_owned())))
            }
            "model" => {
                Some({
                    Model::from_str(value.as_ref())
                        .map(Self::Model)
                        .map_err(|_| InvalidAttribute::Model)
                })
            }
            "stream" => {
                Some({
                    Stream::from_str(value.as_ref())
                        .map(Self::Stream)
                        .map_err(|_| InvalidAttribute::Stream)
                })
            }
            "temperature" => {
                Some({
                    Temperature::from_str(value.as_ref())
                        .map(Self::Temperature)
                        .map_err(|_| InvalidAttribute::Temperature)
                })
            }
            "n" => {
                Some({
                    N::from_str(value.as_ref())
                        .map(Self::N)
                        .map_err(|_| InvalidAttribute::N)
                })
            }
            "max-tokens" => {
                Some({
                    MaxTokens::from_str(value.as_ref())
                        .map(Self::MaxTokens)
                        .map_err(|_| InvalidAttribute::MaxTokens)
                })
            }
            "top-p" => {
                Some({
                    TopP::from_str(value.as_ref())
                        .map(Self::TopP)
                        .map_err(|_| InvalidAttribute::TopP)
                })
            }
            "frequency-penalty" => {
                Some({
                    FrequencyPenalty::from_str(value.as_ref())
                        .map(Self::FrequencyPenalty)
                        .map_err(|_| InvalidAttribute::FrequencyPenalty)
                })
            }
            "presence-penalty" => {
                Some({
                    PresencePenalty::from_str(value.as_ref())
                        .map(Self::PresencePenalty)
                        .map_err(|_| InvalidAttribute::PresencePenalty)
                })
            }
            "logprobs" => {
                Some({
                    Logprobs::from_str(value.as_ref())
                        .map(Self::Logprobs)
                        .map_err(|_| InvalidAttribute::Logprobs)
                })
            }
            "top-logprobs" => {
                Some({
                    TopLogprobs::from_str(value.as_ref())
                        .map(Self::TopLogprobs)
                        .map_err(|_| InvalidAttribute::TopLogprobs)
                })
            }
            "response-format" => {
                Some({
                    ResponseFormat::from_str(value.as_ref())
                        .map(Self::ResponseFormat)
                        .map_err(|_| InvalidAttribute::ResponseFormat)
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromptSettingsBuilder {
    pub name: Option<String>,
    pub model: Option<Model>,
    pub stream: Option<Stream>,
    pub temperature: Option<Temperature>,
    pub n: Option<N>,
    pub max_tokens: Option<MaxTokens>,
    pub top_p: Option<TopP>,
    pub frequency_penalty: Option<FrequencyPenalty>,
    pub presence_penalty: Option<PresencePenalty>,
    pub logprobs: Option<Logprobs>,
    pub top_logprobs: Option<TopLogprobs>,
    pub response_format: Option<ResponseFormat>,
}

impl PromptSettingsBuilder {
    pub fn try_merge(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Option<Result<(), InvalidAttribute>> {
        match PromptAttributeEntry::try_from(key.as_ref(), value.as_ref()) {
            Some(Ok(PromptAttributeEntry::Name(value))) => {
                self.name = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::Model(value))) => {
                self.model = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::Stream(value))) => {
                self.stream = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::Temperature(value))) => {
                self.temperature = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::N(value))) => {
                self.n = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::MaxTokens(value))) => {
                self.max_tokens = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::TopP(value))) => {
                self.top_p = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::FrequencyPenalty(value))) => {
                self.frequency_penalty = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::PresencePenalty(value))) => {
                self.presence_penalty = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::Logprobs(value))) => {
                self.logprobs = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::TopLogprobs(value))) => {
                self.top_logprobs = Some(value);
                Some(Ok(()))
            }
            Some(Ok(PromptAttributeEntry::ResponseFormat(value))) => {
                self.response_format = Some(value);
                Some(Ok(()))
            }
            Some(Err(error)) => Some(Err(error)),
            None => None,
        }
    }
    pub fn build(self) -> Option<PromptSettings> {
        Some(PromptSettings {
            name: self.name?,
            model: self.model,
            stream: self.stream,
            temperature: self.temperature,
            n: self.n,
            max_tokens: self.max_tokens,
            top_p: self.top_p,
            frequency_penalty: self.frequency_penalty,
            presence_penalty: self.presence_penalty,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            response_format: self.response_format,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromptSettings {
    pub name: String,
    pub model: Option<Model>,
    pub stream: Option<Stream>,
    pub temperature: Option<Temperature>,
    pub n: Option<N>,
    pub max_tokens: Option<MaxTokens>,
    pub top_p: Option<TopP>,
    pub frequency_penalty: Option<FrequencyPenalty>,
    pub presence_penalty: Option<PresencePenalty>,
    pub logprobs: Option<Logprobs>,
    pub top_logprobs: Option<TopLogprobs>,
    pub response_format: Option<ResponseFormat>,
}

impl PromptSettings {}
