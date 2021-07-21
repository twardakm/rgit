use super::repo::Repo;
use anyhow::{Context, Result};
use log::debug;

/// Struct describing all repositories `rgit` is working on
pub struct Repositories {
    pub repos: Vec<Repo>,
}

impl Repositories {
    /// Creates new instance of `Repositories`
    pub fn new() -> Repositories {
        Repositories { repos: Vec::new() }
    }

    /// Executes custom git command on all repos
    ///
    /// # Arguments
    ///
    /// * `cmd` - git command to execute
    pub fn custom_cmd(&self, cmd: String) -> Result<()> {
        debug!("Executing command: {} on all repositories", cmd);

        for repo in &self.repos {
            repo.custom_cmd(String::from(&cmd))
                .context("Failed to execute command on repo")?;
        }
        Ok(())
    }
}
