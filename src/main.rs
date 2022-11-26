extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::Parser as ClapParser;
use pest::iterators::Pairs;
use pest::Parser;
use serde_json::Value;

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

fn apply_function(value: String, function: &str) -> String {
    return match function {
        "lower" => {
            value.to_lowercase()
        }
        "upper" => {
            value.to_uppercase()
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
        Value::Array(values) => format!("[{}]", values.iter().map(|v| format_string(v)).reduce(|cur, next| format!("{},{}", &cur, &next)).unwrap_or("".to_string())), //"".to_string(), //"[" + values.map(|v| format_string(v)).join(",") + "]",
        Value::Object(object) => format!("{{{}}}", object.iter().map(|(k, v)| format!("{}:{}", k,format_string(v))).reduce(|cur, next| format!("{},{}", &cur, &next)).unwrap_or("".to_string())),
            // "{".to_string(), // + object.map(|k| k + ":".to_string() + format_string(object.get(k).unwrap())).join(",") + "}"
    }
}

fn evaluate(value: &Value, expression: &mut Pairs<Rule>) -> Option<String> {
    let properties = expression.next().unwrap();

    let mut current_value = value;
    for property in properties.into_inner() {
        match property.as_rule() {
            Rule::property => {
                current_value = &current_value[property.as_str()];
            }
            _ => unreachable!(),
        }
    }
    let mut result = format_string(current_value);
    for function in expression {
        match function.as_rule() {
            Rule::function => {
                result = apply_function(result, function.as_str());
            }
            _ => unreachable!(),
        }
    }
    return Some(result);
}

fn evaluate_boolean(value: &Value, expression: &mut Pairs<Rule>) -> Option<bool> {
    return evaluate(value, expression)
        .map(|result| result != "false" && !result.trim().is_empty());
}

fn main() {
    let args: Cli = Cli::parse();
    let path = args.path;
    eprintln!("Template path '{}'", path.to_str().unwrap());
    // println!("Path: {}", path.as_path().display());

    let template_content = std::fs::read_to_string(path).unwrap();

    let config = args.config;
    eprintln!("Configuration path '{}'", config.to_str().unwrap());
    // println!("Path: {}", config.as_path().display());

    let content = std::fs::read_to_string(config).unwrap();
    let data: Value = serde_json::from_str(content.as_str()).unwrap();
    // println!("Data: {}, {}", data["test"], data["out"]);

    let file = TemplateParser::parse(Rule::file, &template_content)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut result = String::new();
    for record in file.into_inner() {
        match record.as_rule() {
            // Rule::property => {
            //     print!("<property: {}>", record.as_str());
            // }
            Rule::template => {
                // print!("<template: '{}'>", record.as_str());
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
                                _ => unreachable!(),
                            }
                        }
                    }
                    Rule::expression_template => {
                        let mut inner_rules = expression.into_inner();
                        let expression = inner_rules.next().unwrap();

                        result.push_str(&evaluate(&data, &mut expression.into_inner()).unwrap_or("".to_string()))
                    }
                    _ => unreachable!(),
                }
            }
            Rule::character => {
                // let mut inner_rules = record.into_inner(); // { name }
                // let q  = inner_rules.next().unwrap().as_str();
                // print!("{}", record.as_str());
                result.push_str(record.as_str())


                // print!("Character {}", record);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    print!("{}", result);
}
