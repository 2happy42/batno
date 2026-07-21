#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY="$INSTALL_DIR/batno"

if [[ ! -e "$BINARY" ]]; then
  echo "BatNo is not installed at:"
  echo "  $BINARY"
  exit 1
fi

rm "$BINARY"

echo "Removed BatNo from:"
echo "  $BINARY"

# Inform the user if the installation directory is now empty.
if [[ -d "$INSTALL_DIR" ]] && [[ -z "$(ls -A "$INSTALL_DIR")" ]]; then
  echo
  echo "Note: '$INSTALL_DIR' is now empty."
  echo "It has not been removed."
fi

echo
echo "Uninstall complete."
