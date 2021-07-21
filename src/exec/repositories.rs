use super::{repo::Repo, repo_operations::RepoOperations};
use anyhow::{Context, Result};
use log::debug;
use termion::color;

/// Struct describing all repositories `rgit` is working on
pub struct Repositories {
    pub repos: Vec<Repo>,
}

impl Repositories {
    /// Creates new instance of `Repositories`
    pub fn new() -> Repositories {
        Repositories { repos: Vec::new() }
    }
    /// Pretty prints title of executed command
    fn print_title(&self, title: &str) {
        print!(
            "{}\n{}{}{}\n",
            color::Fg(color::Red),
            title,
            color::Reset.fg_str(),
            color::Reset.bg_str()
        );
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

        self.print_title(&format!("git {}", cmd));

        for repo in &self.repos {
            repo.custom_cmd(String::from(&cmd))
                .context("Failed to execute command on repo")?;
        }
        Ok(())
    }
    /// Executes `git status --porcelain` on all repositories
    fn porcelain(&self) -> Result<()> {
        debug!("Executing git status --porcelain on all repositories");

        self.print_title("git status --porcelain");

        for repo in &self.repos {
            repo.porcelain()
                .context("Failed to execute porcelain command")?;
        }

        Ok(())
    }
    /// Finds repositories which have cherry-picks in history
    fn find_cherry_picks(&self) -> Result<Option<String>> {
        debug!("Trying to find all repositories which have cherry-picks in history");

        self.print_title("repositories with cherry-picks in git reflog");

        for repo in &self.repos {
            repo.find_cherry_picks()
                .context("Failed to find cherry picks")?;
        }

        Ok(None)
    }
    /// Prints all cherry picks found in history
    fn print_cherry_picks(&self) -> Result<()> {
        debug!("Prints all cherry picks found in history");

        self.print_title("repositories with cherry-picks in git reflog");

        for repo in &self.repos {
            repo.print_cherry_picks()
                .context("Failed to print cherry picks")?;
        }

        Ok(())
    }
    /// Print repositories for which there is an author in last `number` of commits
    ///
    /// # Arguments
    ///
    /// * `number` - last number of commits to look into
    /// * `author` - author to look for
    fn print_commits_with_author(&self, number: u32, author: &str) -> Result<()> {
        debug!(
            "Prints all repositories which have author in last {} commits",
            number
        );

        self.print_title(&format!(
            "repositories with author {} in last {} commits",
            author, number
        ));

        for repo in &self.repos {
            repo.print_commits_with_author(number, author)
                .context("Failed to print commits with author")?;
        }

        Ok(())
    }
}
