use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn get_default_scan_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("Failed to get home directory")?
        .join(".rgit"))
}
