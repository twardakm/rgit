use clap::{AppSettings, Clap};

/// rgit allows you to control multiple git repositories at the same time.
/// It is a bit different from `repo` tool since, it does not require  initialization and can work with only selected repositories.
#[derive(Clap)]
#[clap(author = "Marcin Twardak <twardakm@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    ///
    /// 0 - warning
    /// 1 - info
    /// 2 - debug
    /// 3 - trace
    #[clap(short, parse(from_occurrences))]
    pub verbosity: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Scan(ScanOpts),
    Exec(ExecOpts),
}

/// Scans repositories in subdirectories
#[derive(Clap, Debug)]
pub struct ScanOpts {
    /// [Optional] Max depth of subdirectories to scan
    ///
    /// Defines how many levels of directories to scan in search for git repositories
    #[clap(long, short, default_value = "3")]
    pub max_depth: usize,
    /// [Optional] Min depth of subdirectories to scan
    ///
    /// Defines on which level of directory to start scanning, 0 means current working directory
    #[clap(long, default_value = "0")]
    pub min_depth: usize,
    /// [Optional] Save output to the specified file for later use by rgit
    ///
    /// Saves scan results to the file to be used by other rgit commands, default: ~/.rgit
    #[clap(short, long)]
    pub save_to_file: Option<Option<String>>,
    /// [Optional] Generate relative paths instead of absolute ones
    ///
    /// Prints relative paths of repositories
    #[clap(long)]
    pub relative: bool,
}

/// Executes git commands in specified repositories
#[derive(Clap, Debug)]
pub struct ExecOpts {
    /// [Optional] Reads repositories saved in the specified file (by `rgit scan`)
    ///
    /// Reads results from the file instead of stdin, default: ~/.rgit
    #[clap(short, long)]
    pub source_file: Option<Option<String>>,
    /// [Optional] Executes similar command to `git status --porcelain`
    ///
    /// It will display only repositories modified in any way with --porcelain result
    #[clap(long)]
    pub porcelain: bool,
    /// [Optional] Executes custom git command on all repositories
    #[clap(short, long)]
    pub cmd: Option<String>,
}
