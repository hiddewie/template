use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum TemplateRenderError {
    UnknownFunctionError(String),
    TypeError(String),
    ArgumentValueError(String),
    LiteralParseError(String),
    RequiredArgumentMissing(String),
    InvalidRegexError(String),
    JsonParseError(String),
    JsonSerializationError,
}

impl Display for TemplateRenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateRenderError::UnknownFunctionError(string) => f.write_str(format!("Unknown function '{}'", string.as_str()).as_str())?,
            TemplateRenderError::TypeError(string) => f.write_str(format!("Invalid type '{}'", string.as_str()).as_str())?,
            TemplateRenderError::ArgumentValueError(string) => f.write_str(format!("Invalid arguments: {}", string.as_str()).as_str())?,
            TemplateRenderError::LiteralParseError(string) => f.write_str(format!("Could not parse literal '{}'", string.as_str()).as_str())?,
            TemplateRenderError::RequiredArgumentMissing(string) => f.write_str(format!("Required argument is missing for function {}", string.as_str()).as_str())?,
            TemplateRenderError::InvalidRegexError(regex) => f.write_str(format!("Invalid regular expression given: '{}'", regex.as_str()).as_str())?,
            TemplateRenderError::JsonParseError(json) => f.write_str(format!("Could not parse JSON: '{}'", json.as_str()).as_str())?,
            TemplateRenderError::JsonSerializationError => f.write_str(format!("Could not serialize JSON").as_str())?,
        }
        return Ok(());
    }
}