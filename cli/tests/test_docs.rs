use expectorate::assert_contents;

#[test]
fn test_docs_json() {
    use assert_cmd::Command;

    let output = Command::cargo_bin("oxide")
        .unwrap()
        .arg("docs")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_contents("./docs/cli.json", &stdout);
}
