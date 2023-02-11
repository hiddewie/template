extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::cell::RefCell;
use std::fmt::Debug;
use std::io::Read;
use std::process::exit;
use std::rc::Rc;

use clap::Parser as ClapParser;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use serde_json::{Map, Value};
use serde_json::error::Category;

use template_cli::function;
use template_cli::error::TemplateRenderError;

#[derive(Parser)]
#[grammar = "grammar/template.pest"]
pub struct TemplateParser;

#[derive(clap::ValueEnum, Clone, Eq, PartialEq)]
enum ConfigurationFormat {
    JSON,
    HCL,
    YAML,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Absolute or relative path to the template file.
    #[arg(short, long)]
    template: std::path::PathBuf,

    /// Absolute or relative path to the configuration file.
    /// Provide `-` as path to read the configuration input from the standard input stream.
    #[arg(short, long)]
    configuration: std::path::PathBuf,

    /// Specify the format of the configuration input. Useful when the configuration file has
    /// a non-standard extension, or when the input is given in the standard input stream.
    #[arg(short, long, value_enum)]
    format: Option<ConfigurationFormat>,
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
                .map(|inner_literal| parse_expression(&value, &mut inner_literal.into_inner()))
                .collect();

            array.map(|result| Value::Array(result))
        }
        Rule::dictionary => {
            let mut result = Map::new();
            for pair in literal.clone().into_inner() {
                let mut key_value_content = pair.into_inner();
                let pair_key = key_value_content.next().unwrap().as_str();
                let pair_value = key_value_content.next().unwrap();
                let evaluated_pair_value = parse_expression(&value, &mut pair_value.into_inner())?;

                result.insert(pair_key.to_string(), evaluated_pair_value);
            }

            Ok(Value::Object(result))
        }
        _ => unreachable!()
    }
}

fn parse_expression(value: &Value, expression: &mut Pairs<Rule>) -> Result<Value, TemplateRenderError> {
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
            Rule::function_call => {
                let mut function_and_arguments = function.into_inner();
                let function_name = function_and_arguments.next().unwrap().as_str();
                let mut arguments: Vec<Value> = vec![];
                for argument in function_and_arguments {
                    match argument.as_rule() {
                        Rule::expression => {
                            arguments.push(parse_expression(value, &mut argument.into_inner())?)
                        }
                        _ => unreachable!(),
                    }
                }

                result = function::apply_function(&result, function_name, &arguments)?;
            }
            _ => unreachable!(),
        }
    }
    return Ok(result);
}


fn evaluate_template(data: &Value, record: Pair<Rule>) -> Result<(String, bool), TemplateRenderError> {
    let mut result = String::new();

    let mut inner_rules = record.into_inner();
    let expression = inner_rules.next().unwrap();

    let mut gobble = false;
    match expression.as_rule() {
        Rule::if_elif_else_template => {
            let mut done = false;
            let mut valid = false;
            for if_inner in expression.into_inner() {
                match if_inner.as_rule() {
                    Rule::if_template => {
                        gobble = true;
                        valid = false;

                        let mut if_inner_expression = if_inner.into_inner();
                        let if_keyword = if_inner_expression.next().unwrap();
                        let invert = match if_keyword.as_rule() {
                            Rule::keyword_if => false,
                            Rule::keyword_unless => true,
                            _ => unreachable!(),
                        };
                        let if_expression = if_inner_expression.next().unwrap();
                        let if_result = parse_expression(&data, &mut if_expression.into_inner())
                            .map(|value| function::to_boolean(&value))
                            .unwrap_or(false);
                        if if_result ^ invert {
                            done = true;
                            valid = true
                        }
                    }
                    Rule::elif_template => {
                        result = result.trim_end_matches(&[' ', '\t']).to_string();
                        valid = false;

                        let mut elif_inner_expression = if_inner.into_inner();
                        let elif_expression = elif_inner_expression.next().unwrap();
                        let elif_result = parse_expression(&data, &mut elif_expression.into_inner())
                            .map(|value| function::to_boolean(&value))
                            .unwrap_or(false);
                        if !done && elif_result {
                            done = true;
                            valid = true
                        }
                    }
                    Rule::else_template => {
                        result = result.trim_end_matches(&[' ', '\t']).to_string();
                        valid = !done;
                    }
                    Rule::end_template => {
                        result = result.trim_end_matches(&[' ', '\t']).to_string();
                        valid = false;
                        done = true;
                    }
                    Rule::character => {
                        if valid {
                            result.push_str(if_inner.as_str())
                        }
                    }
                    Rule::template => {
                        let (evaluation, gobble_inner) = evaluate_template(&data, if_inner)?;
                        if gobble_inner {
                            result = result.trim_end_matches(&[' ', '\t']).to_string();
                        }
                        result.push_str(evaluation.as_str())
                    }
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
                        gobble = true;
                        let mut for_inner_expression = for_inner.into_inner();
                        iterable_name = for_inner_expression.next().unwrap().as_str();
                        let for_iterable = parse_expression(&data, &mut for_inner_expression.next().unwrap().into_inner()).unwrap();
                        iterables = match for_iterable {
                            Value::Array(items) => items,
                            _ => vec![],
                        };
                        done = iterables.is_empty();
                        valid = !iterables.is_empty();
                        iterables_results = iterables.iter().map(|_| Rc::new(RefCell::new(String::new()))).collect();
                    }
                    Rule::else_template => {
                        result = result.trim_end_matches(&[' ', '\t']).to_string();
                        valid = false;
                        if !done {
                            valid = true
                        }
                    }
                    Rule::end_template => {
                        result = result.trim_end_matches(&[' ', '\t']).to_string();
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
                            let (template_result, gobble_inner) = evaluate_template(&context_value, for_inner.clone())?;
                            iterable_result.replace_with(|current_result|
                                if gobble_inner {
                                    format!("{}{}", current_result.as_str().trim_end_matches(&[' ', '\t']), template_result)
                                } else {
                                    format!("{}{}", current_result, template_result)
                                }
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
            let evaluation_result = parse_expression(&data, &mut expression.into_inner())?;
            result.push_str(format_string(&evaluation_result).to_string().as_str())
        }
        Rule::comment => (),
        _ => unreachable!(),
    }

    return Ok((result, gobble));
}

fn evaluate_file(data: &Value, file: Pair<Rule>) -> Result<String, TemplateRenderError> {
    let mut result = String::new();

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::template => {
                let (evaluation, gobble_inner) = evaluate_template(&data, record)?;
                if gobble_inner {
                    result = result.trim_end_matches(&[' ', '\t']).to_string();
                }
                result.push_str(evaluation.as_str())
            }
            Rule::character => {
                result.push_str(record.as_str())
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    return Ok(result);
}

static ERR_TEMPLATE_FILE: i32 = 1;
static ERR_CONFIGURATION_FILE: i32 = 3;
static ERR_PARSING_CONFIGURATION: i32 = 4;
static ERR_PARSING_TEMPLATE: i32 = 5;
static ERR_RENDERING_TEMPLATE: i32 = 6;

fn main() {
    let args: Cli = Cli::parse();
    let template_path = args.template;
    let utf8_template_path = template_path.to_str().unwrap_or("<path not representable in UTF-8>");
    eprintln!("Using template file '{}'", utf8_template_path);

    let template_content = std::fs::read_to_string(template_path.clone())
        .unwrap_or_else(|error| {
            eprintln!("ERROR: Could not read template file '{}': {}", utf8_template_path, error);
            exit(ERR_TEMPLATE_FILE);
        });

    let configuration_path = args.configuration;
    let utf8_configuration_path = configuration_path.to_str().unwrap_or("<path not representable in UTF-8>");
    let configuration_content = if utf8_configuration_path == "-" {
        eprintln!("Reading configuration from standard input stream");
        let mut input = Vec::new();
        let mut handle = std::io::stdin().lock();
        handle.read_to_end(&mut input)
            .unwrap_or_else(|error| {
                eprintln!("ERROR: I/O error while reading configuration input: {}", error);
                exit(ERR_CONFIGURATION_FILE);
            });
        String::from_utf8(input)
            .unwrap_or_else(|error| {
                eprintln!("ERROR: Could not parse configuration input as UTF-8: {}", error);
                exit(ERR_CONFIGURATION_FILE);
            })
    } else {
        eprintln!("Using configuration file '{}'", utf8_configuration_path);
        std::fs::read_to_string(configuration_path.clone())
            .unwrap_or_else(|error| {
                eprintln!("ERROR: Could not read configuration file '{}': {}", utf8_configuration_path, error);
                exit(ERR_CONFIGURATION_FILE);
            })
    };

    let input_format = args.format;
    let configuration: Value = if input_format == Some(ConfigurationFormat::HCL) || utf8_configuration_path.ends_with(".hcl") {
        eprintln!("Parsing configuration using HCL format");
        hcl::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                eprintln!("ERROR: Could not parse HCL configuration:");
                // Formatted error on new line
                eprintln!("{}", parse_error);
                exit(ERR_PARSING_CONFIGURATION)
            })
    } else if input_format == Some(ConfigurationFormat::YAML) || utf8_configuration_path.ends_with(".yml") || utf8_configuration_path.ends_with(".yaml") {
        eprintln!("Parsing configuration using YAML format");
        serde_yaml::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                eprintln!("ERROR: Could not parse YAML configuration: {}", parse_error);
                exit(ERR_PARSING_CONFIGURATION)
            })
    } else {
        // Default to JSON
        eprintln!("Parsing configuration using JSON format");
        serde_json::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                let classification = match parse_error.classify() {
                    Category::Io => "I/O error",
                    Category::Syntax => "syntax error",
                    Category::Data => "data error",
                    Category::Eof => "premature end of file"
                };
                eprintln!("ERROR: Could not parse JSON configuration ({}): {}", classification, parse_error);
                exit(ERR_PARSING_CONFIGURATION)
            })
    };

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
            eprintln!("ERROR: Could not render template: {}", template_render_error);
            exit(ERR_RENDERING_TEMPLATE)
        });

    print!("{}", result);
}
