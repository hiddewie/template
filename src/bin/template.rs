use std::io::Read;
use std::process::exit;

use clap::Parser as ClapParser;
use env_logger::Env;
use log::{error, info};
use serde_json::error::Category;
use serde_json::Value;

use template_cli::evaluate;

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


static ERR_TEMPLATE_FILE: i32 = 1;
static ERR_CONFIGURATION_FILE: i32 = 3;
static ERR_PARSING_CONFIGURATION: i32 = 4;
static ERR_PARSING_TEMPLATE: i32 = 5;
static ERR_RENDERING_TEMPLATE: i32 = 6;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Cli = Cli::parse();
    let template_path = args.template;
    let utf8_template_path = template_path.to_str().unwrap_or("<path not representable in UTF-8>");
    info!("Using template file '{}'", utf8_template_path);

    let template_content = std::fs::read_to_string(template_path.clone())
        .unwrap_or_else(|error| {
            error!("ERROR: Could not read template file '{}': {}", utf8_template_path, error);
            exit(ERR_TEMPLATE_FILE);
        });

    let configuration_path = args.configuration;
    let utf8_configuration_path = configuration_path.to_str().unwrap_or("<path not representable in UTF-8>");
    let configuration_content = if utf8_configuration_path == "-" {
        info!("Reading configuration from standard input stream");
        let mut input = Vec::new();
        let mut handle = std::io::stdin().lock();
        handle.read_to_end(&mut input)
            .unwrap_or_else(|error| {
                error!("ERROR: I/O error while reading configuration input: {}", error);
                exit(ERR_CONFIGURATION_FILE);
            });
        String::from_utf8(input)
            .unwrap_or_else(|error| {
                error!("ERROR: Could not parse configuration input as UTF-8: {}", error);
                exit(ERR_CONFIGURATION_FILE);
            })
    } else {
        info!("Using configuration file '{}'", utf8_configuration_path);
        std::fs::read_to_string(configuration_path.clone())
            .unwrap_or_else(|error| {
                error!("ERROR: Could not read configuration file '{}': {}", utf8_configuration_path, error);
                exit(ERR_CONFIGURATION_FILE);
            })
    };

    let input_format = args.format;
    let configuration: Value = if input_format == Some(ConfigurationFormat::HCL) || utf8_configuration_path.ends_with(".hcl") {
        info!("Parsing configuration using HCL format");
        hcl::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                // Formatted error on new line
                error!("{}", format!("ERROR: Could not parse HCL configuration:\n{}", parse_error));
                exit(ERR_PARSING_CONFIGURATION)
            })
    } else if input_format == Some(ConfigurationFormat::YAML) || utf8_configuration_path.ends_with(".yml") || utf8_configuration_path.ends_with(".yaml") {
        info!("Parsing configuration using YAML format");
        serde_yaml::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                error!("ERROR: Could not parse YAML configuration: {}", parse_error);
                exit(ERR_PARSING_CONFIGURATION)
            })
    } else {
        // Default to JSON
        info!("Parsing configuration using JSON format");
        serde_json::from_str(configuration_content.as_str())
            .unwrap_or_else(|parse_error| {
                let classification = match parse_error.classify() {
                    Category::Io => "I/O error",
                    Category::Syntax => "syntax error",
                    Category::Data => "data error",
                    Category::Eof => "premature end of file"
                };
                error!("ERROR: Could not parse JSON configuration ({}): {}", classification, parse_error);
                exit(ERR_PARSING_CONFIGURATION)
            })
    };

    let file = evaluate::parse_template(&template_content)
        .unwrap_or_else(|parse_error| {
            // Formatted content on new line
            error!("{}", format!("ERROR: Could not parse template\n{parse_error}"));
            exit(ERR_PARSING_TEMPLATE)
        })
        .next().unwrap();

    let result = evaluate::evaluate_file(&configuration, file)
        .unwrap_or_else(|template_render_error| {
            error!("ERROR: Could not render template: {}", template_render_error);
            exit(ERR_RENDERING_TEMPLATE)
        });

    print!("{}", result);
}
