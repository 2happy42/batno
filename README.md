# BatNo

[![CI](https://github.com/2happy42/batno/actions/workflows/ci.yml/badge.svg)](https://github.com/2happy42/batno/actions/workflows/ci.yml)
[![Release](https://github.com/2happy42/batno/actions/workflows/release.yml/badge.svg)](https://github.com/2happy42/batno/actions/workflows/release.yml)
[![Security Audit](https://github.com/2happy42/batno/actions/workflows/audit.yml/badge.svg)](https://github.com/2happy42/batno/actions/workflows/audit.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

**BatNo** is a lightweight Linux utility that monitors your battery level and sends a desktop notification when it falls below a configurable threshold.

Unlike many battery monitoring tools, BatNo is designed to avoid notification spam. Once you've been notified, it won't notify you again until the battery has recharged above a configurable reset threshold.

---

## Why?

Many laptops have an annoying behavior when the charging cable has a loose connection.

Imagine this scenario:

1. Your battery reaches **20%**.
2. BatNo sends a notification reminding you to plug in your charger.
3. You connect the charger.
4. Because of a loose cable, charging briefly disconnects.
5. The battery is still at **20%**.
6. Another notification appears.
7. This repeats every few seconds.

The constant notification spam quickly becomes frustrating.

BatNo solves this by remembering that it has already warned you. After sending a notification, it waits until the battery has charged above a configurable **reset threshold** before another low-battery notification can be sent.

For example:

- Notification threshold: **20%**
- Reset threshold: **25%**

```
Battery: 20%  -> Notification sent
Battery: 19%  -> No notification
Battery: 18%  -> No notification
Battery: 20%  -> No notification
Battery: 24%  -> No notification
Battery: 25%  -> Notification system resets
Battery: 20%  -> Notification sent again
```

---

## Features

- Lightweight
- Reads battery information directly from Linux `sysfs`
- Uses `notify-send` for desktop notifications
- Prevents repeated notifications
- Configurable monitoring interval
- Configurable notification threshold
- Configurable reset threshold
- Displays battery information in a formatted table

---

## Requirements

- Linux
- `notify-send` (usually provided by `libnotify`)
- A desktop environment that supports desktop notifications

---

## Installation

### One-line installer (recommended)

Install the latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/install.sh | bash
```

By default, BatNo is installed to `~/.local/bin`.

If `~/.local/bin` is not on your `PATH`, the installer will let you know and print the command you should add to your shell startup file (e.g. `~/.bashrc` or `~/.zshrc`).

### Custom installation

The installer can be customized using environment variables.

Install a specific release:

```bash
VERSION=v1.2.3 curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/install.sh | bash
```

Install to a different directory:

```bash
INSTALL_DIR="$HOME/bin" curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/install.sh | bash
```

Install a specific version to a custom directory:

```bash
VERSION=v1.2.3 INSTALL_DIR="$HOME/bin" \
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/install.sh | bash
```


### From crates.io

```bash
cargo install batno
```

### From source

Clone the repository:

```bash
git clone https://github.com/2happy42/batno.git
cd batno
```

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run --release
```

---

## Uninstall

If you installed BatNo using the installer script:

```bash
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/uninstall.sh | bash
```

If you installed BatNo to a custom directory, specify the same `INSTALL_DIR` that was used during installation:

```bash
INSTALL_DIR="$HOME/bin" \
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/uninstall.sh | bash
```

> **Note:** The uninstall script removes only the `batno` binary. It does not modify your shell configuration or remove the installation directory.

If you installed BatNo with Cargo:

```bash
cargo uninstall batno
```


## Running BatNo as a background service

BatNo can run continuously in the background using a user-level `systemd` service.

This requires `systemd` user services (i.e. `systemctl --user` must work in your session).

The service starts automatically when you enter your graphical desktop session and keeps monitoring your battery without requiring a terminal window to stay open.

### Install the background service

Run:

```bash
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/install-service.sh | bash
```

The installer will:

1. Find the installed `batno` executable in your `PATH`.
2. Verify that the executable is a valid BatNo installation by running:

```bash
batno --version
```

3. Display the detected installation path and version.
4. Ask you to confirm that this is the correct BatNo installation.
5. Ask for the monitoring configuration:
   - Notification threshold
   - Reset threshold
   - Monitoring interval
6. Create a user-level systemd service.
7. Enable and start the service.

Example:

```text
Found BatNo:
  Path:    /home/user/.local/bin/batno
  Version: batno 1.0.0

Is this the correct BatNo installation? [y/N] y

Notification threshold [%] (default 20): 20
Reset threshold [%] (default 25): 25
Monitoring interval in seconds (default 30): 30
```

### How the service works

The service is installed as a **user systemd service**:

```text
~/.config/systemd/user/batno.service
```

It runs only for the current user and does not require administrator privileges.

The service:

- Starts automatically when the user enters their graphical desktop session.
- Runs BatNo in the background.
- Restarts automatically if BatNo exits unexpectedly.
- Uses the thresholds and interval configured during installation.

The generated service executes BatNo similar to:

```bash
batno \
    --notify-threshold 20 \
    --reset-threshold 25 \
    --monitor-interval 30
```

### Managing the service

Check the current status:

```bash
systemctl --user status batno
```

Stop the service:

```bash
systemctl --user stop batno
```

Start the service:

```bash
systemctl --user start batno
```

Restart the service after changing settings:

```bash
systemctl --user restart batno
```

View service logs:

```bash
journalctl --user -u batno
```

### Remove the background service

To stop BatNo from running automatically:

```bash
curl -fsSL https://raw.githubusercontent.com/2happy42/batno/main/uninstall-service.sh | bash
```

This will:

- Stop the running BatNo service.
- Disable automatic startup.
- Remove the systemd service file.
- Reload the user systemd configuration.

The BatNo executable itself is **not removed**. You can still run it manually with:

```bash
batno
```

### Security and privacy considerations

The service runs with the same permissions as your user account.

This means:

- BatNo can access anything that your user account can access.
- The service does not run with administrator privileges.
- Installing the service does not require `sudo`.
- Removing the service does not remove the BatNo binary.

The installer creates a systemd service file containing the exact path to the BatNo executable and the configured command-line arguments. Review the generated service file if you want to verify the configuration:

```bash
cat ~/.config/systemd/user/batno.service
```

Only install service scripts from sources you trust. A script executed through:

```bash
curl ... | bash
```

runs with your user permissions and should always be reviewed before use if downloaded from an unknown source.


## Usage

Run with default settings:

```bash
batno
```

Show available options:

```bash
batno --help
```

Example:

```bash
batno \
    --notify-threshold 20 \
    --reset-threshold 25 \
    --monitor-interval 30
```

Display battery information:

```bash
batno --battery-info
```

---

## Command-line options

| Option | Default | Description |
|---------|---------|-------------|
| `-m`, `--monitor-interval <SECONDS>` | `30` | The interval, in seconds, at which the program checks the battery |
| `-n`, `--notify-threshold <PERCENTAGE>` | `20` | The battery percentage at or below which the program sends a notification |
| `-r`, `--reset-threshold <PERCENTAGE>` | `25` | The battery percentage that must be reached while charging before another low-battery notification can be sent |
| `-b`, `--battery-info` | `false` | Outputs information about the battery |
| `-h`, `--help` | | Prints help information |
| `-V`, `--version` | | Prints the installed BatNo version |

---

## Extending BatNo

The project separates platform-specific code from the monitoring logic.

Support for new operating systems only requires implementing two traits:

- `BatteryProvider`
- `SystemNotifier`

Once these are implemented, a platform-specific constructor can create a `BatteryMonitor`, and the existing monitoring logic works without modification.

---

## Contributing

Contributions are welcome.

Before contributing, please read the [Code of Conduct](CODE_OF_CONDUCT.md).

If you'd like to contribute:

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Run `cargo fmt`.
5. Run `cargo clippy`.
6. Open a pull request.

Possible areas for improvement include:

- Windows support
- macOS support
- Additional battery information
- Better notification customization
- Configuration file support
- Automated tests
- CI workflows
- Packaging for Linux distributions

If you find a bug or have an idea for an improvement, feel free to open an issue or submit a pull request.

---

## Roadmap

Possible future improvements include:

- Windows support
- macOS support
- More battery information
- Multiple battery support
- Configuration file
- Custom notification icons
- Logging
- Unit and integration tests
---

## License

Copyright (c) 2026 2happy42

This project is licensed under the GNU Affero General Public License v3.0 or later.
See the [LICENSE](LICENSE) file for details.
