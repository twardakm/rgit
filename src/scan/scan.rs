use anyhow::{Context, Result};
use git2::Repository;
use log::{debug, trace};
use walkdir::WalkDir;

use std::fs::File;
use std::io::{prelude::*, LineWriter};
use std::path::{Path, PathBuf};

use crate::options::opts::ScanOpts;

/// Starts `scan` command
///
/// # Arguments
///
/// * `opts` - options from command line
pub fn run(opts: &ScanOpts) -> Result<()> {
    debug!("ENTER scan run: {:?}", opts);

    let repos = get_repo_paths(
        std::env::current_dir()
            .context("Failed to get current directory path")?
            .as_path(),
        opts.min_depth,
        opts.max_depth,
        opts.relative,
    )
    .context("Failed to scan paths")?;

    match &opts.save_to_file {
        None => print_paths_to_stdout(repos),
        Some(save_to_file) => print_paths_to_file(
            repos,
            match save_to_file {
                None => crate::tools::get_default_scan_path()
                    .context("Failed to get default scan path")?,
                Some(path) => PathBuf::from(path),
            },
        )
        .context("Failed to print paths to file")?,
    };

    trace!("EXIT scan run");
    Ok(())
}

fn print_paths_to_stdout(repos: Vec<PathBuf>) {
    for repo in repos {
        println!("{}", repo.display());
    }
}

fn print_paths_to_file(repos: Vec<PathBuf>, path: PathBuf) -> Result<()> {
    debug!("Printing results to file {}", path.display());

    let f = File::create(&path).context(format!("Failed to create file {}", path.display()))?;
    let mut f = LineWriter::new(f);

    for repo in repos {
        f.write_all(
            repo.to_str()
                .context("Error while converting path to str")?
                .as_bytes(),
        )
        .context("Error while writing to file")?;
        f.write_all(b"\n").context("Error while writing to file")?;
    }

    Ok(f.flush().context("Failed to flush file")?)
}

fn get_repo_paths(
    root: &Path,
    min_depth: usize,
    max_depth: usize,
    relative: bool,
) -> Result<Vec<PathBuf>> {
    trace!("ENTER get_repo_paths, root={:?}", root);

    let mut res = Vec::new();

    for entry in WalkDir::new(root).min_depth(min_depth).max_depth(max_depth) {
        let entry = entry.context("Failed to find entry")?;
        let path = entry.path();

        if path.ends_with(".git") {
            continue;
        }

        let path = match relative {
            true => path
                .strip_prefix(std::env::current_dir().context("Failed to get current dir")?)
                .context("Failed to create relative path")?,
            false => path,
        };

        match Repository::open(path) {
            Ok(_) => {
                trace!("Found repository in {:?}", path);
                res.push(PathBuf::from(
                    path.to_str().context("Failed to convert path to str")?,
                ));
            }
            Err(_) => continue,
        }
    }

    debug!(
        "Found {} paths in root={:?}, min_depth={}, max_depth={}",
        res.len(),
        root,
        min_depth,
        max_depth
    );

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use std::io::BufReader;
    use tempfile::tempfile_in;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[test]
    fn get_repo_paths_one_dir() -> Result<()> {
        let root = TempDir::new()?;
        let repo_dir = TempDir::new_in(&root)?;
        let _ = Repository::init(repo_dir.path())?;

        let res = get_repo_paths(root.path(), 0, 3, false)?;

        println!("{:?}", res);

        assert_eq!(res.len(), 1);

        Ok(())
    }

    #[test]
    fn get_repo_paths_multiple_dirs() -> Result<()> {
        let root = TempDir::new()?;

        let dir_lvl_1_1 = TempDir::new_in(&root)?;
        let _ = Repository::init(dir_lvl_1_1.path())?;

        let dir_lvl_1_2 = TempDir::new_in(&root)?;
        let dir_lvl_1_3 = TempDir::new_in(&root)?;

        let _ = TempDir::new_in(&dir_lvl_1_2)?;
        let dir_lvl_2_2 = TempDir::new_in(&dir_lvl_1_2)?;
        let _ = Repository::init(dir_lvl_2_2.path())?;
        let dir_lvl_2_3 = TempDir::new_in(&dir_lvl_1_2)?;
        let _ = Repository::init(dir_lvl_2_3.path())?;

        let dir_lvl_2_4 = TempDir::new_in(&dir_lvl_1_3)?;
        let dir_lvl_3_1 = TempDir::new_in(&dir_lvl_2_4)?;
        let dir_lvl_4_1 = TempDir::new_in(&dir_lvl_3_1)?;
        let _ = Repository::init(dir_lvl_4_1.path())?;

        let res = get_repo_paths(root.path(), 0, 3, false)?;

        println!("{:?}", res);

        assert_eq!(res.len(), 3);

        Ok(())
    }

    #[test]
    fn get_repo_paths_files_and_dirs_inside_repos() -> Result<()> {
        let root = TempDir::new()?;

        let dir_1 = TempDir::new_in(&root)?;
        let _ = Repository::init(dir_1.path())?;

        let _ = TempDir::new_in(&dir_1)?;
        let _ = TempDir::new_in(&dir_1)?;
        let _ = tempfile_in(&dir_1)?;
        let _ = tempfile_in(&dir_1)?;
        let dir_1_1 = TempDir::new_in(&dir_1)?;
        let _ = tempfile_in(&dir_1_1)?;
        let _ = tempfile_in(&dir_1_1)?;

        let dir_2 = TempDir::new_in(&root)?;
        let _ = Repository::init(dir_2.path())?;

        let _ = tempfile_in(&dir_2)?;
        let _ = tempfile_in(&dir_2)?;

        let res = get_repo_paths(root.path(), 0, 3, false)?;

        println!("{:?}", res);

        assert_eq!(res.len(), 2);

        Ok(())
    }

    #[test]
    fn print_paths_to_file_ok() -> Result<()> {
        let temp = NamedTempFile::new()?;
        let vec = vec![
            PathBuf::from("/some"),
            PathBuf::from("/random"),
            PathBuf::from("/vector"),
        ];

        print_paths_to_file(vec, PathBuf::from(temp.path()))?;

        let reader = BufReader::new(temp);

        let mut res = Vec::new();

        for line in reader.lines() {
            res.push(line?);
        }

        assert_eq!(res[0], "/some");
        assert_eq!(res[1], "/random");
        assert_eq!(res[2], "/vector");

        Ok(())
    }
}
