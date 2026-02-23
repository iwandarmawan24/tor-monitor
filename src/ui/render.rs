// src/ui/render.rs
// Main rendering function for the monitor display

use crate::app::App;
use crate::ui::colors::*;
use crate::ui::graphs::*;
use crate::ui::layout::*;
use crate::utils::format::*;

const WIDTH: usize = 78;

/// Sparkline characters
const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn render(app: &App) -> String {
    let mut output = String::new();

    // Clear screen and move cursor to top
    output.push_str(CLEAR_SCREEN);
    output.push_str(CURSOR_HOME);

    // Header
    output.push_str(&render_header(app));
    output.push('\n');

    // CPU Section
    output.push_str(&render_cpu_section(app));
    output.push('\n');

    // Memory Section
    output.push_str(&render_memory_section(app));
    output.push('\n');

    // Disk Section
    output.push_str(&render_disk_section(app));
    output.push('\n');

    // Network Section
    output.push_str(&render_network_section(app));
    output.push('\n');

    // Footer
    output.push_str(&render_footer());

    output
}

fn render_header(app: &App) -> String {
    let uptime = format_duration(app.uptime_secs);
    let title = "TOR-MONITOR - Resource Monitor";
    let uptime_label = format!("Uptime: {}", uptime);

    let padding = WIDTH.saturating_sub(title.len() + uptime_label.len() + 6);

    let line1 = format!(
        "{}{}╔{}╗{}",
        CYAN, BOLD, "═".repeat(WIDTH - 2), RESET
    );
    let line2 = format!(
        "{}║  {}{}{}{}{}{}{}  ║{}",
        CYAN,
        BRIGHT_WHITE, BOLD, title, RESET,
        " ".repeat(padding),
        BRIGHT_YELLOW, uptime_label,
        RESET
    );
    let line3 = format!(
        "{}╚{}╝{}",
        CYAN, "═".repeat(WIDTH - 2), RESET
    );

    format!("{}\n{}\n{}", line1, line2, line3)
}

fn render_cpu_section(app: &App) -> String {
    let mut lines = Vec::new();

    lines.push(section_header("CPU", WIDTH, BRIGHT_CYAN));

    if let Some(ref cpu) = app.cpu {
        // Main usage bar
        let bar = progress_bar(cpu.overall.usage, 30);
        lines.push(format!(
            "  {} {:>5.1}%   {}User:{} {:.1}%  {}Sys:{} {:.1}%  {}Idle:{} {:.1}%",
            bar,
            cpu.overall.usage,
            DIM,
            RESET,
            cpu.overall.user,
            DIM,
            RESET,
            cpu.overall.system,
            DIM,
            RESET,
            cpu.overall.idle
        ));

        lines.push(String::new());

        // Per-core bars (3 per row)
        let cores_per_row = 3;
        for core_line in core_bars(&cpu.per_core, cores_per_row) {
            lines.push(format!("  {}", core_line));
        }

        lines.push(String::new());

        // Process and thread count
        lines.push(format!(
            "  {}Threads:{} {}    {}Processes:{} {}",
            DIM,
            RESET,
            format_number(cpu.thread_count as u64),
            DIM,
            RESET,
            format_number(cpu.process_count as u64)
        ));

        lines.push(String::new());

        // History sparkline
        let history_data = app.cpu_history.get_data();
        let spark = sparkline(&history_data, WIDTH - 14);
        lines.push(format!("  {}History:{} {}", DIM, RESET, spark));
    } else {
        lines.push("  No CPU data available".to_string());
    }

    lines.join("\n")
}

fn render_memory_section(app: &App) -> String {
    let mut lines = Vec::new();

    lines.push(section_header("MEMORY", WIDTH, BRIGHT_GREEN));

    if let Some(ref mem) = app.memory {
        // Main usage bar
        let bar = progress_bar(mem.usage_percent(), 30);
        lines.push(format!("  {} {:>5.1}%", bar, mem.usage_percent()));

        lines.push(String::new());

        // Memory breakdown
        lines.push(format!(
            "  {}Physical:{} {}   {}Used:{} {}   {}Cached:{} {}   {}Swap:{} {}",
            DIM,
            RESET,
            format_bytes(mem.physical_total),
            DIM,
            RESET,
            format_bytes(mem.used),
            DIM,
            RESET,
            format_bytes(mem.cached),
            DIM,
            RESET,
            format_bytes(mem.swap_used)
        ));

        lines.push(String::new());

        // History sparkline
        let history_data = app.mem_history.get_data();
        let spark = sparkline(&history_data, WIDTH - 14);
        lines.push(format!("  {}History:{} {}", DIM, RESET, spark));
    } else {
        lines.push("  No memory data available".to_string());
    }

    lines.join("\n")
}

fn render_disk_section(app: &App) -> String {
    let mut lines = Vec::new();

    let disk_name = app
        .disks
        .get(app.selected_disk)
        .map(|d| d.mount_point.as_str())
        .unwrap_or("N/A");

    lines.push(section_header(
        &format!("DISK I/O  [{}]", disk_name),
        WIDTH,
        BRIGHT_YELLOW,
    ));

    if let Some(rates) = app.get_disk_rates() {
        // Realtime stats
        let realtime = vec![
            format!(
                "{}↓{} Read:  {:>12}",
                BRIGHT_GREEN,
                RESET,
                format_speed(rates.read_bytes_per_sec)
            ),
            format!(
                "{}↑{} Write: {:>12}",
                BRIGHT_RED,
                RESET,
                format_speed(rates.write_bytes_per_sec)
            ),
        ];

        for line in inner_box(&realtime, "Realtime", 30) {
            lines.push(format!("  {}", line));
        }

        lines.push(String::new());

        // History sparklines
        let read_data = app.disk_read_history.get_data();
        let write_data = app.disk_write_history.get_data();

        lines.push(format!(
            "  {}Read: {} {}",
            BRIGHT_GREEN,
            RESET,
            sparkline_static(&read_data, WIDTH - 12, BRIGHT_GREEN)
        ));
        lines.push(format!(
            "  {}Write:{} {}",
            BRIGHT_RED,
            RESET,
            sparkline_static(&write_data, WIDTH - 12, BRIGHT_RED)
        ));
    } else {
        lines.push("  No disk data available".to_string());
    }

    lines.join("\n")
}

fn render_network_section(app: &App) -> String {
    let mut lines = Vec::new();

    let iface_name = app
        .networks
        .get(app.selected_interface)
        .map(|n| n.interface.as_str())
        .unwrap_or("N/A");

    lines.push(section_header(
        &format!("NETWORK  [{}]", iface_name),
        WIDTH,
        BRIGHT_MAGENTA,
    ));

    if let (Some(rates), Some(stats)) = (
        app.get_network_rates(),
        app.networks.get(app.selected_interface),
    ) {
        // Realtime box
        let realtime = vec![
            format!(
                "{}↓{} In:  {:>10} ({:.0} pkt/s)",
                BRIGHT_CYAN,
                RESET,
                format_speed(rates.bytes_in_per_sec),
                rates.packets_in_per_sec
            ),
            format!(
                "{}↑{} Out: {:>10} ({:.0} pkt/s)",
                BRIGHT_MAGENTA,
                RESET,
                format_speed(rates.bytes_out_per_sec),
                rates.packets_out_per_sec
            ),
        ];

        // Totals box
        let totals = vec![
            format!(
                "{}↓{} In:  {} ({} pkts)",
                BRIGHT_CYAN,
                RESET,
                format_bytes(stats.bytes_in),
                format_compact(stats.packets_in)
            ),
            format!(
                "{}↑{} Out: {} ({} pkts)",
                BRIGHT_MAGENTA,
                RESET,
                format_bytes(stats.bytes_out),
                format_compact(stats.packets_out)
            ),
        ];

        // Side by side boxes
        let rt_box = inner_box(&realtime, "Realtime", 34);
        let tot_box = inner_box(&totals, "Total", 34);

        for i in 0..rt_box.len().max(tot_box.len()) {
            let left = rt_box.get(i).map(|s| s.as_str()).unwrap_or("");
            let right = tot_box.get(i).map(|s| s.as_str()).unwrap_or("");
            lines.push(format!("  {}  {}", left, right));
        }

        lines.push(String::new());

        // History sparklines
        let in_data = app.net_in_history.get_data();
        let out_data = app.net_out_history.get_data();

        lines.push(format!(
            "  {}In: {} {}",
            BRIGHT_CYAN,
            RESET,
            sparkline_static(&in_data, WIDTH - 10, BRIGHT_CYAN)
        ));
        lines.push(format!(
            "  {}Out:{} {}",
            BRIGHT_MAGENTA,
            RESET,
            sparkline_static(&out_data, WIDTH - 10, BRIGHT_MAGENTA)
        ));
    } else {
        lines.push("  No network data available".to_string());
    }

    lines.join("\n")
}

fn render_footer() -> String {
    format!(
        "\n{}  Press 'q' to quit | 'd' cycle Disk | 'n' cycle Network{}",
        DIM, RESET
    )
}

/// Sparkline with a single static color
fn sparkline_static(data: &[f32], width: usize, color: &str) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    let start = data.len().saturating_sub(width);
    let visible_data = &data[start..];

    let max_val = visible_data.iter().cloned().fold(0.0_f32, f32::max);
    let max_val = if max_val == 0.0 { 1.0 } else { max_val };

    let mut result = String::new();
    result.push_str(color);

    for &value in visible_data {
        let normalized = (value / max_val * 100.0).clamp(0.0, 100.0);
        let index = ((normalized / 100.0) * 7.0) as usize;
        let index = index.min(7);
        result.push(SPARK_CHARS[index]);
    }

    let padding = width.saturating_sub(visible_data.len());
    result.push_str(RESET);
    result.push_str(&" ".repeat(padding));

    result
}
