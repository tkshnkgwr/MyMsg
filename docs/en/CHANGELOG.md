# MyMsg Detailed Changelog (CHANGELOG)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-08-28

### Added
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
