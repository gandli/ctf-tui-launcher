# Product Requirements Document (PRD)

## 1. Product Name

**ctf-tui-launcher**

## 2. Background

In CTF learning and challenge reproduction workflows, users repeatedly run Docker commands, manually maintain challenge folders, inject dynamic flags by hand, and write post-solve notes in an ad-hoc way. This fragmented process reduces efficiency and reproducibility.

This project builds a Rust-based terminal UI (TUI) that serves as a unified entry point for local CTF practice, following deployment patterns from `CTF-Archives/ctf-docker-template`.

## 3. Target Users

- Beginner to intermediate CTF players who need stable local reproduction
- Individual developers who want fast challenge environment management
- Training organizers who need standardized local practice workflows

## 4. Product Goals

1. Reduce challenge startup complexity to a few keystrokes (target: within 3 key actions)
2. Lower environment failure rate (image, port, env var mistakes)
3. Improve writeup output rate via automatic templates

## 5. Non-Goals (v0.1)

- No online competition platform features (scoreboard, ranking, multi-user)
- No remote cluster orchestration
- Not a replacement for CTFd/GZCTF platforms

## 6. User Stories

- As a practice user, I want to start a selected challenge environment with one key action.
- As a practice user, I want quick access to container logs and runtime state to debug failures.
- As a practice user, I want automatic injection of `FLAG` / `GZCTF_FLAG` / `DASCTF` without manual command assembly.
- As a practice user, I want one-key generation of a writeup draft when I finish a challenge.

## 7. Functional Requirements

### 7.1 Challenge Management

- Scan local challenge directories
- Display basic metadata: name, category, difficulty, status
- Support status transitions: `todo / doing / done`

### 7.2 Environment Lifecycle

- Start: `docker compose up -d`
- Stop: `docker compose down`
- Logs: `docker compose logs -f`
- Shell: open command shell inside target container

### 7.3 Dynamic Flag Injection

- Support the following variable mappings:
  - `FLAG`
  - `GZCTF_FLAG`
  - `DASCTF`
- Allow users to pick a preset or manually input values before startup

### 7.4 Writeup Support

- Generate `writeup.md` scaffold
- Include fixed sections: challenge info, environment, solution path, reproduction steps, pitfalls

### 7.5 Configuration

- Manage via `toml`:
  - challenge root directory
  - default flag strategy
  - default log level

## 8. UX / Interaction Requirements

### 8.1 Layout

- Left panel: challenge list
- Right panel: challenge details + runtime status
- Bottom area: key hints / command feedback

### 8.2 Keymap (Draft)

- `j/k`: move selection
- `Enter`: open challenge details
- `u`: start environment
- `d`: stop environment
- `l`: view logs
- `s`: open shell
- `w`: generate writeup
- `q`: quit

## 9. Quality Requirements

- Startup failures must return readable errors (e.g., missing compose file, Docker daemon not running)
- Command execution status must be visible (`running / success / fail`)
- Critical operations should be recoverable (e.g., restart after accidental stop)

## 10. Milestones

### M1: Bootstrap (Current)

- Initialize repository and baseline docs
- Finalize architecture and interaction model

### M2: TUI Skeleton

- Core split-pane UI + keymap
- List/detail rendering

### M3: Docker Integration

- Full lifecycle actions (`up/down/logs/shell`) usable end to end

### M4: Writeup + Polish

- Writeup template generation
- Better error handling and status UX

## 11. Success Metrics

- Reduce average time from “manual setup” to “running challenge environment” by 50%+
- Increase writeup output rate after challenge completion
- Decrease environment startup failure rate over iterations
