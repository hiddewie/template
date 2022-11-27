use assert_cmd::Command;

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
        .stderr(r#"Template path 'tests/template/empty.template'
Configuration path 'tests/configuration/empty.json'
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
        .stderr(r#"Template path 'tests/template/no_variables.template'
Configuration path 'tests/configuration/empty.json'
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
        .stderr(r#"Template path 'tests/template/variables.template'
Configuration path 'tests/configuration/variables.json'
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
        .stderr(r#"Template path 'tests/template/a.template'
Configuration path 'tests/configuration/empty.json'
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
        .stderr(r#"Template path 'tests/template/if_else.template'
Configuration path 'tests/configuration/if_else.json'
"#);
}

#[test]
fn iteration() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/iteration.template")
        .arg("tests/configuration/iteration.json")
        .assert();

    println!("{}", std::str::from_utf8(&*assert.get_output().stderr).unwrap().to_string());

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
        .stderr(r#"Template path 'tests/template/iteration.template'
Configuration path 'tests/configuration/iteration.json'
"#);
}

#[test]
fn comments() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/comments.template")
        .arg("tests/configuration/empty.json")
        .assert();

    println!("{}", std::str::from_utf8(&*assert.get_output().stderr).unwrap().to_string());

    assert
        .success()
        .stdout(r#"


false


"#)
        .stderr(r#"Template path 'tests/template/comments.template'
Configuration path 'tests/configuration/empty.json'
"#);
}
