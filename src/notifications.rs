use std::path::PathBuf;
use anyhow::{Result, Context};
use notify_rust::{Notification, Urgency};
use serde::{Serialize, Deserialize};
use crate::config::Config;


const ICON: &[u8] = include_bytes!("../assets/rice-notify.png");

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Notifications {
    pub on_success: bool,
    pub on_error: bool,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            on_success: false,
            on_error: true,
        }
    }
}


fn ensure_icon() -> Result<PathBuf> {
    let path = crate::paths::icon_file()?;
    if !path.exists() {
        if let Some(dir) = path.parent() {
            crate::paths::ensure_dir(dir)?;
        }
        std::fs::write(&path, ICON)
            .with_context(|| format!("failed to write {}", path.display()))?;
    }
    Ok(path)
}

fn show(body: &str, urgency: Urgency) {
    let Ok(icon) = ensure_icon() else { return };
    let Some(icon) = icon.to_str() else { return };
    let _ = Notification::new()
        .appname("rice")
        .summary("rice")
        .body(body)
        .icon(icon)
        .urgency(urgency)
        .show();
}

pub fn success(cfg: &Config, body: &str) {
    if cfg.notifications.on_success {
        show(body, Urgency::Low);
    }
}

pub fn error(cfg: &Config, body: &str) {
    if cfg.notifications.on_error {
        show(body, Urgency::Normal);
    }
}
