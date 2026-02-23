// src/metrics/memory.rs
// Memory usage metrics for macOS

use std::process::Command;

pub struct MemoryInfo {
    pub physical_total: u64, // Total RAM
    pub used: u64,           // Currently used
    pub free: u64,           // Free
    pub cached: u64,         // File cache
    pub wired: u64,          // Wired (kernel, can't be swapped)
    pub active: u64,         // Active memory
    pub inactive: u64,       // Inactive (can be reclaimed)
    pub swap_total: u64,     // Total swap
    pub swap_used: u64,      // Used swap
}

impl MemoryInfo {
    /// Percentage of memory used
    pub fn usage_percent(&self) -> f32 {
        if self.physical_total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.physical_total as f64 * 100.0) as f32
    }

    /// Percentage of swap used
    #[allow(dead_code)]
    pub fn swap_percent(&self) -> f32 {
        if self.swap_total == 0 {
            return 0.0;
        }
        (self.swap_used as f64 / self.swap_total as f64 * 100.0) as f32
    }
}

pub fn get_memory_info() -> Option<MemoryInfo> {
    let total = get_total_memory()?;
    let (page_size, stats) = get_vm_stats()?;
    let (swap_total, swap_used) = get_swap_info();

    let free = (stats.free + stats.speculative) * page_size;
    let wired = stats.wired * page_size;
    let active = stats.active * page_size;
    let inactive = stats.inactive * page_size;
    let cached = stats.purgeable * page_size + stats.file_backed * page_size;
    let used = total - free;

    Some(MemoryInfo {
        physical_total: total,
        used,
        free,
        cached,
        wired,
        active,
        inactive,
        swap_total,
        swap_used,
    })
}

fn get_total_memory() -> Option<u64> {
    let output = Command::new("sysctl")
        .args(["-n", "hw.memsize"])
        .output()
        .ok()?;

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .ok()
}

struct VmStats {
    free: u64,
    active: u64,
    inactive: u64,
    speculative: u64,
    wired: u64,
    purgeable: u64,
    file_backed: u64,
}

fn get_vm_stats() -> Option<(u64, VmStats)> {
    let output = Command::new("vm_stat").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let page_size = parse_page_size(&stdout)?;

    let stats = VmStats {
        free: parse_vm_value(&stdout, "Pages free").unwrap_or(0),
        active: parse_vm_value(&stdout, "Pages active").unwrap_or(0),
        inactive: parse_vm_value(&stdout, "Pages inactive").unwrap_or(0),
        speculative: parse_vm_value(&stdout, "Pages speculative").unwrap_or(0),
        wired: parse_vm_value(&stdout, "Pages wired down").unwrap_or(0),
        purgeable: parse_vm_value(&stdout, "Pages purgeable").unwrap_or(0),
        file_backed: parse_vm_value(&stdout, "File-backed pages").unwrap_or(0),
    };

    Some((page_size, stats))
}

fn parse_page_size(output: &str) -> Option<u64> {
    // "Mach Virtual Memory Statistics: (page size of 16384 bytes)"
    let start = output.find("page size of ")? + 13;
    let end = output[start..].find(' ')?;
    output[start..start + end].parse().ok()
}

fn parse_vm_value(output: &str, key: &str) -> Option<u64> {
    for line in output.lines() {
        if line.starts_with(key) || line.contains(key) {
            let value_str = line.split(':').nth(1)?;
            let cleaned: String = value_str.chars().filter(|c| c.is_ascii_digit()).collect();
            return cleaned.parse().ok();
        }
    }
    None
}

fn get_swap_info() -> (u64, u64) {
    // sysctl vm.swapusage
    // vm.swapusage: total = 2048.00M  used = 512.00M  free = 1536.00M

    let output = match Command::new("sysctl").args(["vm.swapusage"]).output() {
        Ok(o) => o,
        Err(_) => return (0, 0),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    let total = parse_swap_value(&stdout, "total").unwrap_or(0);
    let used = parse_swap_value(&stdout, "used").unwrap_or(0);

    (total, used)
}

fn parse_swap_value(output: &str, key: &str) -> Option<u64> {
    // Find "key = XXX.XXM"
    let key_pos = output.find(&format!("{} = ", key))?;
    let start = key_pos + key.len() + 3;

    let remaining = &output[start..];
    let end = remaining.find(|c: char| !c.is_ascii_digit() && c != '.')?;

    let value: f64 = remaining[..end].parse().ok()?;

    // Check unit (M, G)
    let unit_char = remaining.chars().nth(end)?;
    let multiplier = match unit_char {
        'G' => 1024 * 1024 * 1024,
        'M' => 1024 * 1024,
        'K' => 1024,
        _ => 1,
    };

    Some((value * multiplier as f64) as u64)
}
