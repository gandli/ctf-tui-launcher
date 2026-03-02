# ctf-tui-launcher

A Rust TUI launcher for CTF practice environments, inspired by `CTF-Archives/ctf-docker-template`.

## Goals

- One-command challenge environment startup
- Dynamic flag env injection support (`FLAG`, `GZCTF_FLAG`, `DASCTF`)
- Unified workflow: select challenge -> start env -> solve -> writeup

## Planned MVP

- Challenge list and status in TUI
- `docker compose up -d` / `down` / `logs` shortcuts
- Container status panel
- Writeup markdown scaffold generation

## Stack

- `ratatui`
- `crossterm`
- `serde` + `toml`
- `std::process::Command`
