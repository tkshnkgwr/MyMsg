# MyMsg Detailed Changelog (CHANGELOG)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Changed & Improved
- **Documentation Overhaul**:
  - Added Windows Task Scheduler and cron automation integration guide (`USER_GUIDE.md`, `README.md`, `README_JA.md`).
  - Added Windows Session 0 Isolation security constraints and `--timeout` best practices (`SPECIFICATION.md`).
  - Added upcoming roadmap items for instance replacement (`--single-instance`), headless session detection, and timeout progress bars (`TODO.md`).
  - Updated resource benchmark footprints in `FOOTPRINTS.md` (measured release binary size **~5.2 MB**, added OS toast mode latency/RAM metrics).


## [1.0.0] - 2026-08-31


### Added
- **Explicit Monitor Placement (`--monitor <cursor|primary|0|1...>`)**:
  - Targets screens by cursor position (default), primary display (`primary`), or enumerated display index using Win32 `EnumDisplayMonitors`.
- **Auto-Dismissal Timer (`--timeout <seconds>`)**:
  - Automatically closes popup and exits cleanly after specified seconds (default `0` keeps window open until manual dismissal).
- **OS Native Toast Notification Mode (`--toast` / `-T`)**:
  - Dispatches standard OS desktop toast notifications directly via `notify-rust` without creating a GUI window.
- **Time-of-Day (`HH:MM` / `HH:MM:SS`) & Duration Unit (`10m`, `1h`, `10分`) Delay Support**:
  - Automatic difference calculation from current local time using `chrono`, with 24-hour rollover support for past times.
- **Extended Delay Cap (24 Hours / 86,400 Seconds)**:
  - Increased safety clamp from 1 hour to 24 hours.
- **Enhanced CLI Help (`-h` / `--help`)**:
  - Comprehensive usage examples for time formats and options.
- **Expanded Automated Test Suite**:
  - Unit tests for time calculation, rollover, monitor targeting, and delay parsing (11 total tests).

---

## [0.1.0] - 2026-08-28

### Added
- **Multi-Monitor Active Display Auto-Detection**:
  - Automatically identifies the monitor containing the mouse cursor via Windows native APIs (`GetCursorPos`, `MonitorFromPoint`, `GetMonitorInfoW`) and centers the popup within its usable working area (`rcWork`).
  - Native integration with `eframe` (`centered: false`) using physical pixel coordinate calculation to prevent override issues.
- **Multi-Line Text Wrapping & Centering Optimization**:
  - Automatic line wrapping via `egui::ScrollArea` and label wrapping with full horizontal and vertical line centering.
  - Expansion of `\n` and `\r\n` literals in CLI arguments to actual multi-line breaks.
- **Icon Display Option (`-i` / `--icon`)**:
  - Notification symbols for `info` (ℹ), `warn` (⚠), `error` (✖), and `ok` (✔) with distinctive highlight colors.
- **Theme Selection (`-t` / `--theme`)**:
  - `system` (automatic OS light/dark theme tracking - default), `dark`, and `light` presets with cohesive palette resolution.
- **Core CLI Engine**: Powered by `clap` v4 with derive macros, supporting positional message arguments and optional `-m` / `--message` flags.
- **Always-on-Top GUI Notification**: Window rendered using `eframe` / `egui` with native `with_always_on_top()` flag, pinned on top of all windows and full-screen applications.
- **Instant Keyboard & Button Dismissal**: Immediate application exit on pressing `Esc` or `Enter`, or clicking the bottom action bar button (`✕ 閉じる (Esc / Enter)`).
- **Flexible & Tolerant Color Parser (`parse_color`)**:
  - Support for 24+ standard color names (e.g., `red`, `green`, `blue`, `gold`, `crimson`, `navy`).
  - Japanese color names (e.g., `赤`, `青`, `緑`, `黄`, `白`, `黒`).
  - Common typo auto-correction (e.g., `bule` -> Blue).
  - 1-character color abbreviations (`r`, `g`, `b`, `y`, `w`, `k`, `c`, `m`, `o`, `p`).
  - Hex color code resolution (`#RGB`, `#RRGGBB`, `#RRGGBBAA`, with optional leading `#`).
- **Window Size Presets & Custom Font Size**:
  - Size presets: `small` (300x150, 20pt), `medium` (450x220, 26pt, default), and `large` (650x350, 36pt).
  - Explicit font size override via `--font-size <pt>`.
- **Urgent Notification Blink Mode (`-b` / `--blink`)**:
  - Text opacity pulsing effect on a ~0.5s cycle for high-priority alerts.
- **Zero-Resource Timer / Delay Mode (`-d` / `--delay`)**:
  - Safe delayed popups (0 to 3600 seconds / max 1 hour) with zero GUI overhead during wait periods via `clamp_delay_seconds`.
- **Automated CJK / Japanese Font Fallback (`setup_japanese_fonts`)**:
  - Automatic detection and registration of Windows system fonts (`meiryo.ttc`, `YuGothM.ttc`, `msgothic.ttc`, `msmincho.ttc`), macOS fonts, and Linux Noto fonts into `egui::FontDefinitions`.
- **Comprehensive Unit Test Suite**:
  - Unit tests covering color parsing, argument priority resolution, delay clamping, and window dimension calculation.
