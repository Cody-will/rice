use serde::Deserialize;
use anyhow::{Context, Result};
use crate::{cmd, config::Config};

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Bar {
    pub restart_on_theme: bool,
    pub command: String,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            restart_on_theme: true,
            command: "waybar".into(),
        }
    } 
}

pub fn restart(cfg: &Config) -> Result<()> {
    cmd::run("pkill", &[&cfg.bar.command])?;
    std::process::Command::new(&cfg.bar.command)
        .spawn()
        .with_context(|| format!("failed to start {}", cfg.bar.command))?;
    Ok(())
} 
