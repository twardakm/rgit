use anyhow::{Context, Result};

fn main() -> Result<()> {
    rgit::run().context("Error while running rgit")?;

    Ok(())
}
