# MyMsg Test Specification (TESTING)

This document defines the automated unit test suite, CLI argument testing strategy, and manual UI verification checklist for `MyMsg`.

---

## 1. Automated Unit Test Suite

The following test cases are implemented under `mod tests` in `src/main.rs`:

| Test Name | Target Function | Verification Scope | Expected Behavior |
| :--- | :--- | :--- | :--- |
| `test_resolve_message_priority` | `resolve_message` | Hierarchy of positional vs optional `-m` vs default | Positional > `-m` flag > Fallback default string |
| `test_calculate_window_dimensions` | `calculate_window_dimensions` | Window dimensions and font sizes for `small`, `medium`, `large` & custom sizes | Correct (w, h) and font_size returned |
| `test_clamp_delay_seconds` | `clamp_delay_seconds` | Delay boundary clamping (0s, 30s, 3600s, 9999s) | Clamped cleanly within 0–3600s |
| `test_parse_color_named_and_typo` | `parse_color` | Named colors (red/green), typos (bule), shorthands (r/g/b/y/w/k) | Correct `Color32` RGB value returned |
| `test_parse_color_hex` | `parse_color` | 6-digit HEX, 3-digit HEX, 8-digit RGBA, invalid strings | Accurate `Color32` or `None` returned |

### Running the Test Suite
```bash
cargo test
```

---

## 2. Manual Verification Checklist

### 2.1 CLI Argument Verification
- [ ] `MyMsg.exe --help` outputs formatted help text to console and exits cleanly.
- [ ] `MyMsg.exe --version` outputs version information.
- [ ] Invalid flags (e.g. `MyMsg.exe --invalid`) output usage errors and exit with code 2.

### 2.2 GUI Rendering & Interaction
- [ ] `MyMsg.exe "Test Notification"` displays an Always-on-Top popup centered on screen.
- [ ] Pressing `Esc` immediately closes the window.
- [ ] Pressing `Enter` immediately closes the window.
- [ ] Clicking the `[✕ 閉じる (Esc / Enter)]` button closes the window.
- [ ] Japanese / CJK characters render clearly without box glyphs (□).

### 2.3 Feature Options
- [ ] `-s small` and `-s large` scale dimensions and font size accordingly.
- [ ] `-c red` and `-c "#00FFCC"` render the expected colors.
- [ ] `-b` pulses text opacity at ~0.5s intervals.
- [ ] `-d 3` pauses for 3 seconds before displaying the window.
