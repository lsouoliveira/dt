use assert_fs::prelude::*;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_reload() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  reload: echo 'reload command'")?;

    let mut cmd = Command::cargo_bin("dt")?;
    cmd.arg("--reload");
    cmd.env("DT_CONFIG_FILE", file.path());
    cmd.assert().success()
        .stdout(predicate::str::contains("reload command"));

    Ok(())
}

#[test]
fn test_open() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  open: echo 'open command'")?;

    let mut cmd = Command::cargo_bin("dt")?;
    cmd.arg("--open");
    cmd.env("DT_CONFIG_FILE", file.path());
    cmd.assert().success()
        .stdout(predicate::str::contains("open command"));

    Ok(())
}

#[test]
fn test_custom() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  custom: echo 'custom command'")?;

    let mut cmd = Command::cargo_bin("dt")?;
    cmd.arg("custom");
    cmd.env("DT_CONFIG_FILE", file.path());
    cmd.assert().success()
        .stdout(predicate::str::contains("custom command"));

    Ok(())
}

#[test]
fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  sync: git")?;

    let mut cmd = Command::cargo_bin("dt")?;
    cmd.arg("--sync");
    cmd.env("DT_CONFIG_FILE", file.path());
    cmd.assert().success()
        .stderr(predicate::str::contains("git"));

    Ok(())
}
