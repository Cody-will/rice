use std::path::Path;
use anyhow::{Result, bail, Context};
use crate::config::Config;
use crate::state::State;
use crate::cmd;

pub fn set(path: &Path, backend: &str) -> Result<()> {
    let img_path = path.to_str().context("wallpaper path is not valid utf-8")?;
    match backend {
        "hyprpaper" => cmd::run("hyprctl", &["hyprpaper", "wallpaper", &format!("eDP-1, {img_path}")]),
        "awww" => cmd::run("awww", &["img", img_path, "--transition-type", "grow", "--transition-pos", "top-right", "--transition-duration", "0.7", "--transition-fps", "45", "--transition-step", "60"]),
        "none" => Ok(()),
        _ => bail!("unkown backend {backend}"),
    } 
}


pub fn _toggle(_state: &State, _cfg: &Config) -> Result<()> {
    Ok(())
}
