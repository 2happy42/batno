#!/usr/bin/env bash
set -euo pipefail

REPO="2happy42/batno"
VERSION="${VERSION:-latest}"

OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" != "Linux" ]]; then
  echo "Unsupported OS: $OS"
  exit 1
fi

case "$ARCH" in
x86_64)
  TARGET="linux-x86_64"
  ;;
aarch64 | arm64)
  TARGET="linux-arm64"
  ;;
*)
  echo "Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

if [[ "$VERSION" == "latest" ]]; then
  URL="https://github.com/$REPO/releases/latest/download/batno-$TARGET.tar.gz"
else
  URL="https://github.com/$REPO/releases/download/$VERSION/batno-$TARGET.tar.gz"
fi

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading $URL..."

curl -fsSL "$URL" -o "$TMPDIR/batno.tar.gz"
tar -xzf "$TMPDIR/batno.tar.gz" -C "$TMPDIR"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

install -m 755 "$TMPDIR/batno" "$INSTALL_DIR/batno"

echo
echo "Successfully installed batno to:"
echo "  $INSTALL_DIR"

if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  echo
  echo "It looks like ~/.local/bin is not on your PATH."
  echo "Add the following line to your shell startup file (e.g. ~/.bashrc, ~/.zshrc):"
  echo
  echo 'export PATH="$HOME/.local/bin:$PATH"'
fi

echo
echo "Run:"
echo "  batno --help"
