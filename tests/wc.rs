use assert_cmd::Command as A_CMD;

#[test]
fn test_canary_file() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.assert().success();
}

#[test]
fn test_regular_file() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["./tests/cli.rs"]).assert().success().stdout("      31      57     558 ./tests/cli.rs\n");
}