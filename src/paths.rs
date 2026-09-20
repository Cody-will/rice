//! Filesystem locations for rice.
//!
//! This module does not read or parse config/state. It only answers
//! "where does this path live?" and "make this directory exist."

use std::{
    env::home_dir,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use directories::ProjectDirs;


/// XDG project dirs for the qualifier `rice`
/// (`~/.config/rice`, `~/.cache/rice` on Linux).
///
/// Fails only if the home/XDG layout cannot be determined.
pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "rice").context("could not determine XDG dirs for rice")
}

/// `~/.config/rice`
pub fn config_dir() -> Result<PathBuf> {
    Ok(project_dirs()?.config_dir().to_path_buf())
}

/// `~/.cache/rice`
pub fn cache_dir() -> Result<PathBuf> {
    Ok(project_dirs()?.cache_dir().to_path_buf())
}

/// `~/.config/rice/config.toml`
///
/// Does not create the file. Callers write it if missing.
pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// `~/.config/rice/icon.png`
///
/// Image gets stored in folder on first build
pub fn icon_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("icon.png"))
}

/// `~/.cache/rice/state.json`
///
/// Does not create the file. `state::save` should `ensure_dir` first.
pub fn state_file() -> Result<PathBuf> {
    Ok(cache_dir()?.join("state.json"))
}

/// `mkdir -p` for `path`.
///
/// Pass a **directory** (e.g. `config_file()?.parent()`), not the toml/json file.
/// Existing dirs are left alone.
pub fn ensure_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("failed to create {}", path.display()))
}

/// Expand a leading `~` using `$HOME`.
///
/// - `~` → `/home/you`
/// - `~/Pictures/walls` → `/home/you/Pictures/walls`
/// - anything else is returned unchanged (relative or absolute)
///
/// If `HOME` is unset, `~` is left as `~`.
pub fn expand_tilde(path: &Path) -> PathBuf {
    let raw = path.to_string_lossy();

    if raw == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(raw.as_ref()));
    }

    if let Some(rest) = raw.strip_prefix("~/") {
        let mut home = home_dir().unwrap_or_else(|| PathBuf::from("~"));
        home.push(rest);
        return home;
    }

    path.to_path_buf()
}
