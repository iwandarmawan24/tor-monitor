// src/app.rs
// Application state and history storage

use crate::metrics::{cpu, disk, memory, network, uptime};
use crate::utils::history::History;
use std::time::Instant;

/// History capacity (60 samples = 1 minute at 1 sample/sec)
const HISTORY_SIZE: usize = 60;

pub struct App {
    // History tracking
    pub cpu_history: History,
    pub mem_history: History,
    pub disk_read_history: History,
    pub disk_write_history: History,
    pub net_in_history: History,
    pub net_out_history: History,

    // Current metrics
    pub cpu: Option<cpu::CpuDetailedInfo>,
    pub memory: Option<memory::MemoryInfo>,
    pub disks: Vec<disk::DiskStats>,
    pub networks: Vec<network::NetworkStats>,
    pub uptime_secs: u64,

    // Previous samples for rate calculation
    pub prev_disks: Vec<disk::DiskStats>,
    pub prev_networks: Vec<network::NetworkStats>,

    // Session totals
    pub session_start: Instant,
    pub session_disk_read: u64,
    pub session_disk_write: u64,
    pub session_net_in: u64,
    pub session_net_out: u64,

    // UI state
    pub running: bool,
    pub selected_interface: usize,
    pub selected_disk: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            cpu_history: History::new(HISTORY_SIZE),
            mem_history: History::new(HISTORY_SIZE),
            disk_read_history: History::new(HISTORY_SIZE),
            disk_write_history: History::new(HISTORY_SIZE),
            net_in_history: History::new(HISTORY_SIZE),
            net_out_history: History::new(HISTORY_SIZE),

            cpu: None,
            memory: None,
            disks: Vec::new(),
            networks: Vec::new(),
            uptime_secs: 0,

            prev_disks: Vec::new(),
            prev_networks: Vec::new(),

            session_start: Instant::now(),
            session_disk_read: 0,
            session_disk_write: 0,
            session_net_in: 0,
            session_net_out: 0,

            running: true,
            selected_interface: 0,
            selected_disk: 0,
        }
    }

    pub fn update(&mut self) {
        // Save previous for rate calculation
        self.prev_disks = self.disks.clone();
        self.prev_networks = self.networks.clone();

        // Fetch new data
        self.cpu = cpu::get_cpu_info();
        self.memory = memory::get_memory_info();
        self.disks = disk::get_disk_stats();
        self.networks = network::get_network_stats();
        self.uptime_secs = uptime::get_uptime().map(|u| u.seconds).unwrap_or(0);

        // Update CPU history
        if let Some(ref cpu) = self.cpu {
            self.cpu_history.push(cpu.overall.usage);
        }

        // Update memory history
        if let Some(ref mem) = self.memory {
            self.mem_history.push(mem.usage_percent());
        }

        // Update disk history and session totals
        if let Some(rates) = self.get_disk_rates() {
            // Normalize to percentage (arbitrary max for visualization)
            let max_speed: u64 = 500 * 1024 * 1024; // 500 MB/s
            let read_pct =
                (rates.read_bytes_per_sec as f64 / max_speed as f64 * 100.0) as f32;
            let write_pct =
                (rates.write_bytes_per_sec as f64 / max_speed as f64 * 100.0) as f32;

            self.disk_read_history.push(read_pct.min(100.0));
            self.disk_write_history.push(write_pct.min(100.0));

            // Session totals
            self.session_disk_read += rates.read_bytes_per_sec;
            self.session_disk_write += rates.write_bytes_per_sec;
        }

        // Update network history and session totals
        if let Some(rates) = self.get_network_rates() {
            // Normalize to percentage
            let max_speed: u64 = 100 * 1024 * 1024; // 100 MB/s
            let in_pct = (rates.bytes_in_per_sec as f64 / max_speed as f64 * 100.0) as f32;
            let out_pct =
                (rates.bytes_out_per_sec as f64 / max_speed as f64 * 100.0) as f32;

            self.net_in_history.push(in_pct.min(100.0));
            self.net_out_history.push(out_pct.min(100.0));

            // Session totals
            self.session_net_in += rates.bytes_in_per_sec;
            self.session_net_out += rates.bytes_out_per_sec;
        }
    }

    pub fn get_disk_rates(&self) -> Option<disk::DiskRates> {
        let curr = self.disks.get(self.selected_disk)?;
        let prev = self.prev_disks.get(self.selected_disk);

        // If no previous data, return current rates from iostat
        match prev {
            Some(p) => Some(curr.calculate_rates(p)),
            None => Some(disk::DiskRates {
                reads_per_sec: curr.reads_per_sec,
                writes_per_sec: curr.writes_per_sec,
                read_bytes_per_sec: curr.read_bytes_per_sec,
                write_bytes_per_sec: curr.write_bytes_per_sec,
            }),
        }
    }

    pub fn get_network_rates(&self) -> Option<network::NetworkRates> {
        let curr = self.networks.get(self.selected_interface)?;
        let prev = self.prev_networks.get(self.selected_interface)?;
        Some(curr.calculate_rates(prev))
    }

    /// Get session duration in seconds
    #[allow(dead_code)]
    pub fn session_duration(&self) -> u64 {
        self.session_start.elapsed().as_secs()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
