use std::{path::Path, rc::Rc};

// fn path_to_uri(path: &std::path::Path) -> String {
//     use std::os::unix::ffi::OsStrExt;

//     const SEGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
//         .add(b' ')
//         .add(b'"')
//         .add(b'<')
//         .add(b'>')
//         .add(b'`')
//         .add(b'#')
//         .add(b'?')
//         .add(b'{')
//         .add(b'}')
//         .add(b'/')
//         .add(b'%');

//     let path = path.canonicalize().expect("Failed to canonicalise path");

//     let mut result = "file://".to_owned();

//     const CUSTOM_SEGMENT: &percent_encoding::AsciiSet = &SEGMENT.add(b'\\');
//     for component in path.components().skip(1) {
//         result.push('/');
//         result.extend(percent_encoding::percent_encode(
//             component.as_os_str().as_bytes(),
//             CUSTOM_SEGMENT,
//         ));
//     }
//     result
// }

#[derive(Clone, Copy, Debug)]
pub enum Draft {
    Draft4,
    Draft6,
    Draft7,
    Draft201909,
    Draft202012,
}

impl From<Draft> for jsonschema::Draft {
    fn from(d: Draft) -> jsonschema::Draft {
        match d {
            Draft::Draft4 => jsonschema::Draft::Draft4,
            Draft::Draft6 => jsonschema::Draft::Draft6,
            Draft::Draft7 => jsonschema::Draft::Draft7,
            Draft::Draft201909 => jsonschema::Draft::Draft201909,
            Draft::Draft202012 => jsonschema::Draft::Draft202012,
        }
    }
}

// #[derive(Debug, Clone)]
pub enum JsonSchemaValidatorResult {
    Ok,
    Error(JsonError),
}

#[derive(Debug)]
pub enum JsonError {
    Syntax(serde_json::Error),
    Schema(Vec<jsonschema::ValidationError<'static>>),
}

impl JsonError {}

fn parse_json(json_source: impl AsRef<str>) -> Result<serde_json::Value, JsonError> {
    let value = serde_json::from_str::<serde_json::Value>(json_source.as_ref());
    let value = match value {
        Ok(x) => x,
        Err(error) => {
            let error = JsonError::Syntax(error);
            return Err(error)
        }
    };
    Ok(value)
}

fn build_validator(
    json_source: impl AsRef<str>,
    force_draft: Option<Draft>,
    assert_format: Option<bool>,
) -> Result<jsonschema::Validator, JsonError> {
    let json_value = parse_json(json_source)?;
    // let base_uri = path_to_uri(schema_path);
    // let base_uri = referencing::uri::from_str(&base_uri)?;
    let mut options = jsonschema::options().with_base_uri(".");
    if let Some(draft) = force_draft {
        options = options.with_draft(draft.into());
    }
    if let Some(assert_format) = assert_format {
        options = options.should_validate_formats(assert_format);
    }
    match options.build(&json_value) {
        Ok(x) => {
            Ok(x)
        },
        Err(error) => {
            Err(JsonError::Schema(vec![error]))
        }
    }
}

fn evaluate(validator: &jsonschema::Validator, instance_json: impl AsRef<str>) -> Result<(), JsonError> {
    let instance_json = parse_json(instance_json.as_ref())?;
    let errors = validator
        .iter_errors(&instance_json)
        .into_iter()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(JsonError::Schema(errors))
    }
    Ok(())
}

fn in_smart_quotes(msg: String) -> String {
    match (msg.contains("\""), msg.contains("'")) {
        (true, true) => format!("‘{msg}’"),
        (false, false) => format!("'{msg}'"),
        (true, false) => format!("'{msg}'"),
        (false, true) => format!("\"{msg}\""),
    }
}

fn format_json_schema_validation_error(error: &jsonschema::ValidationError<'static>) -> String {
    format!("{error}")
}

fn format_json_schema_validation_error_list(errors: &[jsonschema::ValidationError<'static>]) -> String {
    errors
        .into_iter()
        .map(|x| format_json_schema_validation_error(x))
        .collect::<Vec<_>>()
        .join("\n")
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax(error) => {
                write!(f, "JSON Syntax Error: {}", in_smart_quotes(error.to_string()))
            }
            Self::Schema(error) => {
                write!(f, "JSON Schema Validation Error: {}", in_smart_quotes(format_json_schema_validation_error_list(error)))
            }
        }
    }
}

impl std::error::Error for JsonError {}


#[derive(Debug)]
pub enum ValidatorBuilderError {
    MissingSchema,
    MissingInstance,
    SchemaError(JsonError),
}

impl std::fmt::Display for ValidatorBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInstance => write!(f, "missing instance"),
            Self::MissingSchema => write!(f, "missing schema"),
            Self::SchemaError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ValidatorBuilderError {}

#[derive(Debug, Clone, Default)]
pub struct ValidatorBuilder {
    pub schema: Option<String>,
    pub instance: Option<String>,
    pub force_draft: Option<Draft>,
    pub assert_format: Option<bool>,
}

impl ValidatorBuilder {
    pub fn with_schema_file(mut self, path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let schema = std::fs::read_to_string(path.as_ref())?;
        self.schema = Some(schema);
        Ok(self)
    }
    pub fn with_schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = Some(schema.into());
        self
    }
    pub fn and_force_draft(mut self, force_draft: Draft) -> Self {
        self.force_draft = Some(force_draft);
        self
    }
    pub fn and_assert_format(mut self, assert_format: bool) -> Self {
        self.assert_format = Some(assert_format);
        self
    }
    pub fn build(self) -> Result<Validator, ValidatorBuilderError> {
        let validator = build_validator(
            self.schema.ok_or(ValidatorBuilderError::MissingSchema)?,
            self.force_draft,
            self.assert_format,
        ).map_err(|e| ValidatorBuilderError::SchemaError(e))?;
        let validator = Rc::new(validator);
        Ok(Validator {
            validator,
            instance: self.instance.ok_or(ValidatorBuilderError::MissingInstance)?,
        })
    }
}

pub struct Validator {
    validator: Rc<jsonschema::Validator>,
    instance: String,
}

impl Validator {
    pub fn validate(&self) -> JsonSchemaValidatorResult {
        let result = evaluate(&self.validator, &self.instance);
        match result {
            Ok(()) => JsonSchemaValidatorResult::Ok,
            Err(error) => JsonSchemaValidatorResult::Error(error),
        }
    }
}


// fn validate_str(instance: impl AsRef<str>) -> Result<JsonSchemaValidatorResult, JsonError> {
//     unimplemented!("TODO")
// }
// fn validate_json_file(path: impl AsRef<std::path::Path>) -> Result<JsonSchemaValidatorResult, Box<dyn std::error::Error>> {
//     unimplemented!("TODO")
// }
