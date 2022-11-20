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
        .stdout(r#"--- START ---
--- END ---
"#)
        .stderr("");
}