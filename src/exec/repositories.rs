use super::{repo::Repo, repo_operations::RepoOperations};
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
}

impl RepoOperations for Repositories {
    /// Executes custom git command on all repos
    ///
    /// # Arguments
    ///
    /// * `cmd` - git command to execute
    fn custom_cmd(&self, cmd: String) -> Result<()> {
        debug!("Executing command: {} on all repositories", cmd);

        for repo in &self.repos {
            repo.custom_cmd(String::from(&cmd))
                .context("Failed to execute command on repo")?;
        }
        Ok(())
    }
    /// Executes `git status --porcelain` on all repositories
    fn porcelain(&self) -> Result<()> {
        debug!("Executing git status --porcelain on all repositories");

        for repo in &self.repos {
            repo.porcelain()
                .context("Failed to execute porcelain command")?;
        }

        Ok(())
    }
}
