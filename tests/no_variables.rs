use assert_cmd::Command;

#[test]
fn empty_configuration() {
    let mut cmd = Command::cargo_bin("template").unwrap();
    let assert = cmd
        .arg("tests/template/no_variables.tmpl")
        .arg("tests/configuration/config.json")
        .assert();

    assert
        .success()
        .stdout(r#"--- START ---
line1
line2
line3

line4

done
--- END ---
"#)
        .stderr("");
}