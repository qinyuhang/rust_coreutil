use assert_cmd::Command as A_CMD;

#[test]
fn one_line() {}

#[test]
fn read_me_md() {
    let mut c = A_CMD::cargo_bin("uniq").unwrap();
    c.args(&["./readme.md"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./readme.md").unwrap());
}

#[test]
fn read_me_md_c() {
    let mut c = A_CMD::cargo_bin("uniq").unwrap();
    c.args(&["-c", "./readme.md"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/uniq/readme_c.txt").unwrap());
}

#[test]
fn d_op() {
    let mut c = A_CMD::cargo_bin("uniq").unwrap();
    c.args(&["-d", "./tests/source/uniq/d_source.txt"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/uniq/d_output.txt").unwrap());
}

#[test]
fn i_op() {
    let mut c = A_CMD::cargo_bin("uniq").unwrap();
    c.args(&["-i", "./tests/source/uniq/i_source.txt"])
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/source/uniq/i_output.txt").unwrap());
}