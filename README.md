# ctf-tui-launcher

**English** | [中文](README_CN.md)

A Rust TUI launcher for CTF practice environments, inspired by [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template).

## Quick Start (Install + Use)

### Install (recommended one-liner)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

### Or install from source

```bash
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

## Tech Stack

- UI: `ratatui` + `crossterm`
- Config: `serde` + `toml`
- Runtime: `std::process::Command` (Docker CLI)
