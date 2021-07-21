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
#[clap(setting = AppSettings::ColoredHelp)]
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
///
/// EXAMPLES:
///
/// rgit exec -s --print-cherry-picks --porcelain
///
/// rgit scan --relative | rgit exec --print-cherry-picks
///
/// rgit exec -s --print-cherry-picks --porcelain --with-author
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ExecOpts {
    /// [Optional] Reads repositories saved in the specified file (by `rgit scan`)
    ///
    /// Reads results from the file instead of stdin, default: ~/.rgit
    #[clap(short, long)]
    pub source_file: Option<Option<String>>,
    /// [Optional] Executes similar command to `git status --porcelain`
    ///
    /// It will display only repositories modified in any way with `status --porcelain` result
    #[clap(long)]
    pub porcelain: bool,
    /// [Optional] Founds repositories which have cherry-picks in git reflog and prints them
    #[clap(long)]
    pub find_cherry_picks: bool,
    /// [Optional] Prints cherry-picks titles in repositories which have cherry-picks in git reflog
    #[clap(long)]
    pub print_cherry_picks: bool,
    /// [Optional] Prints repositories in which current user has commits in last `--number` of commits
    ///
    /// By default it looks for commits which belong to current git user
    #[clap(long)]
    pub with_author: Option<Option<String>>,
    /// [Optional] Used for options looking through git log, e.g. `--with-author`, `--show`.
    /// Specifies number of last commits to look into, default: 10
    ///
    /// Ignored when either `--after` or `--before` is specified
    #[clap(short, long, default_value = "10")]
    pub number: u32,
    /// [Optional] Executes custom git command on all repositories
    #[clap(short, long)]
    pub cmd: Option<String>,
}
