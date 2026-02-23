// src/ui/layout.rs
// Box drawing characters and layout helpers

use super::colors::*;

// Box characters
pub const BOX_TL: char = '┌'; // top-left
pub const BOX_TR: char = '┐'; // top-right
pub const BOX_BL: char = '└'; // bottom-left
pub const BOX_BR: char = '┘'; // bottom-right
pub const BOX_H: char = '─'; // horizontal
pub const BOX_V: char = '│'; // vertical

#[allow(dead_code)]
pub const BOX_T_DOWN: char = '┬';
#[allow(dead_code)]
pub const BOX_T_UP: char = '┴';
#[allow(dead_code)]
pub const BOX_T_RIGHT: char = '├';
#[allow(dead_code)]
pub const BOX_T_LEFT: char = '┤';
#[allow(dead_code)]
pub const BOX_CROSS: char = '┼';

// Double line box
#[allow(dead_code)]
pub const DBOX_TL: char = '╔';
#[allow(dead_code)]
pub const DBOX_TR: char = '╗';
#[allow(dead_code)]
pub const DBOX_BL: char = '╚';
#[allow(dead_code)]
pub const DBOX_BR: char = '╝';
pub const DBOX_H: char = '═';
#[allow(dead_code)]
pub const DBOX_V: char = '║';

/// Draw a box with title
#[allow(dead_code)]
pub fn draw_box(width: usize, height: usize, title: &str, color: &str) -> Vec<String> {
    let mut lines = Vec::new();

    // Top border with title
    let title_display = if title.is_empty() {
        String::new()
    } else {
        format!(" {} ", title)
    };

    let title_len = title_display.chars().count();
    let remaining = width.saturating_sub(title_len + 2);
    let left_pad = 2;
    let right_pad = remaining.saturating_sub(left_pad);

    let top = format!(
        "{}{}{}{}{}{}{}{}",
        color,
        BOX_TL,
        BOX_H.to_string().repeat(left_pad),
        BOLD,
        title_display,
        RESET,
        color,
        BOX_H.to_string().repeat(right_pad),
    );
    lines.push(format!("{}{}{}", top, BOX_TR, RESET));

    // Middle lines (empty)
    for _ in 0..height.saturating_sub(2) {
        let middle = format!(
            "{}{}{}{}{}",
            color,
            BOX_V,
            " ".repeat(width - 2),
            BOX_V,
            RESET
        );
        lines.push(middle);
    }

    // Bottom border
    let bottom = format!(
        "{}{}{}{}{}",
        color,
        BOX_BL,
        BOX_H.to_string().repeat(width - 2),
        BOX_BR,
        RESET
    );
    lines.push(bottom);

    lines
}

/// Section header with double line
pub fn section_header(title: &str, width: usize, color: &str) -> String {
    let title_display = format!(" {} ", title);
    let title_len = title_display.chars().count();
    let remaining = width.saturating_sub(title_len);
    let left = 2;
    let right = remaining.saturating_sub(left);

    format!(
        "{}{}{}{}{}{}",
        color,
        DBOX_H.to_string().repeat(left),
        BOLD,
        title_display,
        RESET,
        DBOX_H.to_string().repeat(right),
    )
}

/// Horizontal divider
#[allow(dead_code)]
pub fn divider(width: usize, color: &str) -> String {
    format!("{}{}{}", color, BOX_H.to_string().repeat(width), RESET)
}

/// Inner box (for realtime/history sections)
pub fn inner_box(content: &[String], title: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();

    // Top with title
    let title_str = format!("─ {} ", title);
    let title_len = title_str.chars().count();
    let right_pad = width.saturating_sub(title_len + 1);

    lines.push(format!(
        "{}{}{}{}{}",
        DIM,
        BOX_TL,
        title_str,
        BOX_H.to_string().repeat(right_pad),
        BOX_TR
    ));

    // Content
    for line in content {
        let line_len = strip_ansi(line).chars().count();
        let padding = width.saturating_sub(line_len + 4);
        lines.push(format!(
            "{}{}{}  {}{}{}{}",
            DIM,
            BOX_V,
            RESET,
            line,
            " ".repeat(padding),
            DIM,
            BOX_V
        ));
    }

    // Bottom
    lines.push(format!(
        "{}{}{}{}{}",
        DIM,
        BOX_BL,
        BOX_H.to_string().repeat(width - 2),
        BOX_BR,
        RESET
    ));

    lines
}

/// Strip ANSI codes to calculate actual character length
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }

    result
}
