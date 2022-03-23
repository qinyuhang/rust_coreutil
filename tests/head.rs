use assert_cmd::Command as A_CMD;

#[test]
fn test_regular_file() {
    let mut c = A_CMD::cargo_bin("head").unwrap();
    c.args(&["./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/head/cli.txt").unwrap());
}

#[test]
fn test_regular_file_with_n_switch() {
    let mut c = A_CMD::cargo_bin("head").unwrap();
    c.args(&["-n", "1", "./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/head/cli.1.txt").unwrap());
}

#[test]
fn test_head_multiple_files() {
    let mut c = A_CMD::cargo_bin("head").unwrap();
    c.args(&["./tests/cli.rs", "./tests/echo.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/head/multiple.txt").unwrap());
}

#[test]
fn test_not_exist_file() {
    let mut c = A_CMD::cargo_bin("head").unwrap();
    c.args(&["./not_exist.file"])
        .assert()
        .failure()
        .stderr("head: ./not_exist.file: No such file or directory (os error 2)\n");
}
