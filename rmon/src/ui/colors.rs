// src/ui/colors.rs
// ANSI escape codes for terminal colors and styling

// Reset
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

// Regular colors
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

// Bright colors
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

// Background colors
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";

// 256 color mode: \x1b[38;5;{n}m for foreground
#[allow(dead_code)]
pub fn fg_256(n: u8) -> String {
    format!("\x1b[38;5;{}m", n)
}

#[allow(dead_code)]
pub fn bg_256(n: u8) -> String {
    format!("\x1b[48;5;{}m", n)
}

// RGB true color: \x1b[38;2;{r};{g};{b}m
pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

// Gradient colors for graphs (green -> yellow -> red)
pub fn gradient_color(percentage: f32) -> String {
    if percentage < 50.0 {
        // Green to yellow (0-50%)
        let ratio = percentage / 50.0;
        let r = (255.0 * ratio) as u8;
        let g = 255;
        fg_rgb(r, g, 0)
    } else {
        // Yellow to red (50-100%)
        let ratio = (percentage - 50.0) / 50.0;
        let r = 255;
        let g = (255.0 * (1.0 - ratio)) as u8;
        fg_rgb(r, g, 0)
    }
}

// Terminal control
pub const CLEAR_SCREEN: &str = "\x1b[2J";
pub const CURSOR_HOME: &str = "\x1b[H";
pub const CURSOR_HIDE: &str = "\x1b[?25l";
pub const CURSOR_SHOW: &str = "\x1b[?25h";

#[allow(dead_code)]
pub fn cursor_to(row: u16, col: u16) -> String {
    format!("\x1b[{};{}H", row, col)
}
