use anyhow::{Context, Result};
use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init_logging(verbosity: i32) -> Result<()> {
    let logging_level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };

    SimpleLogger::new()
        .with_level(logging_level)
        .init()
        .context("Error while initializing logger")?;

    log::trace!("Initialized logging successfully");

    Ok(())
}
