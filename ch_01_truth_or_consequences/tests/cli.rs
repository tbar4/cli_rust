use assert_cmd::{assert, Command};
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("ch_01_truth_or_consequences").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    assert_eq!(stdout, "Hello, world!\n")
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}