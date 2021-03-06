use assert_cmd::Command as A_CMD;

#[test]
fn test_cat_empty() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.assert().success().stdout("");
}

#[test]
fn test_cat_file_not_exist() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.arg("./not_exist")
        .assert()
        .success()
        .stderr("cat: ./not_exist: No such file or directory (os error 2)");
}

#[test]
fn test_cat_file_exist() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.arg("./tests/cli.rs")
        .assert()
        .success()
        .stdout(std::fs::read_to_string("./tests/cli.rs").unwrap());
}

#[test]
fn test_cat_multiple_files() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.args(&["./tests/cli.rs", "./tests/echo.rs"])
        .assert()
        .success()
        .stdout(
            std::fs::read_to_string("./tests/cli.rs")
                .unwrap()
                .to_string()
                + &std::fs::read_to_string("./tests/echo.rs")
                    .unwrap()
                    .to_string(),
        );
}

#[test]
fn test_cat_n_option() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.args(&["./tests/cli.rs", "-n"]).assert().success().stdout(
        std::fs::read_to_string("./tests/source/cli.n.txt")
            .unwrap()
            .to_string()
    );
}

#[test]
fn test_cat_b_option() {
    let mut c = A_CMD::cargo_bin("cat").unwrap();
    c.args(&["./tests/cli.rs", "-b"]).assert().success().stdout(
        std::fs::read_to_string("./tests/source/cli.b.txt")
            .unwrap()
            .to_string()
    );
}
