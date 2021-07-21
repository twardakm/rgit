use anyhow::{Context, Result};
use log::{debug, info, trace};

use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use crate::options::opts::ExecOpts;

use super::repo::Repo;
use super::repo_operations::RepoOperations;
use super::repositories::Repositories;

/// Starts `exec` command
///
/// # Arguments
///
/// * `opts` - options from command line
pub fn run(opts: &ExecOpts) -> Result<()> {
    debug!("ENTER exec run: {:?}", opts);

    let repositories = match opts.source_file.as_ref() {
        None => read_repositories_from_stdin()
            .context("Failed to read repositories paths from stdin")?,
        Some(path) => {
            let path = match path {
                Some(path) => PathBuf::from(path),
                None => {
                    crate::tools::get_default_scan_path().context("Failed to get default path")?
                }
            };
            debug!("Reading repositories from file: {}", path.to_str().unwrap());
            read_repositories_from_file(path)
                .context("Failed to read repositories paths from file")?
        }
    };

    if opts.porcelain {
        repositories
            .porcelain()
            .context("Failed to execute porcelain")?;
    }

    match &opts.cmd {
        Some(cmd) => repositories
            .custom_cmd(String::from(cmd))
            .context("Failed to execute command on all repositories")?,
        None => trace!("Skipping cmd command"),
    }

    Ok(())
}

fn read_repositories_from_stdin() -> Result<Repositories> {
    info!("Reading repository paths from stdin");
    let mut repositories = Repositories::new();

    loop {
        let mut line = String::new();
        let n = io::stdin()
            .read_line(&mut line)
            .context("Failed to read from stdin")?;

        let line = line.trim();

        if n == 0 {
            break;
        }

        trace!("Adding path {} to repositories", line);

        match Repo::new(line) {
            Some(repo) => repositories.repos.push(repo),
            None => {}
        }
    }

    Ok(repositories)
}

fn read_repositories_from_file(path: PathBuf) -> Result<Repositories> {
    info!(
        "Reading repository paths from file: {}",
        path.to_str().unwrap()
    );
    let mut repositories = Repositories::new();

    let f =
        File::open(&path).context(format!("Failed to open file: {}", path.to_str().unwrap()))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match Repo::new(&line.context("Failed to read line")?) {
            Some(repo) => repositories.repos.push(repo),
            None => {}
        }
    }

    Ok(repositories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use std::io::LineWriter;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[test]
    fn read_repositories_from_file_test() -> Result<()> {
        let repo_1_dir = TempDir::new()?;
        let repo_2_dir = TempDir::new()?;
        let no_repo_dir = TempDir::new()?;

        let _ = Repository::init(repo_1_dir.path())?;
        let _ = Repository::init(repo_2_dir.path())?;

        let file = NamedTempFile::new()?;

        let mut writer = LineWriter::new(&file);

        writer.write_all(repo_1_dir.path().to_str().unwrap().as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(repo_2_dir.path().to_str().unwrap().as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(no_repo_dir.path().to_str().unwrap().as_bytes())?;
        writer.write_all(b"\n")?;

        let repositories = read_repositories_from_file(PathBuf::from(file.path()))?;

        assert_eq!(repositories.repos.len(), 2);

        Ok(())
    }
}
