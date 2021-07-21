use anyhow::{Context, Result};
use git2::Repository;
use log::{trace, warn};
use termion::color;

use std::path::PathBuf;
use std::process::Command;

use super::repo_operations::RepoOperations;

/// Struct describing single repository
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    /// Creates new Repo struct from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the repository
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::TempDir;
    /// use git2::Repository;
    ///
    /// let path = TempDir::new().unwrap();
    /// let _ = Repository::init(path.path()).unwrap();
    /// let repo = rgit::exec::repo::Repo::new(path.path().to_str().unwrap());
    /// ```
    pub fn new(path: &str) -> Option<Repo> {
        match Repository::open(path) {
            Ok(_) => {
                trace!("Create repo struct for path: {}", path);
                Some(Repo {
                    path: PathBuf::from(path),
                })
            }
            Err(_) => {
                warn!("Did not find git repository in provided path: {}", path);
                None
            }
        }
    }

    fn print_path(&self) {
        print!(
            "{}{}\n{}{}{}\n",
            color::Bg(color::Rgb(32, 32, 32)),
            color::Fg(color::Blue),
            self.path.to_str().unwrap(),
            color::Reset.fg_str(),
            color::Reset.bg_str()
        );
    }
}

impl RepoOperations for Repo {
    /// Executes custom git command on a repository
    ///
    /// # Arguments
    ///
    /// * `cmd` - git command to execute
    fn custom_cmd(&self, cmd: String) -> Result<()> {
        trace!(
            "Executing command {} on repo located in {}",
            cmd,
            self.path.to_str().unwrap()
        );

        self.print_path();

        let args: Vec<&str> = cmd.split(" ").collect();

        Command::new("git")
            .current_dir(self.path.to_str().unwrap())
            .args(args)
            .status()
            .context(format!("Failed to execute: git {}", cmd))?;

        Ok(())
    }
    /// Executes `git status --porcelain` on the repository
    fn porcelain(&self) -> Result<()> {
        let output = Command::new("git")
            .current_dir(self.path.to_str().unwrap())
            .arg("status")
            .arg("--porcelain")
            .output()
            .context("Failed to execute: git status --porcelain")?;

        if output.stdout.len() == 0 {
            trace!(
                "Skipping status --porcelain on {}",
                self.path.to_str().unwrap()
            );
            return Ok(());
        }

        self.print_path();

        Ok(println!("{}", String::from_utf8_lossy(&output.stdout)))
    }
    /// Finds repositories which have cherry-picks in history
    fn find_cherry_picks(&self) -> Result<Option<String>> {
        let reflog = Command::new("git")
            .current_dir(self.path.to_str().unwrap())
            .arg("reflog")
            .output()
            .context("Failed to execute: git reflog")?;
        let reflog = String::from_utf8_lossy(&reflog.stdout);

        if reflog.contains("cherry-pick") {
            self.print_path();
            return Ok(Some(reflog.to_string()));
        }

        Ok(None)
    }
    /// Prints all cherry picks found in history
    fn print_cherry_picks(&self) -> Result<()> {
        match self
            .find_cherry_picks()
            .context("Failed to find cherry picks")?
        {
            Some(reflog) => {
                trace!("Printing cherry-picks line by line");

                let _: Vec<_> = reflog
                    .lines()
                    .filter(|&line| line.contains("cherry-pick"))
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|cherry_pick| println!("{}", cherry_pick))
                    .collect();
            }
            None => {}
        }

        Ok(())
    }
    /// Print repository if there is an author in last `number` of commits
    ///
    /// # Arguments
    ///
    /// * `number` - last number of commits to look into
    /// * `author` - author to look for
    fn print_commits_with_author(&self, number: u32, author: &str) -> Result<()> {
        let number = format!("-{}", number);
        let args = vec![
            "log",
            "--graph",
            "--pretty=%h -%d %s (%cr) <%an>",
            "--abbrev-commit",
            &number,
        ];

        let log = Command::new("git")
            .current_dir(self.path.to_str().unwrap())
            .args(args)
            .output()
            .context("Failed to execute: git log")?;

        let log = String::from_utf8(log.stdout)?;

        let commits = log
            .lines()
            .filter(|&line| line.contains(author))
            .collect::<Vec<&str>>();

        if commits.len() == 0 {
            trace!(
                "Skipping printing commits with author for {}",
                self.path.to_str().unwrap()
            );
            return Ok(());
        }

        self.print_path();

        let _: Vec<_> = commits
            .iter()
            .map(|commit| println!("{}", commit))
            .collect();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use tempfile::TempDir;

    #[test]
    fn repo_provide_path_without_git_repository() -> Result<()> {
        let path = TempDir::new()?;

        match Repo::new(path.path().to_str().unwrap()) {
            Some(_) => bail!("Oops, repository doesn't exist, repo should not be created"),
            None => Ok(()),
        }
    }

    #[test]
    fn repo_provide_path_with_git_repository() -> Result<()> {
        let path = TempDir::new()?;

        let _ = Repository::init(path.path())?;

        match Repo::new(path.path().to_str().unwrap()) {
            Some(_) => Ok(()),
            None => bail!("Oops, repository exist, repo should be created"),
        }
    }
}
