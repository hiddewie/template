use std::ops::Index;

use chrono::{DateTime, Local};
use itertools::Itertools;
use regex::Regex;
use serde_json::{Map, Value};

use crate::error::TemplateRenderError;

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn to_boolean(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(bool) => *bool,
        Value::Number(number) => {
            if number.is_i64() {
                number.as_i64().unwrap() != 0
            } else if number.is_u64() {
                number.as_u64().unwrap() != 0
            } else if number.is_f64() {
                number.as_f64().unwrap() != 0.0
            } else {
                unreachable!();
            }
        }
        Value::String(string) => !string.trim().is_empty(),
        Value::Array(items) => !items.is_empty(),
        Value::Object(object) => !object.is_empty(),
    }
}

fn kebab_case(value: &String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9_]+").unwrap();
    let lower = value.to_lowercase();
    return re.replace_all(lower.as_str(), "-").to_string();
}

fn snake_case(value: &String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9-]+").unwrap();
    let lower = value.to_lowercase();
    return re.replace_all(lower.as_str(), "_").to_string();
}

fn camel_case(value: &String) -> String {
    let mut result = String::new();
    let mut to_upper = false;
    for c in value.chars() {
        if c.is_alphanumeric() {
            result.push(if to_upper { c.to_ascii_uppercase() } else { c });
            to_upper = false;
        } else {
            to_upper = true;
        }
    }
    return result;
}

fn pascal_case(value: &String) -> String {
    let mut result = String::new();
    let mut to_upper = true;
    for c in value.chars() {
        if c.is_alphanumeric() {
            result.push(if to_upper { c.to_ascii_uppercase() } else { c });
            to_upper = false;
        } else {
            to_upper = true;
        }
    }
    return result;
}

fn capitalize(value: &String) -> String {
    if value.is_empty() {
        value.as_str().to_string()
    } else {
        let (first, rest) = value.split_at(1);
        format!("{}{}", first.to_uppercase(), rest)
    }
}

fn capitalize_words(value: &String) -> String {
    let mut result = String::new();
    let mut to_upper = true;
    for c in value.chars() {
        if c.is_whitespace() {
            to_upper = true;
            result.push(c);
        } else {
            result.push(if to_upper { c.to_ascii_uppercase() } else { c });
            to_upper = false;
        }
    }
    return result;
}

fn environment(value: &String) -> Option<String> {
    return std::env::var(value.as_str()).ok();
}

fn default(value: &Value, default: &Value) -> Value {
    if to_boolean(value) {
        value.clone()
    } else {
        default.clone()
    }
}

fn require_string_value(value: &Value) -> Result<&String, TemplateRenderError> {
    match value {
        Value::String(string) => Ok(string),
        _ => Err(TemplateRenderError::TypeError(type_of(&value)))
    }
}

fn require_u64_value(value: &Value) -> Result<u64, TemplateRenderError> {
    value.as_u64()
        .ok_or_else(|| TemplateRenderError::TypeError(type_of(&value)))
}

fn require_array_value(value: &Value) -> Result<&Vec<Value>, TemplateRenderError> {
    value.as_array()
        .ok_or_else(|| TemplateRenderError::TypeError(type_of(&value)))
}

fn require_object_value(value: &Value) -> Result<&Map<String, Value>, TemplateRenderError> {
    value.as_object()
        .ok_or_else(|| TemplateRenderError::TypeError(type_of(&value)))
}

fn require_argument<'a>(function: &'a str, arguments: &'a Vec<Value>, index: usize) -> Result<&'a Value, TemplateRenderError> {
    arguments.get(index)
        .ok_or_else(||
            TemplateRenderError::RequiredArgumentMissing(format!("Argument {} is missing for function '{}'", index + 1, function.to_string()))
        )
}

pub fn apply_function(value: &Value, function: &str, arguments: &Vec<Value>) -> Result<Value, TemplateRenderError> {
    return match function {
        "lowerCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(string.to_lowercase()))
        }
        "upperCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(string.to_uppercase()))
        }
        "kebabCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(kebab_case(string)))
        }
        "snakeCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(snake_case(string)))
        }
        "camelCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(camel_case(string)))
        }
        "pascalCase" => {
            let string = require_string_value(value)?;
            Ok(Value::String(pascal_case(string)))
        }
        "capitalize" => {
            let string = require_string_value(value)?;
            Ok(Value::String(capitalize(string)))
        }
        "capitalizeWords" => {
            let string = require_string_value(value)?;
            Ok(Value::String(capitalize_words(string)))
        }
        "length" => {
            match value {
                Value::String(string) => Ok(Value::from(string.len())),
                Value::Array(array) => Ok(Value::from(array.len())),
                Value::Object(dictionary) => Ok(Value::from(dictionary.len())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "environment" => {
            let string = require_string_value(value)?;
            Ok(environment(string)
                .map(|value| Value::from(value))
                .unwrap_or(Value::Null))
        }
        "default" => {
            let default_value = require_argument(function, arguments, 0)?;
            Ok(default(value, &default_value))
        }
        "coalesce" => {
            let default_value = require_argument(function, arguments, 0)?;
            let result = match value {
                Value::Null => default_value,
                _ => value,
            };
            Ok(result.clone())
        }
        "reverse" => {
            match value {
                Value::String(string) => Ok(Value::String(String::from_iter(string.chars().rev()))),
                Value::Array(array) => {
                    let mut reverted = array.clone();
                    reverted.reverse();
                    Ok(Value::Array(reverted))
                }
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "split" => {
            let string = require_string_value(value)?;
            let splitter = require_argument(function, arguments, 0)?;
            let splitter_string = require_string_value(splitter)?;
            let split_strings = string.split(splitter_string).map(|split| Value::String(split.to_string())).collect();
            Ok(Value::Array(split_strings))
        }
        "lines" => {
            let string = require_string_value(value)?;
            let lines: Vec<Value> = string.trim().lines().map(|item| Value::String(item.to_string())).collect();
            Ok(Value::Array(lines))
        }
        "matches" => {
            let string = require_string_value(value)?;
            let regex = require_argument(function, arguments, 0)?;
            let regex_string = require_string_value(regex)?;
            let re = Regex::new(regex_string).map_err(|_err| TemplateRenderError::InvalidRegexError(regex_string.to_string()))?;
            Ok(Value::Bool(re.is_match(string.as_str())))
        }
        "substring" => {
            let string = require_string_value(value)?;
            let from = require_argument(function, arguments, 0)?;
            let from_value = require_u64_value(from)?;
            let to = arguments.get(1);
            if to.is_some() {
                let to_value = require_u64_value(to.unwrap())?;
                Ok(Value::String(string[usize::try_from(from_value).unwrap().max(0).min(string.len())..usize::try_from(to_value).unwrap().max(0).min(string.len())].to_string()))
            } else {
                Ok(Value::String(string[usize::try_from(from_value).unwrap().max(0).min(string.len())..].to_string()))
            }
        }
        "take" => {
            let n = require_argument(function, arguments, 0)?;
            let n_value = require_u64_value(n)?;
            let to_index = usize::try_from(n_value).unwrap();
            match value {
                Value::String(string) => Ok(Value::String(string[..to_index.max(0).min(string.len())].to_string())),
                Value::Array(array) => Ok(Value::Array(array[..to_index.max(0).min(array.len())].to_vec())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "drop" => {
            let n = require_argument(function, arguments, 0)?;
            let n_value = require_u64_value(n)?;
            let from_index = usize::try_from(n_value).unwrap();
            match value {
                Value::String(string) => Ok(Value::String(string[from_index.max(0).min(string.len())..].to_string())),
                Value::Array(array) => Ok(Value::Array(array[from_index.max(0).min(array.len())..].to_vec())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "first" => {
            let array = require_array_value(value)?;
            Ok(array.first().unwrap_or(&Value::Null).clone())
        }
        "last" => {
            let array = require_array_value(value)?;
            Ok(array.last().unwrap_or(&Value::Null).clone())
        }
        "index" => {
            let index = require_argument(function, arguments, 0)?;
            let index_number = require_u64_value(index)?;
            let array = require_array_value(value)?;
            let result = if index_number < array.len() as u64 {
                array.index(index_number as usize).clone()
            } else {
                Value::Null
            };
            Ok(result)
        }
        "contains" => {
            let needle = require_argument(function, arguments, 0)?;
            match value {
                Value::String(substring) => {
                    let needle_string = require_string_value(needle)?;
                    Ok(Value::Bool(substring.contains(needle_string)))
                }
                Value::Array(array) => Ok(Value::Bool(array.contains(needle))),
                _ => Err(TemplateRenderError::TypeError(type_of(&needle))),
            }
        }
        "containsKey" => {
            let key = require_argument(function, arguments, 0)?;
            let object = require_object_value(value)?;
            let key_value = require_string_value(key)?;
            Ok(Value::Bool(object.contains_key(key_value)))
        }
        "containsValue" => {
            let needle = require_argument(function, arguments, 0)?;
            let object = require_object_value(value)?;
            Ok(Value::Bool(object.values().any(|val| val == needle)))
        }
        "startsWith" => {
            let string = require_string_value(value)?;
            let start = require_argument(function, arguments, 0)?;
            let start_string = require_string_value(start)?;
            Ok(Value::Bool(string.starts_with(start_string)))
        }
        "endsWith" => {
            let string = require_string_value(value)?;
            let end = require_argument(function, arguments, 0)?;
            let end_string = require_string_value(end)?;
            Ok(Value::Bool(string.ends_with(end_string)))
        }
        "empty" => {
            Ok(Value::Bool(!to_boolean(value)))
        }
        "unique" => {
            let array = require_array_value(value)?;
            let unique = array.clone().into_iter()
                .unique_by(|item| format!("{item}"))
                .collect::<Vec<_>>();
            Ok(Value::Array(unique))
        }
        "keys" => {
            let object = require_object_value(value)?;
            let keys = object.keys().into_iter().map(|key| Value::String(key.clone())).collect::<Vec<_>>();
            Ok(Value::Array(keys))
        }
        "values" => {
            let object = require_object_value(value)?;
            Ok(Value::Array(object.values().cloned().into_iter().collect::<Vec<_>>()))
        }
        "invert" => {
            let object = require_object_value(value)?;
            if let Some(item) = object.values().into_iter().find(|value| !value.is_string()) {
                Err(TemplateRenderError::TypeError(type_of(&item)))
            } else {
                let inverted = object.clone().into_iter().map(|(key, value)| (value.as_str().unwrap().to_string(), Value::String(key))).collect();
                Ok(Value::Object(inverted))
            }
        }
        "toJson" => {
            let result = serde_json::to_string(value)
                .map_err(|_| TemplateRenderError::JsonSerializationError)?;
            Ok(Value::String(result))
        }
        "toPrettyJson" => {
            let result = serde_json::to_string_pretty(value)
                .map_err(|_| TemplateRenderError::JsonSerializationError)?;
            Ok(Value::String(result))
        }
        "fromJson" => {
            let string = require_string_value(value)?;
            serde_json::from_str(string.as_str())
                .map_err(|error| TemplateRenderError::JsonParseError(error.to_string()))
        }
        "abbreviate" => {
            let n = require_argument(function, arguments, 0)?;
            let string = require_string_value(value)?;
            let n_value = require_u64_value(n)?;
            if string.len() <= n_value as usize {
                Ok(Value::String(string.clone()))
            } else {
                Ok(Value::String(format!("{}…", string.as_str()[0..((n_value.max(1) as usize) - 1)].to_string())))
            }
        }
        "trimLeft" => {
            let string = require_string_value(value)?;
            Ok(Value::String(string.trim_start().to_string()))
        }
        "trimRight" => {
            let string = require_string_value(value)?;
            Ok(Value::String(string.trim_end().to_string()))
        }
        "trim" => {
            let string = require_string_value(value)?;
            Ok(Value::String(string.trim().to_string()))
        }
        "replace" => {
            let string = require_string_value(value)?;
            let search = require_argument(function, arguments, 0)?;
            let search_string = require_string_value(search)?;
            let replacement = require_argument(function, arguments, 1)?;
            let replacement_string = require_string_value(replacement)?;
            Ok(Value::String(string.replace(search_string, replacement_string)))
        }
        "regexReplace" => {
            let string = require_string_value(value)?;
            let regex = require_argument(function, arguments, 0)?;
            let regex_string = require_string_value(regex)?;
            let parsed_regex = Regex::new(regex_string)
                .map_err(|_| TemplateRenderError::InvalidRegexError(regex_string.to_string()))?;
            let replacement = require_argument(function, arguments, 1)?;
            let replacement_string = require_string_value(replacement)?;
            Ok(Value::String(parsed_regex.replace_all(string.as_str(), replacement_string).to_string()))
        }
        "negate" => {
            Ok(Value::Bool(!to_boolean(value)))
        }
        "all" => {
            let result = require_array_value(value)?;
            Ok(Value::Bool(result.into_iter().all(|item| to_boolean(item))))
        }
        "any" => {
            let result = require_array_value(value)?;
            Ok(Value::Bool(result.into_iter().any(|item| to_boolean(item))))
        }
        "none" => {
            let result = require_array_value(value)?;
            Ok(Value::Bool(result.into_iter().all(|item| !to_boolean(item))))
        }
        "some" => {
            let result = require_array_value(value)?;
            Ok(Value::Bool(result.into_iter().any(|item| !to_boolean(item))))
        }
        "chunked" => {
            let array = require_array_value(value)?;
            let chunk_size = require_argument(function, arguments, 0)?;
            let chunk_size_number = require_u64_value(chunk_size)? as usize;
            let overlap = require_argument(function, arguments, 1)?;
            let overlap_number = require_u64_value(overlap)? as usize;

            if overlap_number >= chunk_size_number {
                Err(TemplateRenderError::ArgumentValueError(format!("The overlap ({overlap_number}) cannot be equal or larger than the chunk size ({chunk_size_number})")))
            } else {
                let mut result: Vec<Value> = vec![];

                for i in (0..(array.len())).step_by(chunk_size_number - overlap_number) {
                    // step 3, overlap 1
                    // 0 to 3
                    // 0+3-1 to 0+3-1+3
                    result.push(Value::Array(array[i..(i + chunk_size_number - overlap_number).min(array.len())].to_vec()))
                }
                Ok(Value::Array(result))
            }
        }
        "parseFormatDateTime" => {
            let string = require_string_value(value)?;

            let parse_result = if string == "now" {
                // First argument is ignored
                DateTime::from(Local::now())
            } else {
                let parse_format = require_argument(function, arguments, 0)?;
                let parse_format_string = require_string_value(parse_format)?;
                DateTime::parse_from_str(string, parse_format_string)
                    .map_err(|err| TemplateRenderError::ArgumentValueError(format!("Could not parse date-time with value '{string}' and parse format string '{parse_format_string}': {err}")))?
            };

            let format = require_argument(function, arguments, 1)?;
            let format_string = require_string_value(format)?;
            let formatted = parse_result.format(format_string).to_string();
            Ok(Value::String(formatted))
        }
        "alternate" => {
            let index = require_u64_value(value)?;
            let items = require_argument(function, arguments, 0)?;
            let items_array = require_array_value(items)?;

            if items_array.is_empty() {
                Ok(Value::Null)
            } else {
                let array_index: usize = index.try_into()
                    .map_err(|error| TemplateRenderError::ArgumentValueError(format!("{} cannot be cast to usize, {}", index, error)))?;

                Ok(items_array[array_index % items_array.len()].clone())
            }
        }
        _ => Err(TemplateRenderError::UnknownFunctionError(function.to_string()))
    };
}