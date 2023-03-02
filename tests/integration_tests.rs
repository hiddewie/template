use assert_cmd::Command;

#[test]
fn no_arguments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd.assert();

    assert
        .failure()
        .code(2)
        .stdout("")
        .stderr(r#"error: The following required arguments were not provided:
  --template <TEMPLATE>
  --configuration <CONFIGURATION>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information try '--help'
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
        .stderr(r#"error: The following required arguments were not provided:
  --template <TEMPLATE>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information try '--help'
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
        .stderr(r#"error: The following required arguments were not provided:
  --configuration <CONFIGURATION>

Usage: template --template <TEMPLATE> --configuration <CONFIGURATION>

For more information try '--help'
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
        .stdout(r#"template-cli 0.1.20
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
  -h, --help                           Print help information
  -V, --version                        Print version information
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
        .stderr(r#"Using template file 'tests/template/does_not_exist.template'
ERROR: Could not read template file 'tests/template/does_not_exist.template': No such file or directory (os error 2)
"#);
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
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/does_not_exist.json'
ERROR: Could not read configuration file 'tests/configuration/does_not_exist.json': No such file or directory (os error 2)
"#);
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
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/invalid.json'
Parsing configuration using JSON format
ERROR: Could not parse JSON configuration (syntax error): key must be a string at line 1 column 2
"#);
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
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/invalid.hcl'
Parsing configuration using HCL format
ERROR: Could not parse HCL configuration:
 --> 2:2
  |
2 |  "   ]
  |  ^---
  |
  = expected Identifier in line 2, col 2
"#);
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
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/invalid.yaml'
Parsing configuration using YAML format
ERROR: Could not parse YAML configuration: found unexpected end of stream at line 3 column 1, while scanning a quoted scalar at line 2 column 2
"#);
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
        .stderr(r#"Using template file 'tests/template/invalid.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
ERROR: Could not parse template
 --> 1:3
  |
1 | {%
  |   ^---
  |
  = expected keyword_if, keyword_unless, or expression
"#);
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
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.hcl'
Parsing configuration using HCL format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.yaml'
Parsing configuration using YAML format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.yml'
Parsing configuration using YAML format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Reading configuration from standard input stream
Parsing configuration using YAML format
"#);
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
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.unknown'
Parsing configuration using HCL format
"#);
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
        .stderr(r#"Using template file 'tests/template/no_variables.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/variables.template'
Using configuration file 'tests/configuration/variables.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/a.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/if_else.template'
Using configuration file 'tests/configuration/if_else.json'
Parsing configuration using JSON format
"#);
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


"#)
        .stderr(r#"Using template file 'tests/template/iteration.template'
Using configuration file 'tests/configuration/iteration.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/comments.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/function_error.template'
Using configuration file 'tests/configuration/function_error.json'
Parsing configuration using JSON format
ERROR: Could not render template: Invalid type '&serde_json::value::Value'
"#);
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
"#)
        .stderr(r#"Using template file 'tests/template/string_functions.template'
Using configuration file 'tests/configuration/string_functions.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/number_functions.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/boolean_functions.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
"#)
        .stderr(r#"Using template file 'tests/template/array_functions.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/dictionary_functions.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/literals.template'
Using configuration file 'tests/configuration/literals.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/default.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
"#);
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
        .stderr(r#"Using template file 'tests/template/unknown_function.template'
Using configuration file 'tests/configuration/empty.json'
Parsing configuration using JSON format
ERROR: Could not render template: Unknown function 'doesNotExist'
"#);
}
