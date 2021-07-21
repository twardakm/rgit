use super::super::common;
use anyhow::Result;
use std::process::{Command, Stdio};

#[test]
fn cli_print_help_long() -> Result<()> {
    let status = Command::new(common::get_rcterm_exec_path()?)
        .arg("--help")
        .stdout(Stdio::null())
        .status()?;

    assert!(status.success());

    Ok(())
}

#[test]
fn cli_print_help_short() -> Result<()> {
    let status = Command::new(common::get_rcterm_exec_path()?)
        .arg("-h")
        .stdout(Stdio::null())
        .status()?;

    assert!(status.success());

    Ok(())
}
