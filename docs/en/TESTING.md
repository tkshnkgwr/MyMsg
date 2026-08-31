# MyMsg Test Specification (TESTING)

This document defines the automated unit test suite, CLI argument testing strategy, and manual UI verification checklist for `MyMsg`.

---

## 1. Automated Unit Test Suite

The following 11 test cases are implemented across `src/cli.rs` and `src/color.rs`:

| Test Name | Target Function | Verification Scope | Expected Behavior |
| :--- | :--- | :--- | :--- |
| `test_resolve_message_priority` | `resolve_message` | Hierarchy of positional vs optional `-m` vs default | Positional > `-m` flag > Fallback default string |
| `test_resolve_message_newlines` | `resolve_message` | Expansion of `\n` and `\r\n` literals | Converted to actual newline breaks |
| `test_calculate_window_dimensions` | `calculate_window_dimensions` | Window dimensions and font sizes for `small`, `medium`, `large` & custom sizes | Correct (w, h) and font_size returned |
| `test_clamp_delay_seconds` | `clamp_delay_seconds` | Delay boundary clamping (0s, 30s, 86400s, 99999s) | Clamped cleanly within 0–86400s (24h) |
| `test_parse_delay_with_reference` | `parse_delay_with_reference` | Seconds, units (`10s`, `5m`, `2h`, `10分`, `1時間`), exact time (`11:00`), seconds (`10:50:30`), next-day rollover (`10:00`) | Exact delay seconds calculated correctly |
| `test_parse_monitor_target` | `parse_monitor_target` | `cursor`, `primary`, `0`, `1`, `2` keywords & indices | Correct `MonitorTarget` enum returned |
| `test_parse_icon` | `parse_icon` | `info`, `warn`, `error`, `ok` and shorthands | Correct `IconType` returned |
| `test_parse_theme` | `parse_theme` | `dark`, `light`, `system` and aliases | Correct `ThemeMode` returned |
| `test_parse_color_named_and_typo` | `parse_color` | Named colors (red/green), Japanese names, typos (bule), shorthands (r/g/b/y/w/k) | Correct `Color32` RGB value returned |
| `test_parse_color_hex` | `parse_color` | 6-digit HEX, 3-digit HEX, 8-digit RGBA, invalid strings | Accurate `Color32` or `None` returned |
| `test_resolve_theme_palette` | `resolve_theme_palette` | OS theme detection and custom color resolution hierarchy | Correct `ThemePalette` assembled |

### Running the Test Suite
```bash
cargo test
```

---

## 2. Manual Verification Checklist

### 2.1 CLI Argument Verification
- [ ] `MyMsg.exe --help` outputs formatted help text with time/unit examples.
- [ ] `MyMsg.exe -h` outputs concise summary help.
- [ ] `MyMsg.exe --version` outputs version information.
- [ ] Invalid flags output usage errors and exit with non-zero code.

### 2.2 GUI Rendering & Interaction
- [ ] `MyMsg.exe "Test Notification"` displays an Always-on-Top popup centered on screen.
- [ ] Pressing `Esc` or `Enter` immediately closes the window.
- [ ] Clicking `[✕ 閉じる (Esc / Enter)]` closes the window.
- [ ] Japanese / CJK characters render clearly without box glyphs (□).

### 2.3 Feature Options
- [ ] `--monitor primary` centers the popup on the primary screen regardless of mouse position.
- [ ] `--timeout 3` automatically closes the popup after 3 seconds.
- [ ] `--toast` sends an OS desktop toast notification without creating a GUI window.
- [ ] `-d 10m` or `-d 12:00` delays notification until the specified duration/time.
- [ ] `-b` pulses text opacity at ~0.5s intervals.
