extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::cell::RefCell;
use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::process::exit;
use std::rc::Rc;

use clap::Parser as ClapParser;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use regex::Regex;
use serde_json::{Map, Value};
use serde_json::error::Category;

#[derive(Parser)]
#[grammar = "template.pest"]
pub struct TemplateParser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(ClapParser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
    /// Config
    config: std::path::PathBuf,
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
enum TemplateRenderError {
    TypeError(String),
    LiteralParseError(String),
}

impl Display for TemplateRenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateRenderError::TypeError(string) => f.write_str(format!("Invalid type {}", string.as_str()).as_str())?,
            TemplateRenderError::LiteralParseError(string) => f.write_str(format!("Could not parse literal '{}'", string.as_str()).as_str())?,
        }
        return Ok(());
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
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                result.push(if to_upper { c.to_ascii_uppercase() } else { c });
                to_upper = false;
            }
            _ => {
                to_upper = true;
            }
        }
    }
    return result;
}

fn pascal_case(value: &String) -> String {
    let mut result = String::new();
    let mut to_upper = true;
    for c in value.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                result.push(if to_upper { c.to_ascii_uppercase() } else { c });
                to_upper = false;
            }
            _ => {
                to_upper = true;
            }
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
    return env::var(value.as_str()).ok();
}

fn apply_function(value: &Value, function: &str) -> Result<Value, TemplateRenderError> {
    return match function {
        "lowerCase" => {
            match value {
                Value::String(string) => Ok(Value::String(string.to_lowercase())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "upperCase" => {
            match value {
                Value::String(string) => Ok(Value::String(string.to_uppercase())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "kebabCase" => {
            match value {
                Value::String(string) => Ok(Value::String(kebab_case(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "snakeCase" => {
            match value {
                Value::String(string) => Ok(Value::String(snake_case(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "camelCase" => {
            match value {
                Value::String(string) => Ok(Value::String(camel_case(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "pascalCase" => {
            match value {
                Value::String(string) => Ok(Value::String(pascal_case(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "capitalize" => {
            match value {
                Value::String(string) => Ok(Value::String(capitalize(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "capitalizeWords" => {
            match value {
                Value::String(string) => Ok(Value::String(capitalize_words(string))),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "length" => {
            match value {
                Value::String(string) => Ok(Value::from(string.len())),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        "environment" => {
            match value {
                Value::String(string) => Ok(environment(string)
                    .map(|value| Value::from(value))
                    .unwrap_or(Value::Null)),
                _ => Err(TemplateRenderError::TypeError(type_of(&value)))
            }
        }
        _ => unreachable!()
    };
}

fn format_string(value: &Value) -> String {
    match value {
        Value::Null => "".to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(string) => string.to_string(),
        Value::Array(values) => format!("[{}]", values.iter().map(|v| format_string(v)).reduce(|cur, next| format!("{},{}", &cur, &next)).unwrap_or("".to_string())),
        Value::Object(object) => format!("{{{}}}", object.iter().map(|(k, v)| format!("{}:{}", k, format_string(v))).reduce(|cur, next| format!("{},{}", &cur, &next)).unwrap_or("".to_string())),
    }
}

fn parse_literal(value: &Value, literal: &Pair<Rule>) -> Result<Value, TemplateRenderError> {
    let content = literal.as_str().to_string();
    match literal.as_rule() {
        Rule::null => Ok(Value::Null),
        Rule::boolean => content.parse::<bool>()
            .map_err(|_err| TemplateRenderError::LiteralParseError(content))
            .map(|result| Value::from(result)),
        Rule::number => match literal.clone().into_inner().next().unwrap().as_rule() {
            Rule::integer_number => content.parse::<i64>()
                .map_err(|_err| TemplateRenderError::LiteralParseError(content))
                .map(|result| Value::from(result)),
            Rule::floating_point_number => content.parse::<f64>()
                .map_err(|_err| TemplateRenderError::LiteralParseError(content))
                .map(|result| Value::from(result)),
            _ => unreachable!()
        }
        Rule::string => content.parse::<String>()
            .map_err(|_err| TemplateRenderError::LiteralParseError(content))
            .map(|result| Value::from(&result[1..result.len() - 1])),
        Rule::array => {
            let array: Result<Vec<Value>, TemplateRenderError> = literal.clone().into_inner()
                .into_iter()
                .map(|inner_literal| evaluate(&value, &mut inner_literal.into_inner()))
                .collect();

            array.map(|result| Value::Array(result))
        }
        Rule::dictionary => {
            let mut result = Map::new();
            for pair in literal.clone().into_inner() {
                let mut key_value_content = pair.into_inner();
                let pair_key = key_value_content.next().unwrap().as_str();
                let pair_value = key_value_content.next().unwrap();
                let evaluated_pair_value = evaluate(&value, &mut pair_value.into_inner())?;

                result.insert(pair_key.to_string(), evaluated_pair_value);
            }

            Ok(Value::Object(result))
        }
        _ => unreachable!()
    }
}

fn evaluate(value: &Value, expression: &mut Pairs<Rule>) -> Result<Value, TemplateRenderError> {
    let properties_or_literal = expression.next().unwrap();

    let current_value = match properties_or_literal.as_rule() {
        Rule::literal => {
            parse_literal(&value, &properties_or_literal.into_inner().next().unwrap())?
        }
        Rule::properties => {
            let mut current_value = value.clone();
            for property in properties_or_literal.into_inner() {
                match property.as_rule() {
                    Rule::property => {
                        current_value = current_value[property.as_str()].clone();
                    }
                    _ => unreachable!(),
                }
            }
            current_value
        }
        _ => unreachable!()
    };

    let mut result = current_value;
    for function in expression {
        match function.as_rule() {
            Rule::function => {
                result = apply_function(&result, function.as_str())?;
            }
            _ => unreachable!(),
        }
    }
    return Ok(result);
}

fn evaluate_boolean(value: &Value, expression: &mut Pairs<Rule>) -> Result<bool, TemplateRenderError> {
    return evaluate(value, expression)
        .map(|result|
            match result {
                Value::Null => false,
                Value::Bool(bool) => bool,
                Value::Number(number) => number.as_i64().map(|i| i == 0)
                    .or(number.as_u64().map(|u| u == 0))
                    .or(number.as_f64().map(|f| f == 0.0))
                    .unwrap_or(false),
                Value::String(string) => !string.trim().is_empty(),
                Value::Array(items) => !items.is_empty(),
                Value::Object(object) => !object.is_empty(),
            }
        );
}

fn evaluate_template(data: &Value, record: Pair<Rule>) -> Result<String, TemplateRenderError> {
    let mut result = String::new();

    let mut inner_rules = record.into_inner();
    let expression = inner_rules.next().unwrap();

    match expression.as_rule() {
        Rule::if_elif_else_template => {
            let mut done = false;
            let mut valid = false;
            for if_inner in expression.into_inner() {
                match if_inner.as_rule() {
                    Rule::if_template => {
                        valid = false;

                        let mut if_inner_expression = if_inner.into_inner();
                        let if_expression = if_inner_expression.next().unwrap();
                        let if_result = evaluate_boolean(&data, &mut if_expression.into_inner()).unwrap_or(false);
                        if if_result {
                            done = true;
                            valid = true
                        }
                    }
                    Rule::elif_template => {
                        valid = false;

                        let mut elif_inner_expression = if_inner.into_inner();
                        let elif_expression = elif_inner_expression.next().unwrap();
                        let elif_result = evaluate_boolean(&data, &mut elif_expression.into_inner()).unwrap_or(false);
                        if !done && elif_result {
                            done = true;
                            valid = true
                        }
                    }
                    Rule::else_template => {
                        valid = !done;
                    }
                    Rule::end_template => {
                        valid = false;
                        done = true;
                    }
                    Rule::character => {
                        if valid {
                            result.push_str(if_inner.as_str())
                        }
                    }
                    Rule::template => result.push_str(evaluate_template(&data, if_inner)?.as_str()),
                    _ => unreachable!(),
                }
            }
        }
        Rule::for_else_template => {
            let mut valid = false;
            let mut done = false;
            let mut iterable_name: &str = "";
            let mut iterables: Vec<Value> = vec![];
            let mut iterables_results: Vec<Rc<RefCell<String>>> = vec![];

            for for_inner in expression.into_inner() {
                match for_inner.as_rule() {
                    Rule::for_template => {
                        let mut for_inner_expression = for_inner.into_inner();
                        iterable_name = for_inner_expression.next().unwrap().as_str();
                        let for_iterable = evaluate(&data, &mut for_inner_expression.next().unwrap().into_inner()).unwrap();
                        iterables = match for_iterable {
                            Value::Array(items) => items,
                            _ => vec![],
                        };
                        done = iterables.is_empty();
                        valid = !iterables.is_empty();
                        iterables_results = iterables.iter().map(|_| Rc::new(RefCell::new(String::new()))).collect();
                    }
                    Rule::else_template => {
                        valid = false;
                        if !done {
                            valid = true
                        }
                    }
                    Rule::end_template => {
                        valid = false;
                        done = true;
                    }
                    Rule::character => {
                        if valid {
                            iterables_results.iter_mut().for_each(|iterables_result| {
                                iterables_result.replace_with(|r| format!("{}{}", r, for_inner.as_str()));
                            })
                        }
                    }
                    Rule::template => {
                        let zipped = iterables.iter().zip(iterables_results.iter());
                        for (iterable, iterable_result) in zipped {
                            let context_value = match data {
                                Value::Object(map) => {
                                    let mut q = map.clone();
                                    q.insert(iterable_name.to_string(), iterable.clone());
                                    Value::Object(q)
                                }
                                _ => iterable.clone(),
                            };
                            let template_result = evaluate_template(&context_value, for_inner.clone())?;
                            iterable_result.replace_with(|current_result|
                                format!("{}{}", current_result, template_result)
                            );
                        }
                    }
                    _ => unreachable!(),
                }
            }

            // Provide the looped results
            iterables_results.iter().for_each(|iterable_result|
                result.push_str(iterable_result.take().as_str())
            )
        }
        Rule::expression_template => {
            let mut inner_rules = expression.into_inner();
            let expression = inner_rules.next().unwrap();
            let evaluation_result = evaluate(&data, &mut expression.into_inner())?;
            result.push_str(format_string(&evaluation_result).to_string().as_str())
        }
        Rule::comment => (),
        _ => unreachable!(),
    }

    return Ok(result);
}

fn evaluate_file(data: &Value, file: Pair<Rule>) -> Result<String, TemplateRenderError> {
    let mut result = String::new();

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::template => result.push_str(evaluate_template(&data, record)?.as_str()),
            Rule::character => result.push_str(record.as_str()),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    return Ok(result);
}

static ERR_TEMPLATE_FILE: i32 = 1;
static ERR_CONFIGURATION_FILE: i32 = 2;
static ERR_PARSING_CONFIGURATION: i32 = 3;
static ERR_PARSING_TEMPLATE: i32 = 4;
static ERR_RENDERING_TEMPLATE: i32 = 5;

fn main() {
    let args: Cli = Cli::parse();
    let path = args.path;
    let utf8_path = path.to_str().unwrap_or("<path not representable in UTF-8>");
    eprintln!("Using template file '{}'", utf8_path);

    let template_content = std::fs::read_to_string(path.clone())
        .unwrap_or_else(|error| {
            eprintln!("ERROR: Could not read template file '{}': {}", utf8_path, error);
            exit(ERR_TEMPLATE_FILE);
        });

    let config = args.config;
    let utf8_config_path = config.to_str().unwrap_or("<path not representable in UTF-8>");
    eprintln!("Using configuration file '{}'", utf8_config_path);

    let configuration_content = std::fs::read_to_string(config.clone())
        .unwrap_or_else(|error| {
            eprintln!("ERROR: Could not read configuration file '{}': {}", utf8_config_path, error);
            exit(ERR_CONFIGURATION_FILE);
        });

    let configuration: Value = serde_json::from_str(configuration_content.as_str())
        .unwrap_or_else(|parse_error| {
            let classification = match parse_error.classify() {
                Category::Io => "I/O error",
                Category::Syntax => "syntax error",
                Category::Data => "data error",
                Category::Eof => "premature end of file"
            };
            eprintln!("ERROR: Could not parse JSON configuration ({}): {}", classification, parse_error);
            exit(ERR_PARSING_CONFIGURATION)
        });

    let file = TemplateParser::parse(Rule::file, &template_content)
        .unwrap_or_else(|parse_error| {
            eprintln!("ERROR: Could not parse template");
            // Formatted content on new line
            eprintln!("{}", parse_error);
            exit(ERR_PARSING_TEMPLATE)
        })
        .next().unwrap();

    let result = evaluate_file(&configuration, file)
        .unwrap_or_else(|template_render_error| {
            match template_render_error {
                TemplateRenderError::TypeError(value) => {
                    eprintln!("ERROR: Could not render template: {}", value);
                }
                TemplateRenderError::LiteralParseError(value) => {
                    eprintln!("ERROR: Could not render template: {}", value);
                }
            }
            exit(ERR_RENDERING_TEMPLATE)
        });

    print!("{}", result);
}
