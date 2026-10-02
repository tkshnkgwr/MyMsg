# MyMsg Test Specification (TESTING)

This document defines the automated unit test suite, CLI argument testing strategy, and manual UI verification checklist for `MyMsg`.

---

## 1. Automated Unit Test Suite

The following 20 test cases are implemented across `src/app.rs`, `src/cli/`, `src/color.rs`, `src/log.rs`, `src/session.rs`, and `src/sound.rs`:

| Test Name                              | Target Function                | Verification Scope                                                         | Expected Behavior                              |
| :------------------------------------- | :----------------------------- | :------------------------------------------------------------------------- | :--------------------------------------------- |
| `test_resolve_message_priority`        | `resolve_message`              | Hierarchy of positional vs optional `-m` vs default                        | Positional > `-m` flag > Fallback default      |
| `test_resolve_message_newlines`        | `resolve_message`              | Expansion of `\n` and `\r\n` literals                                      | Converted to actual newline breaks             |
| `test_calculate_window_dimensions`     | `calculate_window_dimensions`  | Window dimensions and font sizes for `small`, `medium`, `large` & custom   | Correct (w, h) and font_size returned          |
| `test_clamp_delay_seconds`             | `clamp_delay_seconds`          | Delay boundary clamping (0s, 30s, 86400s, 99999s)                          | Clamped cleanly within 0–86400s (24h)          |
| `test_parse_delay_with_reference`      | `parse_delay_with_reference`   | Seconds, units (`10s`, `5m`, `2h`), exact time (`11:00`), next-day rollover| Exact delay seconds calculated correctly       |
| `test_parse_monitor_target`            | `parse_monitor_target`         | `cursor`, `primary`, `0`, `1`, `2` keywords & indices                      | Correct `MonitorTarget` enum returned          |
| `test_parse_icon`                      | `parse_icon`                   | `info`, `warn`, `error`, `ok` and shorthands                               | Correct `IconType` returned                    |
| `test_parse_theme`                     | `parse_theme`                  | `dark`, `light`, `system` and aliases                                      | Correct `ThemeMode` returned                   |
| `test_parse_color_named_and_typo`      | `parse_color`                  | Named colors (red/green), Japanese names, typos (bule), shorthands         | Correct `Color32` RGB value returned           |
| `test_parse_color_hex`                 | `parse_color`                  | 6-digit HEX, 3-digit HEX, 8-digit RGBA, invalid strings                    | Accurate `Color32` or `None` returned          |
| `test_resolve_theme_palette`           | `resolve_theme_palette`        | OS theme detection and custom color resolution hierarchy                   | Correct `ThemePalette` assembled               |
| `test_parse_duration_and_interval`     | `parse_interval_to_seconds`    | Interval durations (`30m`), 0s, and invalid tokens                         | Parsed duration seconds or `None`              |
| `test_parse_at_time`                   | `parse_at_time`                | Single time parsing, rejection of comma-separated multiple times           | Correct `NaiveTime` or `None`                  |
| `test_calculate_at_wait`               | `calculate_at_wait`            | Future wait duration vs past time next-day rollover                        | Accurate seconds until scheduled time          |
| `test_has_next_schedule`               | `MyMsgApp::has_next_schedule`  | Next wait detection for single, `--at`, `--interval`, and count max        | Correct boolean indicating future schedules    |
| `test_append_log_flow`                 | `log::append_log`              | `OPEN`, `CLOSE`, `COPY`, `ACTION` events and newline escape handling       | Properly formatted log entry append            |
| `test_cli_args_parsing_new_features`   | `CliArgs::try_parse_from`      | `--show-progress`, `--log`, `--copy`, `--action`, `--sound`, `--at` flags  | Correctly parsed and mapped CliArgs struct     |
| `test_cli_args_parsing_recurring`      | `CliArgs::try_parse_from`      | `--interval`, `--count`, `--immediate` recurring flags                     | Correctly parsed and mapped CliArgs struct     |
| `test_is_interactive_session_callable` | `session::is_interactive_session` | Window station & desktop accessibility check                            | Callable without panic, returns boolean        |
| `test_play_sound_does_not_panic`       | `sound::play_notification_sound` | System notification audio playback verification                           | Safe execution across platforms without panic  |

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
- [ ] Passing comma-separated multiple times to `--at` (e.g. `--at 09:00,12:00`) outputs an error and exits with code 1.
- [ ] Invalid flags output usage errors and exit with non-zero code.

### 2.2 GUI Rendering & Interaction
- [ ] `MyMsg.exe "Test Notification"` displays an Always-on-Top popup centered on screen.
- [ ] For single runs, pressing `Esc` or `Enter` immediately closes the window and terminates process.
- [ ] For single runs, clicking `[✕ 閉じる (Esc / Enter)]` closes the window and terminates process.
- [ ] For recurring runs (`--interval 10s`), pressing `Enter` or clicking `[✓ 今回閉じる]` dismisses popup and waits for next interval.
- [ ] For recurring runs (`--interval 10s`), pressing `Esc` or clicking `[✕ 中止]` immediately aborts the schedule and terminates process.
- [ ] Japanese / CJK characters render clearly without box glyphs (□).

### 2.3 Feature Options
- [ ] `--monitor primary` centers the popup on the primary screen regardless of mouse position.
- [ ] `--timeout 3` automatically closes the popup after 3 seconds.
- [ ] `--toast` sends an OS desktop toast notification without creating a GUI window.
- [ ] `-d 10m` or `-d 12:00` delays notification until the specified duration/time.
- [ ] `-b` pulses text opacity at ~0.5s intervals.
