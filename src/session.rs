use serde::Deserialize;
use anyhow::{Context, Result};
use crate::config::Config;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Session {
    pub exec: Vec<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            exec: vec!["waybar".into(), "swaync".into(), "discord".into()],
        }
    }
}

pub fn start() -> Result<()> {
    let cfg = Config::load()?;
    
    for exec in cfg.session.exec {
        let notifications: String = "swaync".into();
        if exec == notifications {
            let success = std::process::Command::new("pkill mako").spawn();
            match success {
                Ok(_) => { println!("pkill mako success")},
                Err(_) => { println!("mako not active, could not kill") },
            }
        }

        std::process::Command::new(&exec)
            .spawn()
            .with_context(|| format!("failed to start {exec}"))?;
    };    

    crate::wall::start()?;
    Ok(()) 
}
