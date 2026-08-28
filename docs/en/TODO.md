# MyMsg Development Task List (TODO)

This document tracks completed implementation milestones and the future enhancement roadmap for `MyMsg`.

---

## 1. Completed Features (v0.1.0)

- [x] **CLI Engine**: `clap` v4 argument, flag, and help/version parsing.
- [x] **Always-on-Top GUI**: `eframe` / `egui` native pinned window.
- [x] **Instant Keyboard Dismissal**: Immediate termination on `Esc` / `Enter`.
- [x] **Tolerant Color Parser**: Named colors, Japanese names, typo correction, shorthands, and HEX.
- [x] **Preset & Custom Dimensions**: `small`, `medium`, `large` presets with `--font-size` override.
- [x] **Blink Animation**: `--blink` opacity pulsing for urgent alerts.
- [x] **Zero-Overhead Delay Timer**: `--delay` with 0–3600s clamp and sleep mode.
- [x] **CJK / Japanese Font Detection**: OS font auto-discovery to prevent mojibake.
- [x] **Automated Test Suite**: Unit tests covering parsing and math calculations.

---

## 2. Planned Roadmap

### Phase 2: Audio & Timers
- [ ] **Notification Audio (`--sound` / `--beep`)**: Play optional system chime or beep on popup.
- [ ] **Auto-Dismiss Timer (`--timeout <seconds>`)**: Automatically close popup after a given timeout without keypress.

### Phase 3: GUI Enhancements
- [ ] **Multi-line Word Wrap**: Automatic text wrapping and scrollbars for long messages.
- [ ] **Icon Support (`--icon <info|warn|error|ok>`)**: Display vector status icons next to text.
- [ ] **Window Transparency (`--transparent`)**: Experimental support for frosted glass or alpha-blended windows.

### Phase 4: CI/CD & Distribution
- [ ] **GitHub Actions Matrix**: Automated cross-compilation for Windows, macOS, and Linux.
- [ ] **Package Manager Distribution**: Submissions to `winget`, `Scoop`, and `Homebrew`.
