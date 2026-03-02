# Example Challenges for ctf-tui

This folder provides ready-to-use challenge directories for quick local testing.

## Included categories

- Crypto (`rsa-baby`)
- Pwn (`fmt-lab`)
- Web (`tiny-note`)
- Reverse (`baby-rev`)
- Forensics (`pcap-hunt`)
- Misc (`odd-puzzle`)
- PPC (`speed-script`)
- Blockchain (`chain-vault`)

Each challenge includes a minimal `docker/docker-compose.yml` so you can immediately test:

- challenge discovery
- `u/d` lifecycle actions
- logs panel (`l`)
- shell (`s`)
- writeup generation (`w`)

## Quick test

From repo root:

```bash
cp challenges.toml.example challenges.toml
# Optional: point workdir to examples/challenges/... if needed
ctf-tui doctor
ctf-tui tui
```

For pure auto-discovery testing, copy/symlink this folder to project `./challenges`.
