pub type Number = f32;
pub type Integer = isize;

// ————————————————————————————————————————————————————————————————————————————
// PROMPT ELEMENT ATTRIBUTE TYPES
// ————————————————————————————————————————————————————————————————————————————

use std::str::FromStr;

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
// PROMPT ELEMENT ATTRIBUTE TYPES - SPECIAL
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
