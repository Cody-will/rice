mod bar;
mod cli;
mod cmd;
mod config;
mod error;
mod paths;
mod session;
mod state;
mod theme;
mod wall;
mod notifications;
mod screenshot;

use crate::{cli::ScreenshotAction, config::Config};
use anyhow::Result;
use clap::Parser;
use cli::{BarAction, Cli, Command, SessionAction, WallAction};

fn main() -> Result<()> {
    let result = match Cli::parse().command {
        Command::Wall { action } => match action {
            WallAction::Next => wall::next(),
            WallAction::Prev => wall::prev(),
            WallAction::Start => wall::start(),
        },
        Command::Bar { action } => match action {
            BarAction::Restart => bar::restart(&Config::load()?), 
        },
        Command::Session { action } => match action {
            SessionAction::Start => session::start(),
        },
        Command::Screenshot { action } => match action {
            ScreenshotAction::ScreenshotMonitor => screenshot::caputure_monitor(),
        }
    };
    
    match result {
        Ok(()) => {
            if let Ok(cfg) = config::Config::load() {
                notifications::success(&cfg, "Wallpaper updated");
            }
            Ok(())
        },
        Err(e) => {
            if let Ok(cfg) = config::Config::load() {
                notifications::error(&cfg, &format!("Update failed: {e:#}"));
            }
            eprintln!("{e:#}");
            std::process::exit(1);
        }
    }


}
