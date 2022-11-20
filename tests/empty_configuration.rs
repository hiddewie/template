use assert_cmd::Command;

#[test]
fn empty_configuration() {
    // .env("stdout", "hello")
    // .env("exit", "42")
    // .write_stdin("42")
    // .code(42)

    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/test.tmpl")
        .arg("tests/configuration/config.json")
        .assert();

    assert
        .success()
        .stdout(r#"--- START ---
a
b
{out}
47
prop OUT
<no value>
c
--- END ---
"#);
}