use anyhow::Result;

/// Trait describing interface for available operations on repositories
pub trait RepoOperations {
    /// Executing custom git command on a repository
    ///
    /// # Arguments
    ///
    /// * `cmd` - command to execute, e.g. status --porcelain
    fn custom_cmd(&self, cmd: String) -> Result<()>;
}
