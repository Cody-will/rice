use std::path::PathBuf;
use anyhow::{bail, Result};
use std::fs;
use crate::{config::Config, notifications::notification_success, state::State};
use crate::theme;

pub mod backend;


fn list_images(cfg: &Config) -> Result<Vec<PathBuf>> {
    let dir = &cfg.wallpaper_dir;
    let mut images: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().map_or(false, |ext| cfg.is_valid_img(&ext.to_string_lossy().to_lowercase()))
        })
        .collect();

    if images.is_empty() {
        bail!("no wallpapers in {}", dir.display());
    }

    if cfg.sort == "name" {
        images.sort();
    }
    
     Ok(images)
}

fn step(delta: isize) -> Result<()> {
    let cfg = Config::load()?;
    let mut state = State::load()?;
    let images = list_images(&cfg)?;
    let n = images.len() as isize;

    let idx = state.wallpaper
        .as_ref()
        .and_then(|cur| images.iter().position(|p| p == cur))
        .unwrap_or(0) as isize;

    let next = (idx + delta).rem_euclid(n) as usize;
    let backend = state.backend
        .as_deref()
        .unwrap_or(cfg.backend.as_str());

    backend::set(&images[next], backend)?;
    theme::apply(&images[next], &cfg)?;
    state.wallpaper = Some(images[next].clone());
    state.backend = Some(backend.to_string());
    state.save()?;
    notification_success("Success", "Wallpaper and colors saved and set");
    println!("{}", images[next].display());
    Ok(())
}

pub fn next() -> Result<()> {
    step(1) 
}

pub fn prev() -> Result<()> {
    step(-1) 
}



