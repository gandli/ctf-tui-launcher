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

> Important: package managers require a **public** GitHub repo and public release assets.
