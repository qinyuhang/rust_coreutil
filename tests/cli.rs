use assert_cmd::Command as A_CMD;
use std::process::Command;

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
}
