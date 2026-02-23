# Tor-Monitor

```
████████╗ ██████╗ ██████╗       ███╗   ███╗ ██████╗ ███╗   ██╗██╗████████╗ ██████╗ ██████╗
╚══██╔══╝██╔═══██╗██╔══██╗      ████╗ ████║██╔═══██╗████╗  ██║██║╚══██╔══╝██╔═══██╗██╔══██╗
   ██║   ██║   ██║██████╔╝█████╗██╔████╔██║██║   ██║██╔██╗ ██║██║   ██║   ██║   ██║██████╔╝
   ██║   ██║   ██║██╔══██╗╚════╝██║╚██╔╝██║██║   ██║██║╚██╗██║██║   ██║   ██║   ██║██╔══██╗
   ██║   ╚██████╔╝██║  ██║      ██║ ╚═╝ ██║╚██████╔╝██║ ╚████║██║   ██║   ╚██████╔╝██║  ██║
   ╚═╝    ╚═════╝ ╚═╝  ╚═╝      ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝
                            System Resource Monitor for macOS
```

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-macOS-blue.svg)](https://www.apple.com/macos/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/Dependencies-Zero-brightgreen.svg)](#zero-dependencies)

A lightweight, terminal-based system resource monitor for macOS written in pure Rust with **zero external dependencies**. Features real-time CPU, memory, disk I/O, and network monitoring with colorful graphical displays and historical sparklines.

---

## Table of Contents

- [Features](#features)
- [Demo](#demo)
- [Installation](#installation)
- [Usage](#usage)
- [Architecture](#architecture)
- [Technical Details](#technical-details)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [Limitations](#limitations)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)

---

## Features

### Core Monitoring

| Resource | Metrics | Visualization |
|----------|---------|---------------|
| **CPU** | User, System, Idle %, Per-core usage | Progress bars, Sparklines |
| **Memory** | Physical, Used, Cached, Swap | Progress bar, Sparkline |
| **Disk I/O** | Read/Write speeds (MB/s) | Dual sparklines |
| **Network** | In/Out bytes, Packets, Rates | Dual sparklines |
| **System** | Uptime, Process count, Thread count | Text display |

### Visual Features

- **Color-coded progress bars** — Green → Yellow → Red gradient based on usage
- **Real-time sparklines** — 60-second rolling history graphs
- **Box-drawing characters** — Clean, organized terminal UI
- **ANSI true color** — 24-bit RGB color support
- **Non-blocking input** — Responsive keyboard controls

### Zero Dependencies

This project uses **only the Rust standard library** — no external crates required. All functionality is implemented from scratch:

- ANSI escape code handling
- Terminal raw mode control
- System metrics collection via shell commands
- Ring buffer for history tracking
- Human-readable formatting

---

## Demo

```
╔════════════════════════════════════════════════════════════════════════════╗
║  TOR-MONITOR - Resource Monitor                             Uptime: 5d 3h  ║
╚════════════════════════════════════════════════════════════════════════════╝
══ CPU ═══════════════════════════════════════════════════════════════════════
  ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱  48.2%   User: 32.1%  Sys: 16.1%  Idle: 51.8%

  C0  ▰▰▰▰▰▰▰▱▱▱  65%   C1  ▰▰▰▰▱▱▱▱▱▱  38%   C2  ▰▰▰▱▱▱▱▱▱▱  28%
  C3  ▰▰▰▰▰▰▱▱▱▱  55%   C4  ▰▰▱▱▱▱▱▱▱▱  18%   C5  ▰▰▰▰▰▱▱▱▱▱  45%

  Threads: 1,842    Processes: 423

  History: ▁▂▃▅▇█▇▅▄▃▂▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▁▂▃▄▅▆▇▆▅▄▃▂▁▂▃▄▅▆█▇▅▄▃▂▁▁▂▃▄▅
══ MEMORY ════════════════════════════════════════════════════════════════════
  ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱  75.2%

  Physical: 16.00 GB   Used: 12.03 GB   Cached: 3.21 GB   Swap: 1.50 GB

  History: ▅▅▅▅▆▆▆▆▇▇▇▇▇▇▇▆▆▆▆▆▆▆▆▆▆▆▆▆▆▆▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅▅
══ DISK I/O  [/] ═════════════════════════════════════════════════════════════
  ┌─ Realtime ────────────────────┐
  │  ↓ Read:     125.30 MB/s      │
  │  ↑ Write:     42.10 MB/s      │
  └───────────────────────────────┘

  Read:  ▁▁▂▃▅▇███▇▅▃▂▁▁▁▂▃▄▅▆▇▆▅▄▃▂▁▁▂▃▄▅▆▇▆▅▄▃▂▁▁▂▃▄▅▆▇
  Write: ▁▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃
══ NETWORK  [en0] ════════════════════════════════════════════════════════════
  ┌─ Realtime ──────────────────┐  ┌─ Total ────────────────────────┐
  │  ↓ In:   2.30 MB/s (1200 p) │  │  ↓ In:  1.23 GB (892k pkts)    │
  │  ↑ Out:  0.50 MB/s (800 p)  │  │  ↑ Out: 0.45 GB (523k pkts)    │
  └─────────────────────────────┘  └─────────────────────────────────┘

  In:  ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▁▂▃▄▅▆▇█▇▆▅▄▃▂▁▁▂▃▄▅▆▇
  Out: ▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃▄▄▃▃▂▂▁▁▁▂▂▃▃▄▄

  Press 'q' to quit | 'd' cycle Disk | 'n' cycle Network
```

---

## Installation

### Prerequisites

- **macOS** 10.15 or later
- **Rust** 1.70 or later

### Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yayasandarmawan/tor-monitor.git
cd tor-monitor

# Build release version
cargo build --release

# Run
./target/release/tor-monitor
```

### Install System-wide

```bash
cargo install --path .

# Now you can run from anywhere
tor-monitor
```

### Binary Size

The release binary is only **~430 KB** thanks to:
- Zero external dependencies
- LTO (Link-Time Optimization)
- Symbol stripping
- Size optimization level 3

---

## Usage

### Keyboard Controls

| Key | Action |
|-----|--------|
| `q` | Quit the application |
| `d` | Cycle through disk devices |
| `n` | Cycle through network interfaces |
| `Ctrl+C` | Force quit |

### Command Line

```bash
# Basic usage
tor-monitor

# Run in background (not recommended for interactive use)
tor-monitor &
```

### Tips

- **Terminal Size**: Best viewed in a terminal at least 80 columns wide
- **Color Support**: Use a terminal with 24-bit color support (iTerm2, Terminal.app, Alacritty)
- **Refresh Rate**: Data updates every 1 second
- **History**: Sparklines show 60 seconds of history

---

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────┐
│                    Tor-Monitor Architecture                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐                │
│  │  main.rs │────▶│  app.rs  │────▶│ render.rs│────▶ Terminal  │
│  │          │     │  (State) │     │   (UI)   │                 │
│  └──────────┘     └──────────┘     └──────────┘                 │
│        │               │                                         │
│        │               ▼                                         │
│        │         ┌──────────┐                                   │
│        │         │ history  │                                   │
│        │         │  (Ring   │                                   │
│        │         │  Buffer) │                                   │
│        │         └──────────┘                                   │
│        │                                                         │
│        ▼                                                         │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                     Metrics Layer                        │   │
│  ├─────────┬─────────┬─────────┬─────────┬─────────────────┤   │
│  │  cpu.rs │memory.rs│ disk.rs │network.rs│   uptime.rs    │   │
│  └─────────┴─────────┴─────────┴─────────┴─────────────────┘   │
│        │         │         │         │              │           │
│        ▼         ▼         ▼         ▼              ▼           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              macOS System Commands                       │   │
│  │    top    vm_stat    df    iostat    netstat    sysctl   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
1. Main Loop (1 second interval)
   │
   ├──▶ Collect Metrics
   │    ├── CPU: top -l 2 -n 0
   │    ├── Memory: vm_stat, sysctl
   │    ├── Disk: df, iostat
   │    ├── Network: netstat -ibn
   │    └── Uptime: sysctl kern.boottime
   │
   ├──▶ Update App State
   │    ├── Store current values
   │    ├── Calculate rates (current - previous) / elapsed
   │    └── Push to history ring buffers
   │
   ├──▶ Render UI
   │    ├── Clear screen
   │    ├── Draw header with uptime
   │    ├── Draw CPU section with bars and sparkline
   │    ├── Draw Memory section
   │    ├── Draw Disk I/O section
   │    ├── Draw Network section
   │    └── Draw footer with controls
   │
   └──▶ Handle Input (non-blocking)
        ├── 'q' → Exit
        ├── 'd' → Cycle disk
        └── 'n' → Cycle network interface
```

---

## Technical Details

### System Commands Used

| Metric | Command | Purpose |
|--------|---------|---------|
| CPU Usage | `top -l 2 -n 0 -s 1` | Get user/sys/idle percentages |
| Core Count | `sysctl -n hw.ncpu` | Number of CPU cores |
| Memory | `vm_stat` | Page statistics |
| Total RAM | `sysctl -n hw.memsize` | Physical memory size |
| Swap | `sysctl vm.swapusage` | Swap usage |
| Disk Capacity | `df -k` | Filesystem usage |
| Disk I/O | `iostat -d -c 2 -w 1` | Read/write rates |
| Network | `netstat -ibn` | Interface statistics |
| Uptime | `sysctl kern.boottime` | Boot timestamp |

### ANSI Escape Codes

| Code | Purpose |
|------|---------|
| `\x1b[2J` | Clear screen |
| `\x1b[H` | Cursor home |
| `\x1b[?25l` | Hide cursor |
| `\x1b[?25h` | Show cursor |
| `\x1b[38;2;R;G;Bm` | RGB foreground color |
| `\x1b[0m` | Reset formatting |

### Unicode Characters

| Character | Name | Usage |
|-----------|------|-------|
| `▰` `▱` | Block elements | Progress bars |
| `▁▂▃▄▅▆▇█` | Block elements | Sparklines |
| `═` `║` `╔` `╗` `╚` `╝` | Double box drawing | Headers |
| `─` `│` `┌` `┐` `└` `┘` | Single box drawing | Inner boxes |
| `↓` `↑` | Arrows | In/Out indicators |

### Color Gradient Algorithm

```rust
fn gradient_color(percentage: f32) -> RGB {
    if percentage < 50.0 {
        // Green to Yellow (0-50%)
        let ratio = percentage / 50.0;
        RGB(255 * ratio, 255, 0)
    } else {
        // Yellow to Red (50-100%)
        let ratio = (percentage - 50.0) / 50.0;
        RGB(255, 255 * (1 - ratio), 0)
    }
}
```

### Ring Buffer Implementation

```rust
pub struct History {
    data: Vec<f32>,      // Fixed-size buffer
    capacity: usize,     // Maximum items
    write_pos: usize,    // Next write position
    len: usize,          // Current item count
}

// Push overwrites oldest when full
// O(1) push, O(n) read in chronological order
```

---

## Project Structure

```
tor-monitor/
├── Cargo.toml              # Project manifest
├── README.md               # This file
├── LICENSE                 # MIT License
├── .gitignore              # Git ignore rules
└── src/
    ├── main.rs             # Entry point, main loop, input handling
    ├── app.rs              # Application state, history management
    │
    ├── ui/                 # User Interface
    │   ├── mod.rs          # Module exports
    │   ├── colors.rs       # ANSI escape codes, color functions
    │   ├── graphs.rs       # Progress bars, sparklines
    │   ├── layout.rs       # Box drawing, section headers
    │   └── render.rs       # Main render function
    │
    ├── metrics/            # System Metrics Collection
    │   ├── mod.rs          # Module exports
    │   ├── cpu.rs          # CPU usage, per-core, threads
    │   ├── memory.rs       # RAM, swap, cache
    │   ├── disk.rs         # Disk I/O rates
    │   ├── network.rs      # Network I/O rates
    │   └── uptime.rs       # System uptime
    │
    └── utils/              # Utilities
        ├── mod.rs          # Module exports
        ├── format.rs       # Human-readable formatting
        └── history.rs      # Ring buffer implementation
```

### Module Responsibilities

| Module | Lines | Responsibility |
|--------|-------|----------------|
| `main.rs` | ~80 | Application lifecycle, input handling |
| `app.rs` | ~120 | State management, history updates |
| `render.rs` | ~200 | UI composition and output |
| `colors.rs` | ~60 | ANSI codes, color utilities |
| `graphs.rs` | ~160 | Visual components |
| `layout.rs` | ~100 | Box drawing, layout helpers |
| `cpu.rs` | ~100 | CPU metrics collection |
| `memory.rs` | ~100 | Memory metrics collection |
| `disk.rs` | ~120 | Disk I/O collection |
| `network.rs` | ~80 | Network I/O collection |
| `uptime.rs` | ~40 | Uptime calculation |
| `history.rs` | ~80 | Ring buffer |
| `format.rs` | ~80 | Number/byte formatting |

**Total: ~1,300 lines of Rust**

---

## How It Works

### 1. Startup Sequence

```
1. Setup terminal (hide cursor, set raw mode)
2. Create App state with empty history buffers
3. Fetch initial metrics (2 samples for accurate rates)
4. Spawn input handling thread
5. Enter main loop
```

### 2. Main Loop (every 1 second)

```
1. Check for keyboard input (non-blocking)
2. Fetch all system metrics
3. Calculate rates from previous samples
4. Push values to history ring buffers
5. Render full UI to stdout
6. Sleep 1 second
```

### 3. Metrics Collection

Each metric module follows the same pattern:

```rust
pub fn get_<metric>_info() -> Option<MetricInfo> {
    // 1. Run system command
    let output = Command::new("command")
        .args(["arg1", "arg2"])
        .output()
        .ok()?;

    // 2. Parse stdout
    let stdout = String::from_utf8_lossy(&output.stdout);

    // 3. Extract values
    let value = parse_value(&stdout)?;

    // 4. Return structured data
    Some(MetricInfo { value, ... })
}
```

### 4. Rendering Pipeline

```
render() → String
    │
    ├── render_header()     → "╔═══ TOR-MONITOR ═══╗"
    ├── render_cpu()        → bars + sparkline
    ├── render_memory()     → bar + sparkline
    ├── render_disk()       → rates + sparklines
    ├── render_network()    → rates + sparklines
    └── render_footer()     → "Press 'q' to quit"
```

### 5. Cleanup

```
1. Restore terminal mode (canonical, echo)
2. Show cursor
3. Clear screen
4. Print goodbye message
```

---

## Limitations

### Known Limitations

| Limitation | Reason | Workaround |
|------------|--------|------------|
| Per-core CPU simulated | macOS doesn't expose via CLI | Use IOKit FFI |
| Disk read/write split estimated | `iostat` shows combined | Use `fs_usage` (requires sudo) |
| Requires shell commands | No external crates | Could use `sysinfo` crate |
| macOS only | Uses macOS-specific commands | Implement Linux/Windows modules |
| Fixed refresh rate | Hardcoded 1 second | Add CLI flag |
| Fixed width | Hardcoded 78 columns | Detect terminal size |

### Performance Considerations

- Each update spawns 5-6 shell processes
- CPU overhead: ~1-2% during updates
- Memory usage: ~5 MB
- Binary size: ~430 KB

---

## Roadmap

### Version 0.2.0 (Planned)

- [ ] Dynamic terminal size detection
- [ ] Configurable refresh rate
- [ ] Process list view (top processes)
- [ ] Temperature monitoring (via `powermetrics`)
- [ ] Battery status (for MacBooks)

### Version 0.3.0 (Future)

- [ ] Linux support
- [ ] Configuration file
- [ ] Color themes
- [ ] Export to JSON/CSV
- [ ] Alert thresholds

### Version 1.0.0 (Goal)

- [ ] FFI for direct system calls (no shell commands)
- [ ] Windows support
- [ ] Plugin system
- [ ] Remote monitoring

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy (`cargo clippy`)
- Add tests for new functionality
- Update documentation

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2024 Yayasan Darmawan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## Author

**Yayasan Darmawan**

- GitHub: [@yayasandarmawan](https://github.com/yayasandarmawan)
- Project: [tor-monitor](https://github.com/yayasandarmawan/tor-monitor)

---

## Acknowledgments

- Inspired by [htop](https://htop.dev/), [btop](https://github.com/aristocratos/btop), and [gtop](https://github.com/aksakalli/gtop)
- Unicode block characters from [Unicode Standard](https://unicode.org/)
- ANSI escape codes reference from [ANSI Escape Sequences](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)

---

<p align="center">
  Made with ❤️ and Rust
</p>
