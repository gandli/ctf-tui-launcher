# Product Requirements Document (PRD)

## 1. Product Name

**ctf-tui-launcher**

## 2. Background

CTF 学习和复现场景中，用户通常需要频繁执行 Docker 命令、维护题目目录结构、手动注入动态 Flag，并在做题后自行整理 writeup。流程割裂导致效率低、复现困难。

本项目旨在使用 Rust 开发一个终端内 TUI 工具，作为 CTF 练题的统一入口，并参考 `CTF-Archives/ctf-docker-template` 的部署范式。

## 3. Target Users

- CTF 初中级选手（需要稳定复现环境）
- 个人开发者（希望快速管理本地题目容器）
- 教学/训练组织者（需要标准化训练流程）

## 4. Product Goals

1. 将“启动题目环境”的操作复杂度降到最低（目标：3 次按键内）
2. 降低环境错误率（镜像、端口、变量）
3. 提升复盘产出率（自动生成 writeup 模板）

## 5. Non-Goals (v0.1)

- 不做线上比赛平台功能（计分、排行榜、多用户）
- 不做远程集群编排
- 不替代 CTFd/GZCTF 平台本身

## 6. User Stories

- 作为一名练题用户，我希望在列表里选中题目后，一键启动 Docker 环境。
- 作为一名练题用户，我希望查看容器日志和运行状态，快速定位失败原因。
- 作为一名练题用户，我希望自动注入 `FLAG`/`GZCTF_FLAG`/`DASCTF` 变量，不再手动拼命令。
- 作为一名练题用户，我希望做完题时能一键生成 writeup 草稿。

## 7. Functional Requirements

### 7.1 Challenge Management
- 扫描本地 challenge 目录
- 展示基础信息：名称、分类、难度、状态
- 支持状态变更：todo / doing / done

### 7.2 Environment Lifecycle
- 启动：`docker compose up -d`
- 停止：`docker compose down`
- 日志：`docker compose logs -f`
- Shell：进入目标容器执行命令

### 7.3 Dynamic Flag Injection
- 支持以下变量映射：
  - `FLAG`
  - `GZCTF_FLAG`
  - `DASCTF`
- 启动前允许选择注入模板或手动输入

### 7.4 Writeup Support
- 生成 `writeup.md` 模板
- 包含固定章节：题目信息、环境、解题思路、复现步骤、坑点总结

### 7.5 Config
- 通过 `toml` 管理：
  - challenge 根目录
  - 默认 flag 策略
  - 默认日志级别

## 8. UX / Interaction Requirements

### 8.1 Layout
- 左侧：题目列表
- 右侧：题目详情 + 环境状态
- 底部：快捷键提示 / 命令反馈

### 8.2 Keymap (Draft)
- `j/k`：上下移动
- `Enter`：查看题目详情
- `u`：启动环境
- `d`：停止环境
- `l`：查看日志
- `s`：进入 shell
- `w`：生成 writeup
- `q`：退出

## 9. Quality Requirements

- 启动失败必须给出可读错误（如 compose 文件缺失、docker 未启动）
- 命令执行状态可见（running/success/fail）
- 关键操作需可恢复（例如误停止后可一键重启）

## 10. Milestones

### M1: Bootstrap (当前)
- 初始化仓库与基础文档
- 明确架构与交互

### M2: TUI Skeleton
- 基础分栏 UI + keymap
- 列表与详情渲染

### M3: Docker Integration
- up/down/logs/shell 全链路可用

### M4: Writeup + Polish
- writeup 模板生成
- 错误处理、状态提示优化

## 11. Success Metrics

- 新题环境从“手动命令”到“成功启动”平均时间下降 50%+
- 用户完成题后 writeup 产出率提升
- 环境启动失败率逐步降低
