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
