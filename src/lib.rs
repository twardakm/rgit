pub mod exec;
pub mod logging;
pub mod options;
pub mod scan;
pub mod tools;

use anyhow::{Context, Result};
use clap::Clap;
use log::trace;

use options::opts::{Opts, SubCommand};

pub fn run() -> Result<()> {
    let options: Opts = Opts::parse();

    logging::init_logging(options.verbosity).context("Failed to initialize logging")?;

    trace!("ENTER run");

    match options.subcmd {
        SubCommand::Scan(opts) => {
            trace!("scan");
            scan::scan::run(&opts).context("Failed to run scan")?;
        }
        SubCommand::Exec(opts) => {
            trace!("exec");
            exec::exec::run(&opts).context("Failed to run exec")?;
        }
    }

    trace!("EXIT run");
    Ok(())
}
