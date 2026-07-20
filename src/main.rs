use std::process::exit;

use batno::{BatteryMonitor, monitor_battery, print_battery_info};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Notifies you when your battery level falls below a specified threshold,
/// while preventing repeated notifications until the battery has recharged.
pub struct Args {
    /// The interval, in seconds, at which the program checks the battery.
    #[arg(short, long, default_value_t = 30)]
    monitor_interval: u32,

    /// The battery percentage at or below which the program sends a notification.
    #[arg(short, long, default_value_t = 20)]
    notify_threshold: u8,

    /// The battery percentage that must be reached while charging before
    /// another low-battery notification can be sent.
    #[arg(short, long, default_value_t = 25)]
    reset_threshold: u8,

    /// Outputs information about the battery
    #[arg(short, long, default_value_t = false)]
    battery_info: bool,
}

fn main() {
    let args = Args::parse();

    if args.reset_threshold <= args.notify_threshold {
        eprintln!("reset-threshold must be greater than notify-threshold.");
        exit(1)
    }

    let monitor = BatteryMonitor::new_linux(
        args.notify_threshold,
        args.reset_threshold,
        args.monitor_interval,
    )
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });
    if args.battery_info {
        print_battery_info(&monitor);
        return;
    }
    monitor_battery(monitor);
}
