use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

fn gen_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    
    Ok(())
}

 #[test]
 fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
 }