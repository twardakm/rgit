use anyhow::Result;

/// Trait describing interface for available operations on repositories
pub trait RepoOperations {
    /// Executing custom git command on a repository
    ///
    /// # Arguments
    ///
    /// * `cmd` - command to execute, e.g. status --porcelain
    fn custom_cmd(&self, cmd: String) -> Result<()>;
    /// Executing `git status --porcelain` on the repository and displaying result if it's not clean.
    /// It doesn't display anything on a clean repository.
    fn porcelain(&self) -> Result<()>;
}
