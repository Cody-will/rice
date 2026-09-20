use serde::Deserialize;
use crate::config::Config;
use std::path::Path;
use anyhow::{Result, Context};
use crate::cmd;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Theme {
    enable: bool,
    command: String,
    args: Option<Vec<String>>
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            enable: true,
            command: "wallust".into(),
            args: None,
        }
    }
}

pub fn apply(path: &Path, cfg: &Config) -> Result<()> {
    if !cfg.theme.enable {
        return Ok(());
    }

    let img = path
        .to_str()
        .context("wallpaper path is not valid utf-8")?;

    let mut args: Vec<String> = vec![img.to_string()];
    if let Some(extra) = &cfg.theme.args {
        args.extend(extra.iter().cloned());
    }

    let args_ref: Vec<&str> = args.iter().map(String::as_str).collect();
    cmd::run(&cfg.theme.command, &args_ref)
}
