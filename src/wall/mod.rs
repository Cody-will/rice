use std::path::PathBuf;
use anyhow::{bail, Result};
use std::fs;
use crate::{config::Config, state::State};

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
    state.wallpaper = Some(images[next].clone());
    state.save()?;
    println!("{}", images[next].display());
    Ok(())
}

pub fn next() -> Result<()> {
    step(1) 
}

pub fn prev() -> Result<()> {
    step(-1) 
}



