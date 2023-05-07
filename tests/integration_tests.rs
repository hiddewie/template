use assert_cmd::Command;
use predicate::str::is_match;
use predicates::prelude::*;

#[test]
fn no_arguments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd.assert();

    assert
        .failure()
        .code(2)
        .stdout("")
        .stderr(r#"error: the following required arguments were not provided:
  --template <TEMPLATE>
  --configuration <CONFIGURATION>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information, try '--help'.
"#);
}

#[test]
fn missing_required_template_argument() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .failure()
        .code(2)
        .stdout("")
        .stderr(r#"error: the following required arguments were not provided:
  --template <TEMPLATE>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information, try '--help'.
"#);
}

#[test]
fn missing_required_configuration_argument() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .assert();

    assert
        .failure()
        .code(2)
        .stdout("")
        .stderr(r#"error: the following required arguments were not provided:
  --configuration <CONFIGURATION>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information, try '--help'.
"#);
}

#[test]
fn version() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--version")
        .assert();

    assert
        .success()
        .stdout(r#"template-cli 0.3.0
"#)
        .stderr("");
}

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--help")
        .assert();

    assert
        .success()
        .stdout(r#"CLI for templating based on JSON, YAML or HCL configuration

Usage: template [OPTIONS] --template <TEMPLATE> --configuration <CONFIGURATION>

Options:
  -t, --template <TEMPLATE>            Absolute or relative path to the template file
  -c, --configuration <CONFIGURATION>  Absolute or relative path to the configuration file. Provide `-` as path to read the configuration input from the standard input stream
  -f, --format <FORMAT>                Specify the format of the configuration input. Useful when the configuration file has a non-standard extension, or when the input is given in the standard input stream [possible values: json, hcl, yaml]
  -h, --help                           Print help
  -V, --version                        Print version
"#)
        .stderr("");
}

#[test]
fn template_does_not_exist() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/does_not_exist.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .failure()
        .code(1)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/does_not_exist.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not read template file 'tests/template/does_not_exist.template': No such file or directory \(os error 2\)
$"#).unwrap());
}

#[test]
fn configuration_does_not_exist() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/does_not_exist.json")
        .assert();

    assert
        .failure()
        .code(3)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/empty.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/does_not_exist.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not read configuration file 'tests/configuration/does_not_exist.json': No such file or directory \(os error 2\)
$"#).unwrap());
}

#[test]
fn test_error_log_level() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/does_not_exist.json")
        .env("RUST_LOG", "error")
        .assert();

    assert
        .code(3)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not read configuration file 'tests/configuration/does_not_exist.json': No such file or directory \(os error 2\)
$"#).unwrap());
}

#[test]
fn invalid_configuration_json() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/invalid.json")
        .assert();

    assert
        .failure()
        .code(4)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/empty.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/invalid.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not parse JSON configuration \(syntax error\): key must be a string at line 1 column 2
$"#).unwrap());
}

#[test]
fn invalid_configuration_hcl() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/invalid.hcl")
        .assert();

    assert
        .failure()
        .code(4)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/empty.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/invalid.hcl'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using HCL format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not parse HCL configuration:
 --> 2:2
  |
2 |  "   ]
  |  ^---
  |
  = expected Identifier in line 2, col 2
$"#).unwrap());
}

#[test]
fn invalid_configuration_yaml() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/invalid.yaml")
        .assert();

    assert
        .failure()
        .code(4)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/empty.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/invalid.yaml'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using YAML format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not parse YAML configuration: found unexpected end of stream at line 3 column 1, while scanning a quoted scalar at line 2 column 2
$"#).unwrap());
}

#[test]
fn invalid_template() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/invalid.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .failure()
        .code(5)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/invalid.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not parse template
     --> 1:3
      |
    1 | \{%
      |   ^---
      |
      = expected keyword_if, keyword_unless, or expression
$"#).unwrap());
}

#[test]
fn empty_template() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/empty.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/empty.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn hello_world_json() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("tests/configuration/hello_world.json")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/hello_world.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn hello_world_hcl() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("tests/configuration/hello_world.hcl")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/hello_world.hcl'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using HCL format
$"#).unwrap());
}

#[test]
fn hello_world_yaml() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("tests/configuration/hello_world.yaml")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/hello_world.yaml'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using YAML format
$"#).unwrap());
}

#[test]
fn hello_world_yml() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("tests/configuration/hello_world.yml")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/hello_world.yml'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using YAML format
$"#).unwrap());
}

#[test]
fn read_configuration_from_stdin() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("-")
        .arg("--format")
        .arg("yaml")
        .pipe_stdin("tests/configuration/hello_world.yml").unwrap()
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Reading configuration from standard input stream
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using YAML format
$"#).unwrap());
}

#[test]
fn hcl_with_format_and_unknown_extension() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/hello_world.template")
        .arg("--configuration")
        .arg("tests/configuration/hello_world.unknown")
        .arg("--format")
        .arg("hcl")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/hello_world.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/hello_world.unknown'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using HCL format
$"#).unwrap());
}

#[test]
fn no_variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/no_variables.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"line1
line2
line3

line4

done
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/no_variables.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/variables.template")
        .arg("--configuration")
        .arg("tests/configuration/variables.json")
        .assert();

    assert
        .success()
        .stdout(r#"begin
!1!
string

--value--
false
{g:1}
[1,2,3]
end
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/variables.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/variables.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn missing_configuration_value() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/a.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"!!
!!
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/a.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn if_else() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/if_else.template")
        .arg("--configuration")
        .arg("tests/configuration/if_else.json")
        .assert();

    assert
        .success()
        .stdout(r#"else

false

else

true


nonempty

else

else

    indented content
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/if_else.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/if_else.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn iteration() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/iteration.template")
        .arg("--configuration")
        .arg("tests/configuration/iteration.json")
        .assert();

    assert
        .success()
        .stdout(r#"loop start
outer
value 1
nested value 1

loop end
loop start
outer
value 2
nested value 2

loop end



  0-based index: 0
  1-based index: 1
  true if this is the first iteration: true
  true if this is the last iteration: false
  the number of iterations in the loop: 2
  alternate items in an array, treating it as circular: one
  0-based index: 1
  1-based index: 2
  true if this is the first iteration: false
  true if this is the last iteration: true
  the number of iterations in the loop: 2
  alternate items in an array, treating it as circular: two
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/iteration.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/iteration.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn comments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/comments.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"
false
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/comments.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn function_error() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/function_error.template")
        .arg("--configuration")
        .arg("tests/configuration/function_error.json")
        .assert();

    assert
        .failure()
        .code(6)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/function_error.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/function_error.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not render template: Invalid type '&serde_json::value::Value'
$"#).unwrap());
}

#[test]
fn string_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/string_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/string_functions.json")
        .env("ENV_TEST", "env_test")
        .assert();

    assert
        .success()
        .stdout(r#"print: string
upperCase: STRING
lowerCase: upper
length: 0
length: 6
kebabCase: string-with-spaces
camelCase: stringWithSpaces
snakeCase: string_with_spaces
pascalCase: StringWithSpaces
capitalize: String with spaces
capitalizeWords: String With Spaces
environment: env_test
reverse: gnirts
split: [hello,world]
lines: [a,b,c,,x]
matches: true
matches: false
contains: true
contains: true
contains: false
substring: ello
substring: ello world
substring: h
substring:
substring: hello world
take: hello world
take:
take: hel
drop:
drop: hello world
drop: lo world
empty: false
empty: true
empty: true
toJson: "  "
toJson: " xxx "
fromJson: {}
fromJson: [1,2,3]
fromJson: 1
fromJson:
abbreviate: a…
abbreviate: abdef
abbreviate: …
trim: abdef
trim:
trim: x
trim:
trimLeft: abdef
trimLeft:
trimLeft: x
trimLeft:
trimRight: abdef
trimRight:
trimRight:  x
trimRight:
replace: xx
replace: ab
replace:
regexReplace: axxxb
regexReplace: xxx xxx
regexReplace: unmatching
startsWith: true
startsWith: false
endsWith: true
endsWith: false
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/string_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/string_functions.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn number_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/number_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"empty: true
empty: true
empty: true
empty: true
empty: false
empty: false
empty: false
toJson: 1
toJson: -0.0
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/number_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn boolean_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/boolean_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"empty: true
empty: false
toJson: true
negate: false
negate: true
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/boolean_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn array_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/array_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"length: 0
length: 3
reverse: [3,2,1]
take: [1,2,3]
take: []
take: [1]
drop: []
drop: [1,2,3]
drop: [2,3]
first: 1
first:
last: 3
last:
index: 1
index:
index:
contains: true
contains: false
empty: false
empty: true
unique: [1,2,3]
empty: [1,2,{},[3],,-0.0,0.0]
any: true
any: false
all: false
all: false
none: false
none: false
some: true
some: false
chunked: []
chunked: [[1,2],[3]]
chunked: [[1],[2],[3]]
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/array_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn dictionary_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/dictionary_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"length: 0
length: 3
containsKey: true
containsKey: false
containsValue: true
containsValue: false
empty: false
empty: true
keys: [a,b,c]
keys: []
values: [1,2,3]
values: []
invert: {d:b}
toJson: {"a":1,"b":2,"c":3}
toJson: {}
toPrettyJson: {
  "a": 1,
  "b": 2,
  "c": 3
}
toPrettyJson: {}
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/dictionary_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn literals() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/literals.template")
        .arg("--configuration")
        .arg("tests/configuration/literals.json")
        .assert();

    assert
        .success()
        .stdout(r#"null:
true: true
false: false
integer: 0
integer: 100
integer: -100
float: 0.0
float: 0.0
float: 1.0
float: 1.0
float: 0.1
float: 10000000000.0
float: 10000000000.0
float: -0.0
float: -0.0
float: -1.0
float: -1.0
float: -0.1
float: -10000000000.0
float: -10000000000.0
string:
string: string
array: []
array: []
array: [1]
array: [,a,1,1.0,1,string]
dictionary: {}
dictionary: {}
dictionary: {a:}
dictionary: {" spaceee ":space!,a:b,c:1,d:,e:0.1,integer:1,string:string}
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/literals.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/literals.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn default() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/default.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"default
true
default
1
-1
default
default
default
100.0
-100.0
default
non empty
default
[1]
default
{a:a}
coalesce: x
coalesce: something
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/default.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn unknown_function_call() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/unknown_function.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .code(6)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/unknown_function.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not render template: Unknown function 'doesNotExist'
$"#).unwrap());
}

#[test]
fn invalid_chunked_arguments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/invalid_chunked_arguments.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .code(6)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/invalid_chunked_arguments.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not render template: Invalid arguments: The overlap \(6\) cannot be equal or larger than the chunk size \(3\)
$"#).unwrap());
}

#[test]
fn test_date_time_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/date_time_functions.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(is_match(r#"^parseFormatDateTime: \d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}
parseFormatDateTime: \d+
parseFormatDateTime: \d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}
parseFormatDateTime: Sunday  8 July 2001, 00:34:60
$"#).unwrap())
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/date_time_functions.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn test_context_variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/context_variables.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"  Value is 1
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/context_variables.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
$"#).unwrap());
}

#[test]
fn test_debug() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/debug.template")
        .arg("--configuration")
        .arg("tests/configuration/debug.json")
        .assert();

    assert
        .success()
        .stdout(r#"Template content...
... more content
"#)
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/debug.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/debug.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template_cli::evaluate\] Debug expression: a = 1
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template_cli::evaluate\] Debug expression: object = \{"b":"c"\}
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template_cli::evaluate\] Debug expression: null = null
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template_cli::evaluate\] Debug expression: "" | startsWith("abc") | false
$"#).unwrap());
}

#[test]
fn test_assert() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("--template")
        .arg("tests/template/assert.template")
        .arg("--configuration")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .code(6)
        .stdout("")
        .stderr(is_match(r#"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using template file 'tests/template/assert.template'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Using configuration file 'tests/configuration/empty.json'
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z INFO  template\] Parsing configuration using JSON format
\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ERROR template\] ERROR: Could not render template: Assertion failed: Expected value 'true' but found 'false': This is not OK
$"#).unwrap());
}