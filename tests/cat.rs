use assert_cmd::Command as A_CMD;

const cli_rs_content: &str = r#"use std::process::Command;
use assert_cmd::{Command as A_CMD};

#[test]
fn test_canary() {
    assert!(true);
}

#[test]
fn test_ls() {
    let mut c = Command::new("ls");
    assert!(c.output().is_ok());
}


#[test]
fn test_true() {
    let mut c = Command::new("./target/debug/true");
    assert!(c.output().is_ok());
}

#[test]
fn test_false() {
    let mut c = A_CMD::cargo_bin("false").unwrap();
    c.assert().failure();
}

#[test]
fn test_hello() {
    let mut c = A_CMD::cargo_bin("hello").unwrap();
    c.assert().success().stdout("Hello world!\n");
}"#;

const echo_rs_content: &str = r#"extern crate assert_cmd;
extern crate serde;
extern crate serde_json;

use assert_cmd::Command as A_CMD;
use serde::{Deserialize, Serialize};

#[test]
fn test_echo() {
    let mut c = A_CMD::cargo_bin("echo").unwrap();
    c.args(&["abc", "def"])
        .assert()
        .success()
        .stdout("abc def\n");
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    args: Vec<String>,
    output: String,
}

#[test]
fn test_file() {
    let source = std::fs::read_to_string("./tests/source/echo.json").unwrap();

    let cases: Vec<TestCase> = serde_json::from_str(&source).unwrap();

    cases.iter().for_each(|case| {
        let mut c = A_CMD::cargo_bin("echo").unwrap();
        c.args(&(case.args))
            .assert()
            .success()
            .stdout(case.output.clone());
    });

    // source.for_each(|z| {
    //     let file = std::fs::File::open(z.unwrap().path()).unwrap();
    //     let reader = std::io::BufReader::new(file);
    //     let x: Vec<TestCase> = serde_json::from_reader(reader).unwrap();
    //     c.args(&(x.args)).assert().success().stdout(x.output);
    // });
}
"#;

#[test]
fn test_cat_empty() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.assert().success().stdout("");
}

#[test]
fn test_cat_file_not_exist() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.arg("./not_exist").assert().success().stdout("cat: ./not_exist: No such file or directory (os error 2)");
}

#[test]
fn test_cat_file_exist() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.arg("./tests/cli.rs").assert().success().stdout(cli_rs_content);
}

#[test]
fn test_cat_multiple_files() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.args(&["./tests/cli.rs", "./tests/echo.rs"]).assert().success().stdout(cli_rs_content.to_string() + echo_rs_content);
}
