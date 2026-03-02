# Winget Submission Notes

Package id suggestion: `gandli.ctf-tui`

After each release:

```powershell
# Requires wingetcreate + GitHub auth
wingetcreate new gandli.ctf-tui \
  --version <VERSION> \
  --url https://github.com/gandli/ctf-tui-launcher/releases/download/v<VERSION>/ctf-tui-windows-x86_64.zip
```

Then submit to: https://github.com/microsoft/winget-pkgs
