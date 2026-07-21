# BatNo

[![Rust Tests](https://github.com/2happy42/batno/actions/workflows/rust.yml/badge.svg)](https://github.com/2happy42/batno/actions/workflows/rust.yml)
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
| `--monitor-interval` | `30` | Time between battery checks in seconds |
| `--notify-threshold` | `20` | Battery percentage that triggers a notification |
| `--reset-threshold` | `25` | Battery percentage required before another notification can be sent |
| `--battery-info` | `false` | Prints battery information instead of monitoring |

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
See the LICENSE file for details.
