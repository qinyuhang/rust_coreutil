extern crate assert_cmd;
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
