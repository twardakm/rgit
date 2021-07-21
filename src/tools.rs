use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Returns default path for scan results
pub fn get_default_scan_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("Failed to get home directory")?
        .join(".rgit"))
}

/// Returns current git username
pub fn get_git_user_name() -> Result<String> {
    let user = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .context("Failed to execute: git config user.name")?;

    let user = String::from_utf8(user.stdout)?;

    Ok(String::from(user.trim()))
}
