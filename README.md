# ctf-tui-launcher

A Rust TUI launcher for CTF practice environments, inspired by [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template).

## Why

CTF challenge reproduction is often messy: scattered folders, repetitive Docker commands, environment drift, and inconsistent writeups.

`ctf-tui-launcher` aims to standardize the workflow into:

**Select challenge → Start target → Solve → Record → Reproduce**

with a keyboard-first terminal experience.

## Core Goals

- One-key challenge environment startup via Docker
- Dynamic flag environment variable injection (`FLAG`, `GZCTF_FLAG`, `DASCTF`)
- Unified workflow for local CTF practice and writeup creation
- Minimal friction for repeated challenge reproduction

## Planned MVP (v0.1)

- Challenge list and status in TUI (`todo / doing / done`)
- Quick actions:
  - `docker compose up -d`
  - `docker compose down`
  - `docker compose logs -f`
  - open shell in container
- Container/runtime status panel
- Writeup markdown scaffold generation
- Local config loading from `toml`

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
├── Cargo.toml
└── README.md
```

## Development Plan

- [ ] Bootstrap TUI layout (left: challenge list, right: details)
- [ ] Add keymap and command dispatcher
- [ ] Add Docker actions (`up/down/logs/shell`)
- [ ] Add challenge metadata parsing
- [ ] Add writeup generation
- [ ] Add error handling and health indicators

## Run (Current Skeleton)

```bash
cargo run
```

Optional: create `challenges.toml` from `challenges.toml.example` to load your own challenge list.

## Status

M2 skeleton is implemented. M3 has basic Docker actions (`up/down/logs/ps`) and writeup scaffold generation.

If you want to contribute, open an issue with one of these labels:

- `feature:ui`
- `feature:docker`
- `feature:writeup`
- `bug`
