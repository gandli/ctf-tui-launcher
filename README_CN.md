# ctf-tui-launcher

用于 CTF 复现练题的 Rust TUI 启动器，灵感来自 [`CTF-Archives/ctf-docker-template`](https://github.com/CTF-Archives/ctf-docker-template)。

## 快速开始（安装 + 使用）

## 依赖前置

使用 `ctf-tui` 前请先确保已安装：

- **Docker**（题目环境运行必需）
- Git
- Rust / Cargo

可先检查：

```bash
docker --version
docker compose version
git --version
cargo --version
```

如果 Docker 未安装：
- macOS / Windows：安装 Docker Desktop
- Linux：安装 Docker Engine + Docker Compose 插件

### 按系统安装

#### macOS

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

#### Linux

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/gandli/ctf-tui-launcher/main/install.sh)"
```

#### Windows（PowerShell）

```powershell
git clone https://github.com/gandli/ctf-tui-launcher.git
cd ctf-tui-launcher

# 如果没有 cargo，先安装 Rust
winget install Rustlang.Rustup
# 安装后重启 PowerShell，再执行：
cargo install --path .
```

### 包管理器一键安装（发布后可用）

发布完成后，可直接使用：

#### Homebrew（macOS / Linux）

```bash
brew tap gandli/ctf-tui
brew install ctf-tui
```

#### Scoop（Windows）

```powershell
scoop bucket add gandli https://github.com/gandli/scoop-bucket
scoop install ctf-tui
```

#### Winget（Windows）

```powershell
winget install gandli.ctf-tui
```

#### Chocolatey（Windows）

```powershell
choco install ctf-tui -y
```

以上包模板中已声明 Docker 依赖（作为安装/运行前置）。

### 源码安装（通用）

```bash
# 1) 克隆仓库
git clone https://github.com/gandli/ctf-tui-launcher.git
cd ctf-tui-launcher

# 2) 若本机没有 cargo，先安装 Rust 工具链
# macOS / Linux:
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

# Windows (PowerShell):
# winget install Rustlang.Rustup

# 3) 安装命令行工具
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

### 立即体验（内置示例）

```bash
cp examples/challenges.toml ./challenges.toml
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

### 引导式配置（推荐）

在 TUI 内按引导完成：

1. 运行 `ctf-tui tui`
2. 按 `g` 打开引导面板
3. 按 `a` 自动生成 `challenges.toml`
4. 编辑生成的 `challenges.toml`
5. 回到 TUI 按 `r` 重载

也可用命令行引导流程：

```bash
ctf-tui init
ctf-tui doctor
ctf-tui tui
```

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

## 二进制发布 + 包管理分发

项目已加入 Release 工作流：

- `.github/workflows/release.yml`
- 通过 Git tag `v*` 触发（例如 `v0.1.0`）
- 自动发布多平台二进制与 `checksums.txt`

`packaging/` 目录已提供模板：

- Homebrew
- Scoop
- Winget
- Chocolatey
- AUR

## 示例题包

- `examples/challenges/` 内置 8 类题型示例目录
- 每个示例都包含 `docker/docker-compose.yml`，可直接测试
- `examples/challenges.toml` 可直接复制使用

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
