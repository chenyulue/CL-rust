use assert_cmd::Command;
use predicates::prelude::*;
use std::process;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let output = process::Command::new("c:/ghcup/msys64/usr/bin/echo.exe")
            .arg("Hello     there").output().expect("failed to execute echo");
    
    let mut cmd = Command::cargo_bin("echor")?;
    let echor = cmd.arg("Hello     there").assert().success();
    assert_eq!(output, *echor.get_output());
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let output = process::Command::new("c:/ghcup/msys64/usr/bin/echo.exe")
            .args(vec!["Hello", "there"]).output().expect("failed to execute echo");
    
    let mut cmd = Command::cargo_bin("echor")?;
    let echor = cmd.args(vec!["Hello", "there"]).assert().success();
    assert_eq!(output, *echor.get_output());
    Ok(())
}

#[test]
fn hello1_no_newline() -> TestResult {
    let output = process::Command::new("c:/ghcup/msys64/usr/bin/echo.exe")
            .args(vec!["-n", "Hello      there"]).output().expect("failed to execute echo");
    
    let mut cmd = Command::cargo_bin("echor")?;
    let echor = cmd.args(vec!["-n", "Hello      there"]).assert().success();
    assert_eq!(output, *echor.get_output());
    Ok(())
}

#[test]
fn hello2_no_newline() -> TestResult {
    let output = process::Command::new("c:/ghcup/msys64/usr/bin/echo.exe")
            .args(vec!["-n", "Hello", "there"]).output().expect("failed to execute echo");
    
    let mut cmd = Command::cargo_bin("echor")?;
    let echor = cmd.args(vec!["-n", "Hello", "there"]).assert().success();
    assert_eq!(output, *echor.get_output());
    Ok(())
}