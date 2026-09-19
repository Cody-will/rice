use std::process::Command;
use anyhow::{bail, Context, Result};

pub fn run(bin: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(bin)
        .args(args)
        .status()
        .with_context(|| format!("failed to start {bin}"))?;
    
    if !status.success() {
        bail!("{bin} exited with {status}");
    }

    Ok(())
}
