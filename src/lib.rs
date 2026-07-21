//! # BatNo
//!
//! A battery monitoring library that periodically checks battery status and
//! sends notifications when the battery reaches a configured threshold.
//!
//! ## Extending BatNo
//!
//! Platform-specific functionality is isolated behind two traits:
//!
//! - [`BatteryProvider`] handles reading battery information.
//! - [`SystemNotifier`] handles sending notifications.
//!
//! To add support for a new operating system:
//!
//! 1. Implement [`BatteryProvider`] for the new platform.
//! 2. Implement [`SystemNotifier`] for the platform's notification system.
//! 3. Create a constructor that combines those implementations into a
//!    [`BatteryMonitor`].
//!
//! The monitoring logic does not need to change as long as the new types
//! implement the required traits.

use std::{fmt, fs};
use std::{io, path::PathBuf};
use std::{
    process::Command,
    thread::sleep,
    time::{Duration, Instant},
};
/// Location where Linux exposes power supply devices.
///
/// Every battery, AC adapter, and other power source is represented as a
/// directory inside this path.
const LINUX_POWER_SUPPLY_PATH: &str = "/sys/class/power_supply/";

/// Represents the current state of a battery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    /// Battery is currently charging.
    Charging,
    /// Battery is currently supplying power to the system.
    Discharging,
    /// Battery is neither charging nor discharging.
    ///
    /// This can represent states such as a fully charged battery or a temporarily
    /// inactive charging state.
    Idle,
    /// Battery state could not be determined.
    Unknown,
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatteryStatus::Charging => write!(f, "Charging"),
            BatteryStatus::Discharging => write!(f, "Discharging"),
            BatteryStatus::Idle => write!(f, "Idle"),
            BatteryStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Provides access to battery information.
///
/// This trait abstracts the source of battery data. The default implementation for Linux
/// reads information from Linux's `/sys/class/power_supply` filesystem, but
/// alternative implementations can provide data from another source.
pub trait BatteryProvider {
    /// Returns the current battery percentage (0-100).
    fn get_capacity(&self) -> Result<u8, io::Error>;
    /// Returns the current charging state of the battery.
    fn get_battery_status(&self) -> Result<BatteryStatus, io::Error>;
    /// Returns the battery technology (for example Lithium-Polymer).
    fn get_technology(&self) -> Result<String, io::Error>;
    /// Returns the battery manufacturer name.
    fn get_manufacturer(&self) -> Result<String, io::Error>;
    /// Returns the battery model name.
    fn get_model_name(&self) -> Result<String, io::Error>;
    /// Returns the battery serial number.
    fn get_serial_number(&self) -> Result<String, io::Error>;
}

/// Provides a way to send system notifications.
///
/// This abstraction allows the battery monitor to work with different
/// notification systems without depending on a specific implementation.
pub trait SystemNotifier {
    /// Sends a notification to the user.
    ///
    /// # Arguments
    ///
    /// * `title` - Notification title.
    /// * `message` - Notification body text.
    /// * `urgency` - Notification importance level.
    ///
    /// `urgency` is an implementation-defined hint describing the importance
    /// of the notification. Individual notifier implementations may interpret
    /// this value differently or ignore it.
    fn notify(&self, title: &str, message: &str, urgency: &str) -> Result<(), io::Error>;
}

/// Searches the Linux power supply directory for a battery device.
///
/// Returns the path of the first device whose `type` file contains `Battery`.
fn find_linux_battery_path() -> Option<PathBuf> {
    fs::read_dir(LINUX_POWER_SUPPLY_PATH)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            fs::read_to_string(path.join("type"))
                .map(|t| t.trim() == "Battery")
                .unwrap_or(false)
        })
}

/// Battery provider implementation for Linux systems.
///
/// Reads battery information from the Linux sysfs power supply interface.
pub struct LinuxBattery {
    path: PathBuf,
}

impl LinuxBattery {
    /// Creates a Linux battery provider.
    ///
    /// Returns an error if no battery device can be found.
    fn new() -> Result<Self, io::Error> {
        match find_linux_battery_path() {
            Some(path) => Ok(LinuxBattery { path }),
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No Linux battery found",
            )),
        }
    }
}

impl BatteryProvider for LinuxBattery {
    fn get_capacity(&self) -> Result<u8, io::Error> {
        fs::read_to_string(self.path.join("capacity"))?
            .trim()
            .parse::<u8>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn get_battery_status(&self) -> Result<BatteryStatus, io::Error> {
        let content = fs::read_to_string(self.path.join("status"))?;
        match content.trim() {
            "Discharging" => Ok(BatteryStatus::Discharging),
            "Charging" => Ok(BatteryStatus::Charging),
            "Full" => Ok(BatteryStatus::Idle),
            "Not charging" => Ok(BatteryStatus::Idle),
            _ => Ok(BatteryStatus::Unknown),
        }
    }

    fn get_technology(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.path.join("technology"))?
            .trim()
            .to_string())
    }

    fn get_manufacturer(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.path.join("manufacturer"))?
            .trim()
            .to_string())
    }

    fn get_model_name(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.path.join("model_name"))?
            .trim()
            .to_string())
    }

    fn get_serial_number(&self) -> Result<String, io::Error> {
        Ok(fs::read_to_string(self.path.join("serial_number"))?
            .trim()
            .to_string())
    }
}

/// Sends desktop notifications using the Linux `notify-send` command.
///
/// Supported urgency values:
///
/// - `"low"`
/// - `"normal"`
/// - `"critical"`
pub struct LinuxNotifier;

impl SystemNotifier for LinuxNotifier {
    /// Runs: notify-send --urgency low|normal|critical "<title>" "<message>"
    fn notify(&self, title: &str, message: &str, urgency: &str) -> Result<(), io::Error> {
        Command::new("notify-send")
            .arg("--urgency")
            .arg(urgency)
            .arg(title)
            .arg(message)
            .status()
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(io::Error::other("notify-send returned a failure status"))
                }
            })
            .map_err(|e| io::Error::other(format!("Notify-send failed: {e}")))
    }
}

/// Coordinates battery monitoring and notification handling.
///
/// The monitor periodically checks battery capacity and sends a notification
/// when the configured threshold is reached. After a notification is sent,
/// the battery must recharge above the reset threshold before another
/// notification can occur.
pub struct BatteryMonitor<S: SystemNotifier, T: BatteryProvider> {
    provider: T,
    notifier: S,
    notify_threshold: u8,
    reset_threshold: u8,
    monitor_interval: u32,
    notification_sent: bool,
}

impl BatteryMonitor<LinuxNotifier, LinuxBattery> {
    /// Creates a battery monitor using Linux implementations.
    ///
    /// Uses sysfs for battery information and `notify-send` for desktop
    /// notifications.
    ///
    /// # Errors
    ///
    /// Returns an error if no battery device can be found.
    pub fn new_linux(
        notify_threshold: u8,
        reset_threshold: u8,
        monitor_interval: u32,
    ) -> Result<Self, io::Error> {
        Ok(BatteryMonitor {
            provider: LinuxBattery::new()?,
            notifier: LinuxNotifier,
            notify_threshold,
            reset_threshold,
            monitor_interval,
            notification_sent: false,
        })
    }
}

/// Converts a `Result` into a displayable string.
///
/// Successful values are converted using `Display`. Failed values are replaced
/// with a human-readable error message for table output.
fn format_result<T: std::fmt::Display>(result: Result<T, io::Error>, field: &str) -> String {
    result
        .map(|v| v.to_string())
        .unwrap_or_else(|_| format!("Failed to fetch {field}"))
}

/// Prints battery information as a formatted table.
///
/// If a value cannot be read, the table displays an error message in the
/// corresponding row instead of failing completely.
pub fn print_battery_info<S: SystemNotifier, T: BatteryProvider>(monitor: &BatteryMonitor<S, T>) {
    let capacity = monitor.provider.get_capacity();
    let status = monitor.provider.get_battery_status();
    let technology = monitor.provider.get_technology();
    let manufacturer = monitor.provider.get_manufacturer();
    let model_name = monitor.provider.get_model_name();
    let serial_number = monitor.provider.get_serial_number();

    let rows = vec![
        (
            "Capacity",
            format_result(capacity.map(|v| format!("{v}%")), "capacity"),
        ),
        ("Status", format_result(status, "status")),
        ("Technology", format_result(technology, "technology")),
        ("Manufacturer", format_result(manufacturer, "manufacturer")),
        ("Model", format_result(model_name, "model")),
        (
            "Serial Number",
            format_result(serial_number, "serial number"),
        ),
    ];

    let left_width = rows.iter().map(|(k, _)| k.len()).max().unwrap_or(0);

    let right_width = rows.iter().map(|(_, v)| v.len()).max().unwrap_or(0);

    println!(
        "┌{:─<left$}┬{:─<right$}┐",
        "",
        "",
        left = left_width + 2,
        right = right_width + 2
    );

    println!(
        "│ {:^left$} │ {:^right$} │",
        "Property",
        "Value",
        left = left_width,
        right = right_width
    );

    println!(
        "├{:─<left$}┼{:─<right$}┤",
        "",
        "",
        left = left_width + 2,
        right = right_width + 2
    );

    for (key, value) in rows {
        println!(
            "│ {:<left$} │ {:<right$} │",
            key,
            value,
            left = left_width,
            right = right_width
        );
    }

    println!(
        "└{:─<left$}┴{:─<right$}┘",
        "",
        "",
        left = left_width + 2,
        right = right_width + 2
    );
}

/// Continuously monitors battery level and sends notifications when required.
///
/// The function runs indefinitely. Temporary failures while reading battery
/// information or sending notifications are logged and retried during the next
/// monitoring cycle.
pub fn monitor_battery<S: SystemNotifier, T: BatteryProvider>(mut monitor: BatteryMonitor<S, T>) {
    loop {
        let start = Instant::now();
        match monitor.provider.get_capacity() {
            Ok(capacity) => {
                if capacity <= monitor.notify_threshold && !monitor.notification_sent {
                    let mut result = Err(io::Error::other("not attempted"));
                    for _ in 0..5 {
                        result = monitor.notifier.notify(
                            "BatNo",
                            &format!("Your battery capacity is equal to or below your set threshold of {}%. Please plug your computer into a power source to charge it", monitor.notify_threshold ),
                            "critical",
                        );

                        if result.is_ok() {
                            monitor.notification_sent = true;
                            break;
                        }
                    }

                    if let Err(e) = result {
                        eprintln!("Failed to send notification: {e}");
                    }
                } else if capacity >= monitor.reset_threshold && monitor.notification_sent {
                    monitor.notification_sent = false
                }
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }

        let sleep_time =
            Duration::from_secs(monitor.monitor_interval.into()).saturating_sub(start.elapsed());
        sleep(sleep_time);
    }
}
