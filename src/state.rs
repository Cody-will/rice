use std::{path::PathBuf, str::pattern::Utf8Pattern::StringPattern};

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

use crate::paths;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct State {
    wallpaper: Option<PathBuf>,
    backend: Option<String>,
}


impl Default for State {
    fn default() -> Self {
        Self {
            wallpaper: None,
            backend: None,
        }
    }
}


impl State {

    pub fn load() -> Result<State> {
        let path = paths::state_file()?;

        if !path.exists() {
            return Ok(State::default());  
        }
        
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let state = serde_json::from_str(&raw)
            .with_context(|| format!("failed for parse {}", path.display()))?;

        Ok(state)

    }


    pub fn save(&self) -> Result<()> {
        let path = paths::state_file()?;
        let parent = path.parent().context("Path does not have a parent")?;
        
        paths::ensure_dir(parent)?;

        let json = serde_json::to_string_pretty(self).context("failed to serialize state")?;
        
        std::fs::write(&path, json)
            .with_context(|| format!("failed to write {}", path.display()))?;

        Ok(())
    }
}
