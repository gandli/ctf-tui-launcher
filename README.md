# ctf-tui-launcher

A Rust TUI launcher for CTF practice environments, inspired by [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template).

## Why

CTF challenge reproduction is often messy: scattered folders, repetitive Docker commands, environment drift, and inconsistent writeups.

`ctf-tui-launcher` standardizes the workflow into:

**Select challenge → Start target → Solve → Record → Reproduce**

with a keyboard-first terminal experience.

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

## Run

From repository root:

```bash
cargo run
```

Or install and run as a command:

```bash
cargo install --path .
ctf-tui-launcher
# short alias
ctf-tui
```

`ctf-tui-launcher` can be launched from any challenge subdirectory. It will walk upward to detect the project root by looking for `challenges.toml` or `challenges/`.

## Tech Stack

- UI: `ratatui` + `crossterm`
- Config: `serde` + `toml`
- Runtime: `std::process::Command` (Docker CLI)

## Repository Structure

```text
.
├── src/
├── docs/
│   └── PRD.MD
├── challenges.toml.example
├── Cargo.toml
└── README.md
```

## Roadmap (next)

- Better in-app log UX (jump to bottom, tail-follow mode)
- Safer shell/container interaction
- Native challenge metadata discovery (category/difficulty conventions)
- Optional writeup template customization
