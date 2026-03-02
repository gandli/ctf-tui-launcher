# ctf-tui-launcher

A Rust TUI launcher for CTF practice environments, inspired by [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template).

## Why

CTF 复现的常见痛点是：目录杂、命令多、环境易出错、复盘难沉淀。

`ctf-tui-launcher` 的目标是把流程统一成：

**选题 → 起靶机 → 做题 → 记录 → 复现**

并且尽量做到“键盘内完成”。

## Core Goals

- One-key challenge environment startup via Docker
- Dynamic flag env injection support (`FLAG`, `GZCTF_FLAG`, `DASCTF`)
- Unified workflow for local CTF practice and writeup
- Minimal friction for repeated challenge reproduction

## Planned MVP (v0.1)

- Challenge list and status in TUI (todo / doing / done)
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
- [ ] Add docker actions (`up/down/logs/shell`)
- [ ] Add challenge metadata parsing
- [ ] Add writeup generation
- [ ] Add error handling and health indicators

## Status

Currently in bootstrap stage.

If you want to contribute, open an issue with one of these labels:
- `feature:ui`
- `feature:docker`
- `feature:writeup`
- `bug`
