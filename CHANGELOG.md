# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project uses SemVer tags.

## [0.1.2] - 2026-03-02

### Added
- GitHub Release workflow for multi-platform binary artifacts.
- Packaging templates for Homebrew, Scoop, Winget, Chocolatey, and AUR.
- Resolved package manifests for `v0.1.2` with real checksums.

### Changed
- Release matrix adjusted to remove unstable Linux aarch64 target.
- README updated with package-manager install matrix and dependency notes.

### Fixed
- Borrow checker issue in log refresh path that caused CI build failures.

## [0.1.1] - 2026-03-02

### Fixed
- Internal log refresh ownership/borrow handling improvements.

## [0.1.0] - 2026-03-02

### Added
- Initial `ctf-tui` / `ctf-tui-launcher` CLI with subcommands:
  - `tui`, `init`, `doctor`, `help`
- Split-pane TUI skeleton with challenge list and detail panel.
- Docker actions (`up/down`) and in-app logs panel with refresh/scroll.
- Shell action (`s`) and writeup scaffold generation (`w`).
- Status cycling and persistence to `challenges.toml`.
- Auto-discovery fallback for `./challenges/*/docker`.
- Guided setup panel and one-key config bootstrap.
- Example challenge pack for major CTF categories.
- English/Chinese README docs.

[0.1.2]: https://github.com/gandli/ctf-tui-launcher/releases/tag/v0.1.2
[0.1.1]: https://github.com/gandli/ctf-tui-launcher/releases/tag/v0.1.1
[0.1.0]: https://github.com/gandli/ctf-tui-launcher/releases/tag/v0.1.0
