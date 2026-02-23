// src/ui/graphs.rs
// Graphical elements: progress bars, sparklines, charts

use super::colors::*;

// ═══════════════════════════════════════════════════════
// PROGRESS BARS
// ═══════════════════════════════════════════════════════

/// Block-style progress bar: ▰▰▰▰▰▱▱▱▱▱
pub fn progress_bar(percentage: f32, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    let color = gradient_color(percentage);

    format!(
        "{}{}{}{}{}",
        color,
        "▰".repeat(filled),
        DIM,
        "▱".repeat(empty),
        RESET
    )
}

/// Slim bar style: ━━━━━━━━━━░░░░░░░░░░
#[allow(dead_code)]
pub fn slim_bar(percentage: f32, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    let color = gradient_color(percentage);

    format!(
        "{}{}{}{}{}",
        color,
        "━".repeat(filled),
        DIM,
        "░".repeat(empty),
        RESET
    )
}

/// Block bar style with label: [████████░░░░] 65%
#[allow(dead_code)]
pub fn labeled_bar(percentage: f32, width: usize, _label: &str) -> String {
    let inner_width = width.saturating_sub(8); // space for [] and percentage
    let filled = ((percentage / 100.0) * inner_width as f32) as usize;
    let empty = inner_width.saturating_sub(filled);

    let color = gradient_color(percentage);

    format!(
        "{}[{}{}{}{}{}] {:>5.1}%{}",
        DIM,
        color,
        "█".repeat(filled),
        DIM,
        "░".repeat(empty),
        RESET,
        percentage,
        RESET
    )
}

// ═══════════════════════════════════════════════════════
// SPARKLINES (History Graph)
// ═══════════════════════════════════════════════════════

/// Sparkline characters from low to high
const SPARK_CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Create sparkline from history data
/// Data is percentage (0-100)
pub fn sparkline(data: &[f32], width: usize) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    // Get last N data points matching width
    let start = data.len().saturating_sub(width);
    let visible_data = &data[start..];

    let mut result = String::new();

    for &value in visible_data {
        // Clamp value 0-100
        let clamped = value.clamp(0.0, 100.0);

        // Map to index 0-7
        let index = ((clamped / 100.0) * 7.0) as usize;
        let index = index.min(7);

        // Apply color gradient
        let color = gradient_color(clamped);
        result.push_str(&color);
        result.push(SPARK_CHARS[index]);
    }

    // Padding if data is less than width
    let padding = width.saturating_sub(visible_data.len());
    result.push_str(RESET);
    result.push_str(&" ".repeat(padding));

    result
}

/// Sparkline with custom color per value
#[allow(dead_code)]
pub fn sparkline_colored(data: &[f32], width: usize, color_fn: fn(f32) -> String) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    let start = data.len().saturating_sub(width);
    let visible_data = &data[start..];

    let mut result = String::new();

    for &value in visible_data {
        let clamped = value.clamp(0.0, 100.0);
        let index = ((clamped / 100.0) * 7.0) as usize;
        let index = index.min(7);

        result.push_str(&color_fn(clamped));
        result.push(SPARK_CHARS[index]);
    }

    let padding = width.saturating_sub(visible_data.len());
    result.push_str(RESET);
    result.push_str(&" ".repeat(padding));

    result
}

// ═══════════════════════════════════════════════════════
// SPARKLINE WITH STATIC COLOR
// ═══════════════════════════════════════════════════════

/// Sparkline with a single static color (for network/disk)
pub fn sparkline_with_color(data: &[f32], width: usize, color: &str) -> String {
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

// ═══════════════════════════════════════════════════════
// PER-CORE CPU MINI BARS
// ═══════════════════════════════════════════════════════

/// Render multiple small CPU core bars in grid
pub fn core_bars(cores: &[f32], cores_per_row: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for chunk in cores.chunks(cores_per_row) {
        let mut line = String::new();

        for (i, &usage) in chunk.iter().enumerate() {
            let core_num = lines.len() * cores_per_row + i;
            let bar = progress_bar(usage, 8);

            line.push_str(&format!(
                "{}C{:<2}{} {} {:>3.0}%  ",
                CYAN, core_num, RESET, bar, usage
            ));
        }

        lines.push(line);
    }

    lines
}
