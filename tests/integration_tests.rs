use assert_cmd::Command;

#[test]
fn empty_template() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/empty.tmpl")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout("")
        .stderr(r#"Template path 'tests/template/empty.tmpl'
Configuration path 'tests/configuration/empty.json'
"#);
}

#[test]
fn no_variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/no_variables.tmpl")
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
        .stderr(r#"Template path 'tests/template/no_variables.tmpl'
Configuration path 'tests/configuration/empty.json'
"#);
}

#[test]
fn variables() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/variables.tmpl")
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
        .stderr(r#"Template path 'tests/template/variables.tmpl'
Configuration path 'tests/configuration/variables.json'
"#);
}

#[test]
fn missing_configuration_value() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/a.tmpl")
        .arg("tests/configuration/empty.json")
        .assert();

    assert
        .success()
        .stdout(r#"!!
"#)
        .stderr(r#"Template path 'tests/template/a.tmpl'
Configuration path 'tests/configuration/empty.json'
"#);
}
