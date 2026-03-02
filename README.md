# ctf-tui-launcher

**English** | [中文](README_CN.md)

A Rust TUI launcher for CTF practice environments, inspired by [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template).

## Quick Start (Install + Use)

## Prerequisites

Before using `ctf-tui`, make sure these are installed:

- **Docker** (required for challenge environments)
- Git
- Rust / Cargo

Check dependencies:

```bash
docker --version
docker compose version
git --version
cargo --version
```

If Docker is missing:
- macOS / Windows: install Docker Desktop
- Linux: install Docker Engine + Docker Compose plugin

### Install by OS

#### macOS

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

#### Linux

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

#### Windows (PowerShell)

```powershell
git clone https://github.com/gandli/ctf-tui-launcher.git
cd ctf-tui-launcher

# install Rust (if cargo is missing)
winget install Rustlang.Rustup
# restart PowerShell after install, then:
cargo install --path .
```

### Package manager one-liner installs (planned distribution)

After package publishing is completed, you can install directly:

#### Homebrew (macOS / Linux)

```bash
brew tap gandli/ctf-tui
brew install ctf-tui
```

#### Scoop (Windows)

```powershell
scoop bucket add gandli https://github.com/gandli/scoop-bucket
scoop install ctf-tui
```

#### Winget (Windows)

```powershell
winget install gandli.ctf-tui
```

#### Chocolatey (Windows)

```powershell
choco install ctf-tui -y
```

These packages are configured to declare Docker as dependency metadata in their manifests/templates.

### Universal source install (all platforms)

```bash
# 1) clone repository
git clone https://github.com/gandli/ctf-tui-launcher.git
cd ctf-tui-launcher

# 2) install Rust toolchain if cargo is missing
# macOS / Linux:
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

# Windows (PowerShell):
# winget install Rustlang.Rustup

# 3) install the CLI
cargo install --path .
```

After install, both commands are available:

- `ctf-tui` (short alias, recommended)
- `ctf-tui-launcher` (full name)

### First run

```bash
ctf-tui init
ctf-tui doctor
ctf-tui tui
```

### Instant demo (ready-made examples)

```bash
cp examples/challenges.toml ./challenges.toml
ctf-tui doctor
ctf-tui tui
```

You can run `ctf-tui` from any challenge subdirectory. The tool walks upward to detect project root by checking `challenges.toml` or `challenges/`.

---

## CLI Usage (`ctf-tui`)

```bash
ctf-tui tui        # start interactive TUI (default if omitted)
ctf-tui init       # create challenges.toml from template (if missing)
ctf-tui doctor     # inspect workspace/challenges and compose availability
ctf-tui help       # show command help
```

Equivalent full-name usage:

```bash
ctf-tui-launcher tui
ctf-tui-launcher doctor
```

## Current Features (M2 + M3)

- Split-pane TUI (challenge list + details panel)
- Challenge status tracking (`todo / doing / done`)
- Docker actions:
  - `u`: `docker compose up -d`
  - `d`: `docker compose down`
- In-app logs panel:
  - `l` to open/close
  - `r` to refresh logs (inside panel)
  - `j/k` or arrow keys to scroll
- `s` to open shell in selected challenge workdir (returns to TUI on exit)
- `w` to generate writeup scaffold at `writeups/<challenge>.md`
- `t` to cycle challenge status (`todo -> doing -> done -> todo`)
- Status persistence to `challenges.toml` when config file is present
- Compose file validation before Docker actions
- Auto-discovery fallback when no `challenges.toml` is provided
- Built-in guide panel for adding challenges (`g`)
- One-key config bootstrap (`a`) to create `challenges.toml`

## Configuration

### Guided setup (recommended)

Use the built-in guide flow inside TUI:

1. Run `ctf-tui tui`
2. Press `g` to open the guide panel
3. Press `a` to generate `challenges.toml`
4. Edit generated `challenges.toml`
5. Back in TUI, press `r` to reload

Or use CLI-only guided bootstrap:

```bash
ctf-tui init
ctf-tui doctor
ctf-tui tui
```

### Option A: Explicit config (`challenges.toml`)

Copy `challenges.toml.example` to `challenges.toml` and edit fields.

```toml
[[challenges]]
name = "rsa-baby"
category = "Crypto"
difficulty = "Easy"
status = "todo"
description = "Recover plaintext using weak RSA key setup."
workdir = "./challenges/rsa-baby/docker"
```

Built-in example categories in `challenges.toml.example`:

- Crypto
- Pwn
- Web
- Reverse
- Forensics
- Misc
- PPC
- Blockchain

### Option B: Auto-discovery

If `challenges.toml` is missing, the app scans:

- `./challenges/*/docker`

and includes directories containing one of:

- `docker-compose.yml`
- `docker-compose.yaml`
- `compose.yml`
- `compose.yaml`

## Keymap

### Main view

- `j/k` or `↑/↓`: move selection
- `u`: start environment
- `d`: stop environment
- `l`: open logs panel
- `s`: open shell in workdir
- `w`: generate writeup
- `t`: cycle challenge status
- `r`: reload challenges
- `a`: create `challenges.toml` from template
- `g`: open add-challenge guide panel
- `q`: quit

### Logs panel

- `j/k` or `↑/↓`: scroll logs
- `r`: refresh logs
- `Esc` or `l`: close logs panel
- `q`: quit app

### Guide panel

- `a`: generate `challenges.toml`
- `Esc` or `g`: close guide

## Example Challenge Pack

- `examples/challenges/` includes 8 category demo challenge folders
- each demo has `docker/docker-compose.yml` for immediate testing
- `examples/challenges.toml` is ready to use

## Binary Release + Package Managers

The project includes a release workflow:

- `.github/workflows/release.yml`
- trigger by tag: `v*` (example: `v0.1.0`)
- publishes binaries + `checksums.txt` to GitHub Releases

Packaging templates are provided in `packaging/` for:

- Homebrew
- Scoop
- Winget
- Chocolatey
- AUR

## Tech Stack

- UI: `ratatui` + `crossterm`
- Config: `serde` + `toml`
- Runtime: `std::process::Command` (Docker CLI)
