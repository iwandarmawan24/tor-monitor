// src/main.rs
// RMON - Resource Monitor for macOS

mod app;
mod metrics;
mod ui;
mod utils;

use app::App;
use ui::colors::*;
use ui::render::render;

use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Setup terminal
    setup_terminal();

    // Create app state
    let mut app = App::new();

    // Initial data fetch (first fetch to populate prev_* for rate calculation)
    app.update();

    // Small delay then fetch again for accurate rates
    thread::sleep(Duration::from_millis(500));
    app.update();

    // Render initial
    print!("{}", render(&app));
    io::stdout().flush().unwrap();

    // Setup input handling (non-blocking)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Set terminal to raw mode for single key input
        set_raw_mode(true);

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = [0u8; 1];

        loop {
            if handle.read_exact(&mut buffer).is_ok() {
                if tx.send(buffer[0]).is_err() {
                    break;
                }
            }
        }
    });

    // Main loop
    loop {
        // Check for input (non-blocking)
        if let Ok(key) = rx.try_recv() {
            match key {
                b'q' | b'Q' | 3 => {
                    // q, Q, or Ctrl+C
                    app.running = false;
                    break;
                }
                b'd' | b'D' => {
                    // Cycle disk selection
                    if !app.disks.is_empty() {
                        app.selected_disk = (app.selected_disk + 1) % app.disks.len();
                    }
                }
                b'n' | b'N' => {
                    // Cycle network interface
                    if !app.networks.is_empty() {
                        app.selected_interface =
                            (app.selected_interface + 1) % app.networks.len();
                    }
                }
                _ => {}
            }
        }

        if !app.running {
            break;
        }

        // Update data
        app.update();

        // Render
        print!("{}", render(&app));
        io::stdout().flush().unwrap();

        // Sleep for 1 second
        thread::sleep(Duration::from_millis(1000));
    }

    // Cleanup terminal
    cleanup_terminal();
}

fn setup_terminal() {
    // Hide cursor
    print!("{}", CURSOR_HIDE);
    io::stdout().flush().unwrap();
}

fn cleanup_terminal() {
    // Restore terminal mode
    set_raw_mode(false);

    // Show cursor
    print!("{}", CURSOR_SHOW);

    // Clear screen
    print!("{}{}", CLEAR_SCREEN, CURSOR_HOME);

    println!("Goodbye!");

    io::stdout().flush().unwrap();
}

/// Set terminal to raw mode (for single key input without Enter)
fn set_raw_mode(enable: bool) {
    use std::process::Command;

    if enable {
        // Disable canonical mode and echo
        let _ = Command::new("stty")
            .args(["-icanon", "-echo"])
            .status();
    } else {
        // Restore canonical mode and echo
        let _ = Command::new("stty")
            .args(["icanon", "echo"])
            .status();
    }
}
