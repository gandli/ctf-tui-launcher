#!/usr/bin/env bash
set -euo pipefail

REPO="gandli/ctf-tui-launcher"
BIN_NAME="ctf-tui"
INSTALL_DIR="${HOME}/.local/bin"

log() {
  printf "[ctf-tui-install] %s\n" "$*"
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    log "Missing required command: $1"
    exit 1
  }
}

need_cmd cargo
need_cmd git

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

log "Cloning ${REPO}..."
git clone --depth 1 "https://github.com/${REPO}.git" "$TMP_DIR/repo"

log "Installing via cargo..."
(
  cd "$TMP_DIR/repo"
  cargo install --path .
)

if [ -x "${HOME}/.cargo/bin/${BIN_NAME}" ]; then
  mkdir -p "$INSTALL_DIR"
  ln -sf "${HOME}/.cargo/bin/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
  log "Linked ${BIN_NAME} -> ${INSTALL_DIR}/${BIN_NAME}"
fi

if ! command -v "$BIN_NAME" >/dev/null 2>&1; then
  log "Install completed, but '${BIN_NAME}' is not in your PATH yet."
  log "Add to your shell profile: export PATH=\"$HOME/.cargo/bin:$HOME/.local/bin:$PATH\""
else
  log "Install successful: $(${BIN_NAME} help | head -n 1)"
fi

log "Done. Run: ${BIN_NAME} tui"
