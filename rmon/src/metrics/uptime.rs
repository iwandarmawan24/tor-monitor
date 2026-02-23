// src/metrics/uptime.rs
// System uptime for macOS

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UptimeInfo {
    pub seconds: u64,
    pub formatted: String,
}

pub fn get_uptime() -> Option<UptimeInfo> {
    let output = Command::new("sysctl")
        .arg("kern.boottime")
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse: "kern.boottime: { sec = 1703123456, usec = 123456 } ..."
    let sec_start = stdout.find("sec = ")? + 6;
    let sec_end = stdout[sec_start..].find(',')?;
    let boot_time: u64 = stdout[sec_start..sec_start + sec_end].trim().parse().ok()?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();

    let uptime_secs = now.saturating_sub(boot_time);

    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    let formatted = if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else {
        format!("{}h {}m", hours, minutes)
    };

    Some(UptimeInfo {
        seconds: uptime_secs,
        formatted,
    })
}
