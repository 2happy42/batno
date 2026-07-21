#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="batno.service"
SERVICE_FILE="$HOME/.config/systemd/user/$SERVICE_NAME"

echo "Removing BatNo systemd user service..."

if systemctl --user list-unit-files | grep -q "$SERVICE_NAME"; then
  echo "Stopping service..."
  systemctl --user stop "$SERVICE_NAME" || true

  echo "Disabling service..."
  systemctl --user disable "$SERVICE_NAME" || true
else
  echo "Service is not currently enabled."
fi

if [[ -f "$SERVICE_FILE" ]]; then
  echo "Removing service file..."
  rm "$SERVICE_FILE"
else
  echo "Service file does not exist."
fi

systemctl --user daemon-reload

echo
echo "BatNo service removed successfully."
