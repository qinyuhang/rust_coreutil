use assert_cmd::Command as A_CMD;

#[test]
fn test_canary_file() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.assert().success();
}

#[test]
fn test_regular_file() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/cli.txt").unwrap());
}

#[test]
fn test_m_option() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-m", "./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/m_cli.txt").unwrap());
}

#[test]
fn test_c_option() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-c", "./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/c_cli.txt").unwrap());
}

#[test]
fn test_l_option() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-l", "./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/l_cli.txt").unwrap());
}

#[test]
fn test_w_option() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-w", "./tests/cli.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/w_cli.txt").unwrap());
}

/// start test multiple files
///

#[test]
fn test_files() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["./tests/cli.rs", "./tests/cat.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/files.txt").unwrap());
}

#[test]
fn test_m_option_files() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-m", "./tests/cli.rs", "./tests/cat.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/m_files.txt").unwrap());
}

#[test]
fn test_c_option_files() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-c", "./tests/cli.rs", "./tests/cat.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/c_files.txt").unwrap());
}

#[test]
fn test_l_option_files() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-l", "./tests/cli.rs", "./tests/cat.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/l_files.txt").unwrap());
}

#[test]
fn test_w_option_files() {
    let mut c = A_CMD::cargo_bin("wc").unwrap();
    c.args(&["-w", "./tests/cli.rs", "./tests/cat.rs"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/wc/w_files.txt").unwrap());
}
