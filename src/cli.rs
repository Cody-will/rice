use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rice", about = "Hyprland helpers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Wall {
        #[command(subcommand)]
        action: WallAction,
    },
    Bar {
        #[command(subcommand)]
        action: BarAction,
    },
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },
    Screenshot {
        #[command(subcommand)]
        action: ScreenshotAction,
    }
}

#[derive(Subcommand)]
pub enum WallAction {
    Next,
    Prev,
    Start,
}

#[derive(Subcommand)]
pub enum BarAction {
    Restart,
}

#[derive(Subcommand)]
pub enum SessionAction {
    Start,
}

#[derive(Subcommand)]
pub enum ScreenshotAction {
    ScreenshotMonitor,
}
