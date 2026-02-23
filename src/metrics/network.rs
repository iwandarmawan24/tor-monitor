// src/metrics/network.rs
// Network I/O metrics for macOS

use std::process::Command;
use std::time::Instant;

#[derive(Clone)]
pub struct NetworkStats {
    pub interface: String,

    // Packet counters
    pub packets_in: u64,
    pub packets_out: u64,

    // Byte counters
    pub bytes_in: u64,
    pub bytes_out: u64,

    // Errors
    pub errors_in: u64,
    pub errors_out: u64,

    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct NetworkRates {
    pub packets_in_per_sec: f64,
    pub packets_out_per_sec: f64,
    pub bytes_in_per_sec: u64,
    pub bytes_out_per_sec: u64,
}

impl NetworkStats {
    pub fn calculate_rates(&self, previous: &NetworkStats) -> NetworkRates {
        let elapsed = self.timestamp.duration_since(previous.timestamp).as_secs_f64();

        if elapsed <= 0.0 {
            return NetworkRates {
                packets_in_per_sec: 0.0,
                packets_out_per_sec: 0.0,
                bytes_in_per_sec: 0,
                bytes_out_per_sec: 0,
            };
        }

        let delta_pkt_in = self.packets_in.saturating_sub(previous.packets_in);
        let delta_pkt_out = self.packets_out.saturating_sub(previous.packets_out);
        let delta_bytes_in = self.bytes_in.saturating_sub(previous.bytes_in);
        let delta_bytes_out = self.bytes_out.saturating_sub(previous.bytes_out);

        NetworkRates {
            packets_in_per_sec: delta_pkt_in as f64 / elapsed,
            packets_out_per_sec: delta_pkt_out as f64 / elapsed,
            bytes_in_per_sec: (delta_bytes_in as f64 / elapsed) as u64,
            bytes_out_per_sec: (delta_bytes_out as f64 / elapsed) as u64,
        }
    }
}

pub fn get_network_stats() -> Vec<NetworkStats> {
    let output = match Command::new("netstat").args(["-ibn"]).output() {
        Ok(o) => o,
        Err(_) => return vec![],
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let timestamp = Instant::now();
    let mut interfaces: Vec<NetworkStats> = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 10 {
            continue;
        }

        let name = parts[0].to_string();

        // Skip loopback and duplicates
        if name == "lo0" || seen.contains(&name) {
            continue;
        }

        // Skip non-primary entries (those without Link# in Network field)
        if !parts.get(2).map(|s| s.contains("Link")).unwrap_or(false) {
            continue;
        }

        // netstat -ibn columns:
        // Name Mtu Network Address Ipkts Ierrs Ibytes Opkts Oerrs Obytes
        // 0    1   2       3       4     5     6      7     8     9

        let packets_in: u64 = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
        let errors_in: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
        let bytes_in: u64 = parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
        let packets_out: u64 = parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0);
        let errors_out: u64 = parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0);
        let bytes_out: u64 = parts.get(9).and_then(|s| s.parse().ok()).unwrap_or(0);

        seen.push(name.clone());

        interfaces.push(NetworkStats {
            interface: name,
            packets_in,
            packets_out,
            bytes_in,
            bytes_out,
            errors_in,
            errors_out,
            timestamp,
        });
    }

    interfaces
}

/// Get primary interface (usually en0 on Mac)
#[allow(dead_code)]
pub fn get_primary_interface() -> Option<NetworkStats> {
    let interfaces = get_network_stats();

    // Prefer en0 (WiFi/Ethernet)
    interfaces.into_iter().find(|i| i.interface == "en0")
}
