# Packaging Targets

This folder tracks package-manager publishing templates for `ctf-tui`.

## Targets

- Homebrew tap (macOS/Linux)
- Scoop bucket (Windows)
- Winget manifest (Windows)
- Chocolatey package (Windows)
- AUR package (Arch Linux)

## Release flow

1. Push tag like `v0.1.0`
2. GitHub Actions builds binaries and publishes release assets + `checksums.txt`
3. Update templates in this folder with real version/hash
4. Submit to corresponding package ecosystem
5. Verify one-liner install commands:
   - `brew install ctf-tui`
   - `scoop install ctf-tui`
   - `winget install gandli.ctf-tui`
   - `choco install ctf-tui`

## Dependency policy

`ctf-tui` requires Docker runtime for challenge lifecycle commands.
Templates include Docker dependency declarations where supported.

> Important: package managers require a **public** GitHub repo and public release assets.
