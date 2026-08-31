<div align="center">

# ⚡ MyMsg

**Ultra-lightweight, Always-on-Top popup notification CLI tool optimized for resource-constrained environments.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021/2024](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/tkshnkgwr/MyMsg)

[English](./README.md) | [日本語](./README_JA.md)

</div>

---

## 📌 Overview

`MyMsg` is a high-visibility, command-line-driven popup message notification utility engineered in Rust with `egui/eframe`. It operates with minimal CPU and memory consumption (~15–30 MB), making it ideal for low-spec PCs, virtual machines, background automation scripts, build pipelines, and periodic reminders.

With instant start-up, native window pinning, and event-driven rendering (0% CPU at idle), **`MyMsg` delivers immediate notifications right in front of you with a single command.**

---

## ✨ Features

- 🪟 **Always on Top**: Window stays pinned above all other windows and full-screen applications.
- 🖥️ **Multi-Monitor Cursor Auto-Follow & Explicit Targeting**: Automatically opens on active cursor display (default), or explicitly target screens with `--monitor primary` or display index.
- ⚡ **Instant Dismissal & Auto-Timeout**: Dismiss immediately with `Esc` or `Enter` (Exit code 0), or automatically close after a duration with `--timeout <seconds>`.
- 📑 **Multi-Line & Auto Word Wrap**: Automatically wraps long messages and newline characters (`\n`) with full line centering.
- 💡 **Status Icons (`--icon` / `-i`)**: Display status symbols for `info` (ℹ), `warn` (⚠), `error` (✖), and `ok` (✔).
- 🌗 **Theme Presets (`--theme` / `-t`)**: `system` (automatic OS light/dark detection - default), `dark`, and `light` modes.
- 🎨 **Flexible & Tolerant Color Parser**:
  - Web standard color names (`red`, `green`, `blue`, `gold`, `crimson`, `navy`, etc. across 24+ colors)
  - Japanese color names (`赤`, `青`, `緑`, `黄`, `白`, `黒`)
  - 1-character shorthands (`r`, `g`, `b`, `y`, `w`, `k`, `c`, `m`, `o`, `p`)
  - Typo tolerance (`bule` -> Blue)
  - Hex codes (`#RGB`, `#RRGGBB`, `#RRGGBBAA`, optional leading `#`)
- ⏱️ **Zero-Overhead Timer / Delay & Time-of-Day Mode (`--delay` / `-d`)**: Supports seconds (`60`), unit duration (`10m`, `1h`), or exact time of day (`12:00`) before displaying notification (zero GUI resource consumption during sleep mode; safety-capped at 24h / 86400s).
- 🍞 **OS Native Toast Notification Mode (`--toast` / `-T`)**: Dispatches a standard OS desktop banner notification (Action Center / Notification Center) and exits immediately without spawning a GUI window.
- 🚨 **Blink Mode (`--blink` / `-b`)**: Pulses text opacity every ~0.5s for urgent alerts.
- 🔤 **Automatic CJK / Japanese Font Detection**: Automatically discovers and registers OS Japanese fonts (Windows/macOS/Linux) to prevent tofu (□) or mojibake.
- 📦 **Self-Contained Executable**: Single zero-dependency native binary.

---

## 🚀 Installation & Build

### Prerequisites
- Rust 1.80 or later (Cargo)

### Local Build
```bash
git clone https://github.com/tkshnkgwr/MyMsg.git
cd MyMsg
cargo build --release
```
The compiled binary will be located at `target/release/MyMsg.exe` (or `MyMsg` on Unix-like platforms).

---

## 📋 CLI Options Reference

```
Usage: MyMsg.exe [OPTIONS] [MESSAGE]
```

| Argument / Option | Short | Default | Description |
| :--- | :---: | :---: | :--- |
| `[MESSAGE]` | - | None | Message string to display (Positional argument) |
| `--message <STR>` | `-m` | None | Message string to display (Optional argument) |
| `--size <SIZE>` | `-s` | `medium` | Window size preset (`small`: 300x150, `medium`: 450x220, `large`: 650x350) |
| `--font-size <PT>` | - | Auto | Font size in points (overrides size preset font size) |
| `--color <COLOR>` | `-c` | Default theme color | Text color (named, 1-char shorthand, typo-tolerant, #HEX) |
| `--bg-color <COLOR>` | - | Default theme color | Window background color |
| `--blink` | `-b` | `false` | Enable text blink pulsing animation |
| `--font <FONT>` | `-f` | `default` | Font family type (`default`/`sans`, `mono`/`2`, `serif`/`3`, `impact`) |
| `--icon <ICON>` | `-i` | None | Icon symbol type (`info`, `warn`, `error`, `ok`) |
| `--theme <THEME>` | `-t` | `system` | Theme selection (`system`, `dark`, `light`) |
| `--delay <SPEC>` | `-d` | `0` | Delay duration or time of day (`60`, `10m`, `12:00`, max 24h) |
| `--monitor <TARGET>` | - | `cursor` | Target monitor (`cursor`, `primary`, `0`, `1`, `2`...) |
| `--timeout <SEC>` | - | `0` | Auto-dismiss timer in seconds (0 to disable) |
| `--toast` | `-T` | `false` | OS native toast notification mode (no GUI window, immediate exit) |
| `--help` | `-h` | - | Display help message and exit |
| `--version` | `-V` | - | Display version information and exit |

> [!NOTE]
> Message string resolution follows strict priority: Positional argument (`MESSAGE`) > Option flag (`-m / --message`) > Default fallback (`"MyMsg: 通知が届きました"`).

---

## 💡 Usage Examples

### Basic Notifications & Icons
```powershell
# Notification with OK icon
MyMsg "Build completed successfully!" -i ok

# Warning alert with blinking
MyMsg "Memory usage nearing limit" -i warn -b
```

### Themes & Multi-line Text
```powershell
# Multi-line message with newline expansion
MyMsg "Deployment Summary:\n- Passed: 25\n- Skipped: 1\n- Failed: 0" -i info

# Light mode notification
MyMsg "Team standup starting now" -t light -i info
```

### Color & Styling Customization
```powershell
# Urgent alert in large red text
MyMsg "Critical error encountered" -c red -s large -i error

# Shorthand color and custom background
MyMsg "Server online" -c g --bg-color "#0f172a"

# Hex color and custom font point size
MyMsg "Operation done" -c "#00E5FF" --font-size 32
```

### Timed Reminders
```powershell
# Reminder in 5 minutes (300 seconds; 0% CPU during wait)
MyMsg "Time for the meeting" -d 300 -c gold -i info
```

### Shell & Script Integration
```powershell
# Notify on build script completion
npm run build; MyMsg "npm build complete!" -c cyan -i ok
```

---

## 📚 Documentation

Complete specifications and architectural design documents are maintained in the `docs/` directory:

| Japanese (docs/ja/) | English (docs/en/) | Description |
| :--- | :--- | :--- |
| [Specification](docs/ja/SPECIFICATION.md) | [Specification](docs/en/SPECIFICATION.md) | Full CLI options, UI behaviors, and exit triggers |
| [Architecture](docs/ja/ARCHITECTURE.md) | [Architecture](docs/en/ARCHITECTURE.md) | Internal module design, state flow, and rendering loop |
| [User Guide](docs/ja/USER_GUIDE.md) | [User Guide](docs/en/USER_GUIDE.md) | Practical command cookbook and script recipes |
| [Development](docs/ja/DEVELOPMENT.md) | [Development](docs/en/DEVELOPMENT.md) | Environment setup, building, and unit tests |
| [Release Guide](docs/ja/RELEASE.md) | [Release Guide](docs/en/RELEASE.md) | Release builds, binary size optimization, and distribution |
| [AI Instructions](docs/ja/INSTRUCTIONS.md) | [AI Instructions](docs/en/INSTRUCTIONS.md) | AI agent coding rules, prompts, and guidelines |
| [Testing Plan](docs/ja/TESTING.md) | [Testing Plan](docs/en/TESTING.md) | Test matrix, unit tests, and manual verification |
| [Test Report](docs/ja/TEST_REPORT.md) | [Test Report](docs/en/TEST_REPORT.md) | Initial verification report and evidence |
| [TODO Roadmap](docs/ja/TODO.md) | [TODO Roadmap](docs/en/TODO.md) | Completed milestones and upcoming features |
| [Footprints](docs/ja/FOOTPRINTS.md) | [Footprints](docs/en/FOOTPRINTS.md) | RAM, CPU, startup time, and binary size benchmarks |
| [Changelog](docs/ja/CHANGELOG.md) | [Changelog](docs/en/CHANGELOG.md) | Detailed version history |
| [Security Policy](docs/ja/SECURITY.md) | [Security Policy](docs/en/SECURITY.md) | Security model, input validation, and reporting |
| [Contributing](docs/ja/CONTRIBUTING.md) | [Contributing](docs/en/CONTRIBUTING.md) | Issue/PR workflow, branching, and commit conventions |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
