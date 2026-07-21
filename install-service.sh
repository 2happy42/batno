#!/usr/bin/env bash
set -euo pipefail

SERVICE_DIR="$HOME/.config/systemd/user"
SERVICE_FILE="$SERVICE_DIR/batno.service"

BATNO_PATH="$(command -v batno || true)"

if [[ -z "$BATNO_PATH" ]]; then
  echo "Error: batno was not found in your PATH."
  echo "Please install BatNo first."
  exit 1
fi

if ! VERSION_OUTPUT="$("$BATNO_PATH" --version 2>&1)"; then
  echo "Error: Could not execute batno."
  echo "Found binary:"
  echo "  $BATNO_PATH"
  exit 1
fi

echo
echo "Found BatNo:"
echo "  Path:    $BATNO_PATH"
echo "  Version: $VERSION_OUTPUT"
echo

read -rp "Is this the correct BatNo installation? [y/N] " CONFIRM

case "$CONFIRM" in
y | Y | yes | YES)
  ;;
*)
  echo "Installation cancelled."
  exit 0
  ;;
esac

echo

read -rp "Notification threshold [%] (default 20): " NOTIFY_THRESHOLD
NOTIFY_THRESHOLD="${NOTIFY_THRESHOLD:-20}"

read -rp "Reset threshold [%] (default 25): " RESET_THRESHOLD
RESET_THRESHOLD="${RESET_THRESHOLD:-25}"

read -rp "Monitoring interval in seconds (default 30): " MONITOR_INTERVAL
MONITOR_INTERVAL="${MONITOR_INTERVAL:-30}"

if ! [[ "$NOTIFY_THRESHOLD" =~ ^[0-9]+$ ]] || (( NOTIFY_THRESHOLD > 100 )); then
  echo "Error: Notification threshold must be an integer between 0 and 100."
  exit 1
fi

if ! [[ "$RESET_THRESHOLD" =~ ^[0-9]+$ ]] || (( RESET_THRESHOLD > 100 )); then
  echo "Error: Reset threshold must be an integer between 0 and 100."
  exit 1
fi

if (( RESET_THRESHOLD <= NOTIFY_THRESHOLD )); then
  echo "Error: Reset threshold must be greater than notification threshold."
  exit 1
fi

if ! [[ "$MONITOR_INTERVAL" =~ ^[0-9]+$ ]] || (( MONITOR_INTERVAL < 1 )); then
  echo "Error: Monitoring interval must be an integer greater than 0."
  exit 1
fi
mkdir -p "$SERVICE_DIR"

cat >"$SERVICE_FILE" <<EOF
[Unit]
Description=BatNo battery monitor
After=graphical-session.target

[Service]
Type=simple
ExecStart=$BATNO_PATH --notify-threshold $NOTIFY_THRESHOLD --reset-threshold $RESET_THRESHOLD --monitor-interval $MONITOR_INTERVAL
Restart=on-failure
RestartSec=5

[Install]
WantedBy=graphical-session.target
EOF

echo "Created service:"
echo "  $SERVICE_FILE"

systemctl --user daemon-reload
systemctl --user enable batno.service
systemctl --user restart batno.service

echo
echo "BatNo service installed successfully."
echo
echo "Current status:"
systemctl --user status batno.service --no-pager
