use anyhow::Result;
use std::path::PathBuf;

/// Returns path to executable
pub fn get_rcterm_exec_path() -> Result<PathBuf> {
    Ok(std::env::current_dir()?.join("target/debug/rgit"))
}
