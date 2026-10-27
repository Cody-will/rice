use std::process::Command;
use anyhow::{Context, Result};

pub fn caputure_monitor() -> Result<()> {
    let _ = Command::new("hyprshot")
        .args(["-m", "output", "-m", "eDP-1"])
        .status()
        .with_context(|| "failed to take screenshot".to_string())?;
    Ok(())
}
