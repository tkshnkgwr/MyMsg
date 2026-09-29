# MyMsg Development Task List (TODO)

This document tracks completed implementation milestones and the future enhancement roadmap for `MyMsg`.

---

## 1. Completed Features

- [x] **CLI Engine**: `clap` v4 argument, flag, and help/version parsing.
- [x] **Always-on-Top GUI**: `eframe` / `egui` native pinned window.
- [x] **Instant Keyboard Dismissal**: Immediate termination on `Esc` / `Enter`.
- [x] **Tolerant Color Parser**: Named colors, Japanese names, typo correction, shorthands, and HEX.
- [x] **Preset & Custom Dimensions**: `small`, `medium`, `large` presets with `--font-size` override.
- [x] **Blink Animation**: `--blink` opacity pulsing for urgent alerts.
- [x] **Zero-Overhead Delay Timer & Time-of-Day (`--delay <sec|HH:MM|unit>`)**: Seconds, unit duration (`10m`, `1h`), and exact time of day (`12:00`) with auto difference calculation and 24h clamp.
- [x] **Multi-line Word Wrap**: Automatic text wrapping and horizontal/vertical centering for multi-line and long messages.
- [x] **Icon Support (`--icon <info|warn|error|ok>`)**: Display vector status icons next to text.
- [x] **Theme Selection (`--theme <system|dark|light>`)**: Cohesive palettes with automatic OS dark/light mode detection.
- [x] **Multi-Monitor Active Display Targeting**: Automatically places the popup in the center of the active monitor where the mouse cursor resides (Windows native coordinates).
- [x] **Explicit Monitor Selection (`--monitor <cursor|primary|0|1...>`)**: Explicitly target screens by keyword (`primary`) or display index.
- [x] **Auto-Dismiss Timer (`--timeout <seconds>`)**: Automatically close popup after a given duration without keypress.
- [x] **OS Native Toast Notification Mode (`--toast` / `-T`)**: Dispatches desktop notifications to OS notification center without spawning a GUI window.
- [x] **Interval / Recurring Notifications (`--interval <duration>` / `--every <duration>`)**: Repeat/cycle notifications at fixed intervals (e.g. `30m`, `1h`) with `--count` limit and `--immediate` skip.
- [x] **Multiple Scheduled Times (`--at <HH:MM,...>` / `--schedule`)**: Specify multiple exact times of day (comma-separated or multiple flags) to trigger messages sequentially.
- [x] **Timeout Progress & Countdown (`--show-progress`)**: Render an animated visual countdown progress bar and remaining seconds for auto-dismissing timeouts.
- [x] **Clipboard Copy / Custom Action Buttons (`--copy`, `--action <cmd>`)**: Actionable buttons inside the popup to copy text to clipboard (with feedback) and execute external commands.
- [x] **Logging to File (`--log <file>`)**: Append invocation timestamps, popup counts, dismiss reasons, and action execution logs to a specified file.
- [x] **Headless / Non-Interactive Session (Session 0) Detection**: Automatically detect non-interactive environments and fallback to OS toast/logging with warnings.
- [x] **Active Desktop (`Default`) Strict Verification & Sandbox Isolation Detection**: Verify thread desktop attachment to detect sandbox environments and safely fallback to OS toast notifications.
- [x] **Auto-Dismiss Console Window on Standalone Execution (`auto_detach_console_if_standalone`)**: Automatically detects standalone invocations and calls `FreeConsole()` to dismiss DOS windows.
- [x] **Secondary Fail-Safe for GUI Window Failures**: Automatically dispatches an OS toast notification when window context creation fails.
- [x] **Notification Sound / Chime Option (`--sound` / `--beep`)**: Play system alerts, chimes, or beeps when popup or toast notification triggers.
- [x] **CJK / Japanese Font Detection**: OS font auto-discovery to prevent mojibake.
- [x] **Automated Test Suite**: Unit tests covering parsing, time calculations, theme resolution, logging, session detection, sound, and math.

---

## 2. Planned Roadmap

### Phase 2: Display, Placement & Process Control
- [ ] **Parent Process / Terminal Attachment (`--attach-parent`)**: Position the popup relative to the calling terminal / console window HWND.
- [ ] **Singleton / Instance Replacement (`--single-instance` / `--replace`)**: Close existing popups or prevent duplicate instances when triggered periodically.
- [ ] **Cross-Platform Multi-Monitor Support**: Active monitor detection on macOS (CoreGraphics) and Linux (X11 / Wayland).

### Phase 3: Audio & Alerts
- [ ] **Custom Audio File Playback (`--sound-file <path>`)**: Directly play custom WAV / audio files on notification.

### Phase 4: Interaction & Styling
- [ ] **Window Transparency (`--transparent`)**: Experimental support for frosted glass or alpha-blended windows.
- [ ] **Custom Emoji/Icon String**: Direct support for custom emoji symbols in icon flag.

### Phase 5: CI/CD & Distribution
- [ ] **GitHub Actions Matrix**: Automated cross-compilation for Windows, macOS, and Linux.
- [ ] **Package Manager Distribution**: Submissions to `winget`, `Scoop`, and `Homebrew`.


