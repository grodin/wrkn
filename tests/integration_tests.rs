use assert_cmd::cargo::CargoError;
use assert_cmd::Command;
use assert_fs::prelude::{FileWriteStr, PathChild, SymlinkToFile};
use assert_fs::{NamedTempFile, TempDir};
use predicates::prelude::predicate;
use std::fs;
use testresult::TestResult;

fn app_cmd_name() -> Result<Command, CargoError> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
}

#[test]
fn cmd_now_creates_file_if_needed() -> TestResult {
    let temp_file = NamedTempFile::new("wrkn_on")?;

    let mut cmd = app_cmd_name()?;

    let assert = cmd
        .args(["--file", &temp_file.path().display().to_string()])
        .args(["now", "Doing something"])
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("Doing something").count(1));

    assert!(temp_file.path().exists());

    Ok(())
}

#[test]
fn cmd_recent_succeeds_if_file_doesnt_exist() -> TestResult {
    let temp_file = NamedTempFile::new("wrkn_on")?;

    let mut cmd = app_cmd_name()?;

    cmd.args(["--file", &temp_file.path().display().to_string()])
        .arg("recent")
        .assert()
        .success();
    Ok(())
}

#[test]
fn cmd_recent_can_read_file() -> TestResult {
    let test_file = include_str!("./sample_file");
    let temp_file = NamedTempFile::new("wrkn_on")?;
    fs::write(temp_file.path(), test_file)?;

    let mut cmd = app_cmd_name()?;

    let all_lines_predicate =
        predicate::function(|stdout: &str| stdout.lines().count() == test_file.lines().count());
    cmd.args(["--file", &temp_file.path().display().to_string()])
        .arg("recent")
        .assert()
        .success()
        .stdout(all_lines_predicate);

    Ok(())
}

#[test]
fn doesnt_overwrite_symlink() -> TestResult {
    let temp_dir = TempDir::new()?;
    let data_file = temp_dir.child("wrkn_on_data");
    data_file.write_str(include_str!("./sample_file"))?;
    let symlink_data_file = temp_dir.child("wrkn_on_symlink");
    symlink_data_file.symlink_to_file(data_file.path())?;

    let mut cmd = app_cmd_name()?;

    cmd.args([
        "--file",
        &symlink_data_file.path().display().to_string(),
        "now",
        "Doing something",
    ])
    .assert()
    .success();

    let metadata = symlink_data_file.symlink_metadata()?;

    assert!(metadata.file_type().is_symlink());
    Ok(())
}
