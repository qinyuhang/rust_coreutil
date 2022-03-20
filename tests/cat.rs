use assert_cmd::Command as A_CMD;
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
    c.arg("./tests/cli.rs").assert().success().stdout(r#"use std::process::Command;
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
}"#);
}