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
    /// Finds repositories which have cherry-picks in history
    fn find_cherry_picks(&self) -> Result<Option<String>>;
    /// Prints all cherry picks found in history
    fn print_cherry_picks(&self) -> Result<()>;
    /// Print repository and commits with author if there is one in last `number` of commits
    ///
    /// # Arguments
    ///
    /// * `number` - last number of commits to look into
    /// * `author` - author to look for
    fn print_commits_with_author(&self, number: u32, author: &str) -> Result<()>;
}
