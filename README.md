<div id="top">

<div align="center">

<img src="rice.png" width="30%" alt="Project Logo"/>

# RICE

<em>Wallpaper, wallust, and session helpers for Hyprland</em>

<img src="https://img.shields.io/github/last-commit/Cody-will/rice?style=flat&logo=git&logoColor=white&color=0080ff" alt="last-commit">
<img src="https://img.shields.io/github/languages/top/Cody-will/rice?style=flat&color=0080ff" alt="repo-top-language">
<img src="https://img.shields.io/github/languages/count/Cody-will/rice?style=flat&color=0080ff" alt="repo-language-count">

<em>Built with:</em>

<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white" alt="Rust">
<img src="https://img.shields.io/badge/TOML-9C4121.svg?style=flat&logo=TOML&logoColor=white" alt="TOML">

</div>

<br>

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Install](#install)
  - [Config](#config)
  - [Usage](#usage)
  - [Hyprland](#hyprland)
- [Development](#development)
- [Contributing](#contributing)

---

## Overview

`rice` is a small Rust CLI for a Hyprland desktop. Keybinds call one binary. That binary cycles wallpapers, runs wallust, restarts waybar, and can start a session.

```text
SUPER + W         →  rice wall next
SUPER + SHIFT + W →  rice wall prev
SUPER + R         →  rice bar restart
```

---

## Features

- Wallpaper next/prev/start with **hyprpaper** or **swww**
- Optional **wallust** after a change
- Optional **waybar** restart after a theme
- Session start: spawn listed apps, then restore the last wall
- Config in `~/.config/rice/config.toml` (written from the example on first run)
- State in `~/.cache/rice/state.json`

---

## Project Structure

```text
rice/
├── Cargo.toml
├── examples/
│   └── config.toml
├── rustfmt.toml
└── src/
    ├── main.rs
    ├── cli.rs
    ├── paths.rs
    ├── config.rs
    ├── state.rs
    ├── cmd.rs
    ├── theme.rs
    ├── bar.rs
    ├── session.rs
    └── wall/
        ├── mod.rs
        └── backend.rs
```

---

## Getting Started

### Prerequisites

- Rust toolchain with Cargo — [rustup](https://rustup.rs/) or Arch `sudo pacman -S rust`
- A Hyprland session
- At least one wallpaper backend: `hyprpaper` and/or `swww`
- `wallust` if you want colors to follow the wall
- `waybar` if you use `rice bar restart`

### Install

The binary is installed into `~/.cargo/bin`. That directory must be on your `PATH`.

**From GitHub (no clone):**

```sh
cargo install --git https://github.com/Cody-will/rice.git
```

**From a clone:**

```sh
git clone https://github.com/Cody-will/rice.git
cd rice
cargo install --path .
```

Check:

```sh
which rice
rice --help
```

If `which rice` is empty:

```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc   # or ~/.zshrc
```

Open a new terminal. `cargo build` only builds in `target/`; it does **not** put `rice` on `PATH`. Use `cargo install`.

Reinstall after you pull changes:

```sh
cd rice
git pull
cargo install --path .
```

### Config

First command creates `~/.config/rice/config.toml` from `examples/config.toml` if it's missing.

```toml
wallpaper_dir = "~/Pictures/walls"
backend = "hyprpaper"    # hyprpaper | swww | none
sort = "name"
extensions = ["jpg", "jpeg", "png", "webp"]

[theme]
enable = true
command = "wallust"
args = []                 # apply() should call: wallust run <image>

[bar]
restart_on_theme = true
command = "waybar"

[session]
exec = ["waybar", "swaync"]
```

State (current wall + backend) is `~/.cache/rice/state.json`. Don't edit it by hand.

**hyprpaper:** run the daemon from Hyprland, not a throwaway terminal. Give `~/.config/hypr/hyprpaper.conf` a real target so it creates a layer:

```conf
ipc = on
preload = /home/YOU/Pictures/walls/example.jpg
wallpaper = eDP-1,/home/YOU/Pictures/walls/example.jpg
```

### Usage

```text
rice wall next
rice wall prev
rice wall start

rice bar restart
rice session start
```

`rice session start` spawns `[session].exec`, then `rice wall start`. Start `hyprpaper`/`swww` from Hyprland **before** that if `wall start` talks to them over IPC.

### Hyprland

```lua
hl.on("hyprland.start", function()
  hl.exec_cmd("hyprpaper")
  hl.exec_cmd("rice session start")
end)

hl.bind("SUPER + W",         hl.dsp.exec_cmd("rice wall next"))
hl.bind("SUPER + SHIFT + W", hl.dsp.exec_cmd("rice wall prev"))
hl.bind("SUPER + R",         hl.dsp.exec_cmd("rice bar restart"))
```

---

## Development

```sh
git clone https://github.com/Cody-will/rice.git
cd rice
cargo build
cargo run -- wall next
cargo fmt
cargo clippy
cargo test
```

`cargo run -- wall next` uses the debug build in this repo. The Hyprland bind uses whatever `cargo install` put on `PATH`. Reinstall after you change code you want on a keybind.

---

## Contributing

Issues and pull requests are welcome.

1. Fork the repo
2. Branch from `main`
3. `cargo fmt` and `cargo clippy` before you open the PR
4. Describe the command you changed (`rice wall …`, session, config)

Bug reports: include `rice --help` output, backend (`hyprpaper`/`swww`), and the exact command that failed.

---

<div align="left"><a href="#top">⬆ Return</a></div>
