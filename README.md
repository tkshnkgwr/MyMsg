<div align="center">

# ⚡ MyMsg

**Ultra-lightweight, Always-on-Top popup notification CLI tool optimized for resource-constrained environments.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021/2024](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-v1.4.0-brightgreen.svg)](Cargo.toml)
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
- 🔄 **Interval & Scheduled Notifications (`--interval` / `--at`)**: Recurring notifications at fixed intervals (e.g., `30m`) or designated time of day (`15:00`) with `--count` and `--immediate` flags.
- ⏳ **Timeout Progress Bar (`--show-progress`)**: Smooth countdown progress bar displayed in the bottom panel when `--timeout` is set.
- 📋 **Clipboard Copy & Command Execution (`--copy`, `--action <cmd>`)**: One-click action buttons to copy message text or trigger external commands in the background.
- 📝 **Event Logging (`--log <file>`)**: Automatically append popup invocation timestamps, counts, dismiss reasons, and user actions to a log file.
- 🍞 **OS Native Toast Notification Mode (`--toast` / `-T`)**: Dispatches a standard OS desktop banner notification (Action Center / Notification Center) and exits immediately without spawning a GUI window.
- 🚨 **Blink Mode (`--blink` / `-b`)**: Pulses text opacity every ~0.5s for urgent alerts.
- 🔤 **Automatic CJK / Japanese Font Detection**: Automatically discovers and registers OS Japanese fonts (Windows/macOS/Linux) to prevent tofu (□) or mojibake.
- 🔔 **System Notification Sound (`--sound` / `--beep`)**: Plays the OS system chime or beep sound (`MessageBeep` on Windows) on popup display and toast dispatch, dynamically mapping tone to `--icon`.
- 🛡️ **Headless / Non-Interactive (Session 0) Auto-Detection**: Automatically detects non-interactive sessions (Windows services, background schedulers without desktop), issues a warning, and safely falls back to OS toast notifications.
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

| Argument / Option    | Short | Default             | Description                                                               |
| :------------------- | :---: | :-----------------: | :------------------------------------------------------------------------ |
| `[MESSAGE]`          | -     | None                | Message string to display (Positional argument)                           |
| `--message <STR>`    | `-m`  | None                | Message string to display (Optional argument)                             |
| `--size <SIZE>`      | `-s`  | `medium`            | Window size preset (`small`: 300x150, `medium`: 450x220, `large`: 650x350)|
| `--font-size <PT>`   | -     | Auto                | Font size in points (overrides size preset font size)                     |
| `--color <COLOR>`    | `-c`  | Default theme color | Text color (named, 1-char shorthand, typo-tolerant, #HEX)                 |
| `--bg-color <COLOR>` | -     | Default theme color | Window background color                                                   |
| `--blink`            | `-b`  | `false`             | Enable text blink pulsing animation                                       |
| `--font <FONT>`      | `-f`  | `default`           | Font family type (`default`/`sans`, `mono`/`2`, `serif`/`3`, `impact`)     |
| `--icon <ICON>`      | `-i`  | None                | Icon symbol type (`info`, `warn`, `error`, `ok`)                          |
| `--theme <THEME>`    | `-t`  | `system`            | Theme selection (`system`, `dark`, `light`)                               |
| `--delay <SPEC>`     | `-d`  | `0`                 | Sleep delay duration or exact target time (`60`, `10m`, `12:00`, max 24h) |
| `--monitor <TARGET>` | -     | `cursor`            | Target monitor screen (`cursor`, `primary`, `0`, `1`, `2`...)             |
| `--timeout <SEC>`    | -     | `0`                 | Auto-dismiss timer in seconds (0 = disabled, manual close)                |
| `--toast`            | `-T`  | `false`             | OS native desktop toast notification mode (no GUI window, instant exit)   |
| `--interval <INT>`   | -     | None                | Periodic repeat interval (`30m`, `1h`, `300`). Alias `--every`            |
| `--at <TIME>`        | -     | None                | Scheduled time of day (`15:00`, single only). Alias `--schedule`          |
| `--count <NUM>`      | -     | `0`                 | Maximum notification iterations (0 = infinite loop). Alias `--times`      |
| `--immediate`        | -     | `false`             | Show 1st notification immediately without initial wait during repeats     |
| `--show-progress`    | -     | `false`             | Display animated progress bar during auto-timeout. Alias `--progress`     |
| `--log <FILE>`       | -     | None                | Append popup lifecycle events and button clicks to specified log file     |
| `--copy`             | -     | `false`             | Display "📋 Copy" button to copy message string to clipboard              |
| `--action <CMD>`     | -     | None                | Display "⚡ Run" button to execute external command asynchronously         |
| `--sound`            | -     | `false`             | Play OS system notification chime / beep sound. Alias `--beep`            |
| `--help`             | `-h`  | -                   | Print help information and exit                                           |
| `--version`          | `-V`  | -                   | Print version information and exit                                        |

> [!NOTE]
> Message string resolution priority: Positional argument `[MESSAGE]` > `-m / --message` > Default message (`"MyMsg: 通知が届きました"`).

---

## 💡 Examples

### Basic Popup & Status Icons
```powershell
# Success notification with checkmark icon
MyMsg "All builds completed successfully!" -i ok

# Warning alert with blinking animation
MyMsg "High memory usage detected" -i warn -b
```

### Multi-Line Formatting & Themes
```powershell
# Multi-line message with newlines (auto word-wrapping)
MyMsg "Job Summary:\nSuccess: 25\nWarnings: 1\nFailures: 0" -i info

# Light mode theme
MyMsg "Meeting starts in 5 minutes" -t light -i info
```

### Color & Styling Customization
```powershell
# Red warning text in large size
MyMsg "Critical Service Alert" -c red -s large -i error

# Shorthand color and custom dark background
MyMsg "Server Online" -c g --bg-color "#0f172a"

# Hex color and custom font size
MyMsg "Processing Finished" -c "#00E5FF" --font-size 32
```

### Delay & Time-of-Day Reminders
```powershell
# Reminder after 5 minutes (300 seconds; zero CPU/RAM while sleeping)
MyMsg "Time for standup meeting" -d 300 -c gold -i info
```

### Pipeline & PowerShell Automation
```powershell
# Notify upon completion of a build script
npm run build; MyMsg "npm build complete!" -c cyan -i ok
```

### Task Scheduler / Scheduled Execution (`--timeout` Recommended)
```powershell
# Auto-dismiss after 15 seconds to prevent blocking unattended jobs
MyMsg "Scheduled backup finished" -i ok -c green --timeout 15

# Auto-dismiss with progress bar countdown
MyMsg "Sync complete" --timeout 5 --show-progress -i ok

# Send to OS Notification Center instead of GUI popup
MyMsg "Daily stretch reminder" -i info --toast
```

### Interval & Scheduled Notifications
```powershell
# Repeat notification every 30 minutes (3 times maximum, immediate 1st run)
# Note: Press Enter (or [✓ 今回閉じる]) for next wait, Esc (or [✕ 中止]) to abort and exit
MyMsg "Stay hydrated!" --interval 30m --immediate --count 3 -i info

# Trigger notification at an exact time of day (single run, exits cleanly on Esc/Enter)
MyMsg "Team sync" --at 15:00 -i warn
```

### Clipboard Copy, Command Action & Audit Logging
```powershell
# Display copy button to quickly copy tokens or URLs
MyMsg "TOKEN=abc123xyz456" --copy -i info

# Action button to open log file, appending events to audit log
MyMsg "Build failed!" -i error --action "notepad.exe %USERPROFILE%\Logs\build.log" --log "%USERPROFILE%\Logs\mymsg.log"
```

### System Notification Sound (Chime / Beep)
```powershell
# Play warning beep alongside warning icon
MyMsg "Fatal connection timeout!" -i warn --sound

# Play chime sound on toast notification (alias --beep)
MyMsg "All operations succeeded" --toast -i ok --beep
```

---

## 📚 Documentation Index

Comprehensive specifications and technical design docs are located in the `docs/` directory:

| Japanese (docs/ja/)                    | English (docs/en/)                       | Description                                      |
| :------------------------------------- | :--------------------------------------- | :----------------------------------------------- |
| [詳細仕様書](docs/ja/SPECIFICATION.md) | [Specification](docs/en/SPECIFICATION.md)| Complete CLI arguments, UI, and lifecycle specs  |
| [内部アーキテクチャ](docs/ja/ARCHITECTURE.md) | [Architecture](docs/en/ARCHITECTURE.md) | Module layout, data flow, and render loop        |
| [ユーザーガイド](docs/ja/USER_GUIDE.md)| [User Guide](docs/en/USER_GUIDE.md)      | Practical recipes, shell integrations, and tips  |
| [開発ガイド](docs/ja/DEVELOPMENT.md)   | [Development](docs/en/DEVELOPMENT.md)    | Build environment setup and unit tests           |
| [リリース手順](docs/ja/RELEASE.md)     | [Release Guide](docs/en/RELEASE.md)      | Optimization flags and distribution instructions |
| [AI開発指示書](docs/ja/INSTRUCTIONS.md) | [AI Instructions](docs/en/INSTRUCTIONS.md) | Guidelines for automated AI agent contributions |
| [テスト仕様書](docs/ja/TESTING.md)     | [Testing](docs/en/TESTING.md)            | Unit test suites and verification matrices       |
| [テスト実行報告書](docs/ja/TEST_REPORT.md) | [Test Report](docs/en/TEST_REPORT.md)  | Initial version verification report              |
| [開発タスクリスト](docs/ja/TODO.md)    | [TODO](docs/en/TODO.md)                  | Implemented features and forward roadmap         |
| [リソース指標](docs/ja/FOOTPRINTS.md)  | [Footprints](docs/en/FOOTPRINTS.md)      | Memory, startup time, and CPU benchmarks         |
| [変更履歴](docs/ja/CHANGELOG.md)       | [Changelog](docs/en/CHANGELOG.md)        | Version history and changelog                    |
| [セキュリティ方針](docs/ja/SECURITY.md)| [Security](docs/en/SECURITY.md)          | Security policy and vulnerability disclosure     |
| [コントリビューション](docs/ja/CONTRIBUTING.md) | [Contributing](docs/en/CONTRIBUTING.md) | Contributing guidelines and PR workflow          |

---

## 📄 License

This project is licensed under the MIT License - Copyright (c) 2026 tkshnkgwr. See the [LICENSE](LICENSE) file for details.
