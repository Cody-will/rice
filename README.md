# rice

Small Rust CLI for a Hyprland desktop: wallpapers, wallust, the bar, and session startup.

Hyprland keybinds call this binary. The binary calls `wallust`, `waybar`, and the wallpaper backend. Logic lives here instead of a pile of shell scripts.

```text
SUPER + W         →  rice wall next
SUPER + SHIFT + W →  rice wall prev
SUPER + O         →  rice wall backend toggle
SUPER + R         →  rice bar restart
```

## Commands

```text
rice wall next
rice wall prev
rice wall start
rice wall backend toggle

rice bar restart
rice session start
```

`rice --help` and `rice wall --help` list the same thing.

## Config

`~/.config/rice/config.toml`

```toml
wallpaper_dir = "~/Pictures/walls"
backend = "swww"          # swww | hyprpaper | mpvpaper | none
sort = "name"             # name | mtime | random
extensions = ["jpg", "jpeg", "png", "webp"]

[theme]
enable = true
command = "wallust"
args = []

[bar]
restart_on_theme = true
command = "waybar"

[session]
exec = [
  "waybar",
  "swaync",
]
```

Missing keys use defaults in code. Paths with `~` are expanded.

## State

`~/.cache/rice/state.json` — current wallpaper path and backend. Not meant to be edited.

## Install

Needs a recent Rust toolchain (`rustup` or Arch `rust`).

```bash
git clone git@github.com:Skubaaaaa/rice.git
cd rice
cargo install --path .
```

`rice` ends up in `~/.cargo/bin`. Put that on `PATH`.

Rebuild after changes:

```bash
cargo install --path .
```

## Hyprland

```lua
local rice = "rice"

hl.bind(mainMod .. " + W",         hl.dsp.exec_cmd(rice .. " wall next"))
hl.bind(mainMod .. " + SHIFT + W", hl.dsp.exec_cmd(rice .. " wall prev"))
hl.bind(mainMod .. " + O",         hl.dsp.exec_cmd(rice .. " wall backend toggle"))
hl.bind(mainMod .. " + R",         hl.dsp.exec_cmd(rice .. " bar restart"))

hl.on("hyprland.start", function()
  hl.exec_cmd(rice .. " session start")
end)
```

## Layout

```text
src/
  main.rs      clap entry
  cli.rs       subcommands
  config.rs    config.toml
  state.rs     state.json
  wall/        next / prev / backend
  theme.rs     wallust
  bar.rs
  session.rs
```

## Dev

```bash
cargo fmt
cargo clippy
cargo build --release
```
