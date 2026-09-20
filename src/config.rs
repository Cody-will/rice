use std::path::PathBuf;
use anyhow::{Context, Result};
use serde::{Deserialize};
use crate::notifications::Notifications;
use crate::paths;
use crate::theme::Theme;
use crate::bar::Bar;
use crate::session::Session;

const DEFAULT: &str = include_str!("../examples/config.toml");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub wallpaper_dir: PathBuf,
    pub backend: String,
    pub sort: String,
    pub extensions: Vec<String>,
    pub theme: Theme,
    pub bar: Bar,
    pub session: Session,
    pub notifications: Notifications,
}


impl Config {
    pub fn load() -> Result<Config> {
        let path = paths::config_file()?;

        if !path.exists() {
            let parent = path.parent().context("Config path has no parent")?;
            paths::ensure_dir(parent)?;
            std::fs::write(&path, DEFAULT)
                .with_context(|| format!("failed to write {}", path.display()))?;
        }

        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let mut cfg: Config = toml::from_str::<Config>(&raw)
            .with_context(|| format!("failed to parse {}", path.display()))?;

        cfg.wallpaper_dir = paths::expand_tilde(&cfg.wallpaper_dir);

        Ok(cfg)
    }
    
    pub fn is_valid_img(&self, img_ext: &str) -> bool {
        self.extensions.contains(&img_ext.to_string())
    }
}








