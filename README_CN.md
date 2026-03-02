# ctf-tui-launcher

用于 CTF 复现练题的 Rust TUI 启动器，灵感来自 [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template)。

## 快速开始（安装 + 使用）

### 一键安装（推荐）

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

### 或从源码安装

```bash
cargo install --path .
```

安装后可用命令：

- `ctf-tui`（推荐短命令）
- `ctf-tui-launcher`（完整命令）

### 首次使用

```bash
ctf-tui init
ctf-tui doctor
ctf-tui tui
```

支持在任意题目子目录运行，会自动向上查找项目根目录（`challenges.toml` 或 `challenges/`）。

---

## CLI 子命令

```bash
ctf-tui tui        # 进入交互式 TUI（默认）
ctf-tui init       # 生成 challenges.toml（若不存在）
ctf-tui doctor     # 检查题目目录与 compose 文件
ctf-tui help       # 查看帮助
```

## 当前能力（M2 + M3）

- 左右分栏 TUI（题目列表 + 详情面板）
- 题目状态管理（`todo / doing / done`）
- Docker 动作：
  - `u` 启动：`docker compose up -d`
  - `d` 停止：`docker compose down`
- 日志面板：
  - `l` 打开/关闭
  - `r` 刷新
  - `j/k` 或方向键滚动
- `s` 打开当前题目 workdir shell（退出后返回 TUI）
- `w` 生成 writeup 模板（`writeups/<challenge>.md`）
- `t` 切换状态（`todo -> doing -> done -> todo`）
- 有 `challenges.toml` 时状态自动持久化
- 执行 Docker 前进行 compose 文件校验
- 无配置文件时自动发现题目目录
- `g` 打开新增题目引导面板
- `a` 一键生成 `challenges.toml`

## 配置方式

### 方式 A：`challenges.toml` 显式配置

复制 `challenges.toml.example` 为 `challenges.toml` 并编辑：

```toml
[[challenges]]
name = "rsa-baby"
category = "Crypto"
difficulty = "Easy"
status = "todo"
description = "Recover plaintext using weak RSA key setup."
workdir = "./challenges/rsa-baby/docker"
```

`challenges.toml.example` 已包含题型示例：

- Crypto
- Pwn
- Web
- Reverse
- Forensics
- Misc
- PPC
- Blockchain

### 方式 B：自动发现

当不存在 `challenges.toml` 时，自动扫描：

- `./challenges/*/docker`

并识别以下 compose 文件之一：

- `docker-compose.yml`
- `docker-compose.yaml`
- `compose.yml`
- `compose.yaml`

## 键位说明

### 主界面

- `j/k` 或 `↑/↓`：切换题目
- `u`：启动环境
- `d`：停止环境
- `l`：打开日志面板
- `s`：打开 shell
- `w`：生成 writeup
- `t`：切换题目状态
- `r`：重载题目
- `a`：生成 `challenges.toml`
- `g`：打开引导面板
- `q`：退出

### 日志面板

- `j/k` 或 `↑/↓`：滚动
- `r`：刷新
- `Esc` 或 `l`：关闭
- `q`：退出程序

### 引导面板

- `a`：生成 `challenges.toml`
- `Esc` 或 `g`：关闭
