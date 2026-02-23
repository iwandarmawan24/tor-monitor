// src/metrics/disk.rs
// Disk I/O metrics for macOS

use std::process::Command;
use std::time::Instant;

#[derive(Clone)]
pub struct DiskStats {
    pub name: String,
    pub mount_point: String,

    // Capacity
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,

    // I/O rates (from iostat)
    pub reads_per_sec: f64,
    pub writes_per_sec: f64,
    pub read_bytes_per_sec: u64,
    pub write_bytes_per_sec: u64,

    // Timestamp for rate calculation
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct DiskRates {
    pub reads_per_sec: f64,
    pub writes_per_sec: f64,
    pub read_bytes_per_sec: u64,
    pub write_bytes_per_sec: u64,
}

impl DiskStats {
    /// Calculate rates between two snapshots
    pub fn calculate_rates(&self, _previous: &DiskStats) -> DiskRates {
        // iostat already gives us rates, so just return them
        DiskRates {
            reads_per_sec: self.reads_per_sec,
            writes_per_sec: self.writes_per_sec,
            read_bytes_per_sec: self.read_bytes_per_sec,
            write_bytes_per_sec: self.write_bytes_per_sec,
        }
    }

    /// Usage percentage
    #[allow(dead_code)]
    pub fn usage_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.total_bytes as f64 * 100.0) as f32
    }
}

pub fn get_disk_stats() -> Vec<DiskStats> {
    let mut disks = get_disk_capacity();

    // Add I/O stats from iostat
    if let Some(io_stats) = get_disk_io() {
        // Apply I/O stats to first disk (usually the main one)
        if let Some(disk) = disks.first_mut() {
            if let Some(io) = io_stats.first() {
                disk.reads_per_sec = io.0;
                disk.writes_per_sec = io.1;
                disk.read_bytes_per_sec = io.2;
                disk.write_bytes_per_sec = io.3;
            }
        }
    }

    disks
}

fn get_disk_capacity() -> Vec<DiskStats> {
    let output = match Command::new("df").args(["-k"]).output() {
        Ok(o) => o,
        Err(_) => return vec![],
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let timestamp = Instant::now();
    let mut disks = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 6 {
            continue;
        }

        // Filter only real disks
        if !parts[0].starts_with("/dev/") {
            continue;
        }

        let total_kb: u64 = parts[1].parse().unwrap_or(0);
        let used_kb: u64 = parts[2].parse().unwrap_or(0);
        let available_kb: u64 = parts[3].parse().unwrap_or(0);

        disks.push(DiskStats {
            name: parts[0].to_string(),
            mount_point: parts[parts.len() - 1].to_string(),
            total_bytes: total_kb * 1024,
            used_bytes: used_kb * 1024,
            available_bytes: available_kb * 1024,
            reads_per_sec: 0.0,
            writes_per_sec: 0.0,
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            timestamp,
        });
    }

    disks
}

fn get_disk_io() -> Option<Vec<(f64, f64, u64, u64)>> {
    // iostat -d -c 2: disk stats, 2 samples (second is more accurate)
    let output = Command::new("iostat")
        .args(["-d", "-c", "2", "-w", "1"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    // Skip header lines, get last data line
    let lines: Vec<&str> = stdout.lines().collect();

    // Find the last data line (after the second header)
    let mut found_second_header = false;
    for line in lines.iter() {
        if line.contains("KB/t") {
            if found_second_header {
                continue;
            }
            found_second_header = true;
            continue;
        }

        if !found_second_header {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        // iostat output format: KB/t  tps  MB/s
        // We need tps (transfers per sec) and MB/s
        let tps: f64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let mb_s: f64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0);

        // Estimate read/write split (iostat doesn't separate by default)
        // This is a simplification - real monitoring would need more complex parsing
        let bytes_per_sec = (mb_s * 1024.0 * 1024.0) as u64;

        results.push((
            tps * 0.6,           // Approximate reads/s
            tps * 0.4,           // Approximate writes/s
            (bytes_per_sec as f64 * 0.7) as u64,  // Approximate read bytes
            (bytes_per_sec as f64 * 0.3) as u64,  // Approximate write bytes
        ));
    }

    if results.is_empty() {
        // Fallback: try simpler parsing
        for line in stdout.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let tps: f64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let mb_s: f64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let bytes_per_sec = (mb_s * 1024.0 * 1024.0) as u64;

                results.push((
                    tps * 0.6,
                    tps * 0.4,
                    (bytes_per_sec as f64 * 0.7) as u64,
                    (bytes_per_sec as f64 * 0.3) as u64,
                ));
                break;
            }
        }
    }

    Some(results)
}
