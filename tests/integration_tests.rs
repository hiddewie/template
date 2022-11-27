use assert_cmd::Command;

#[test]
fn template_does_not_exist() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/does_not_exist.template")
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
        .arg("tests/template/empty.template")
        .arg("tests/configuration/does_not_exist.json")
        .assert();

    assert
        .failure()
        .code(2)
        .stdout("")
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/does_not_exist.json'
ERROR: Could not read configuration file 'tests/configuration/does_not_exist.json': No such file or directory (os error 2)
"#);
}

#[test]
fn invalid_configuration() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/empty.template")
        .arg("tests/configuration/invalid.json")
        .assert();

    assert
        .failure()
        .code(3)
        .stdout("")
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/invalid.json'
ERROR: Could not parse JSON configuration (syntax error): key must be a string at line 1 column 2
"#);
}

#[test]
fn invalid_template() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/invalid.template")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .failure()
        .code(4)
        .stdout("")
        .stderr(r#"Using template file 'tests/template/invalid.template'
Using configuration file 'tests/configuration/empty.json'
ERROR: Could not parse template
 --> 1:3
  |
1 | {%
  |   ^---
  |
  = expected property
"#);
}

#[test]
fn empty_template() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/empty.template")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout("")
        .stderr(r#"Using template file 'tests/template/empty.template'
Using configuration file 'tests/configuration/empty.json'
"#);
}

#[test]
fn hello_world() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/hello_world.template")
        .arg("tests/configuration/hello_world.json")
        .assert();

    assert
        .success()
        .stdout("Hello world!")
        .stderr(r#"Using template file 'tests/template/hello_world.template'
Using configuration file 'tests/configuration/hello_world.json'
"#);
}

#[test]
fn no_variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/no_variables.template")
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
"#);
}

#[test]
fn variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/variables.template")
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
"#);
}

#[test]
fn missing_configuration_value() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/a.template")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"!!
!!
"#)
        .stderr(r#"Using template file 'tests/template/a.template'
Using configuration file 'tests/configuration/empty.json'
"#);
}

#[test]
fn if_else() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/if_else.template")
        .arg("tests/configuration/if_else.json")
        .assert();

    assert
        .success()
        .stdout(r#"
else



true





nonempty



else



else
"#)
        .stderr(r#"Using template file 'tests/template/if_else.template'
Using configuration file 'tests/configuration/if_else.json'
"#);
}

#[test]
fn iteration() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/iteration.template")
        .arg("tests/configuration/iteration.json")
        .assert();

    assert
        .success()
        .stdout(r#"
loop start
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
"#);
}

#[test]
fn comments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/comments.template")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"


false


"#)
        .stderr(r#"Using template file 'tests/template/comments.template'
Using configuration file 'tests/configuration/empty.json'
"#);
}

#[test]
fn function_error() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/function_error.template")
        .arg("tests/configuration/function_error.json")
        .assert();

    assert
        .failure()
        .code(5)
        .stdout("")
        .stderr(r#"Using template file 'tests/template/function_error.template'
Using configuration file 'tests/configuration/function_error.json'
ERROR: Could not render template: &serde_json::value::Value
"#);
}

#[test]
fn string_functions() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/string_functions.template")
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
"#)
        .stderr(r#"Using template file 'tests/template/string_functions.template'
Using configuration file 'tests/configuration/string_functions.json'
"#);
}
