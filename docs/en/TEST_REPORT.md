# MyMsg Test Execution Report (TEST_REPORT)

This document contains the execution report and verification evidence for the `MyMsg` release.

---

## 1. Test Environment

| Parameter            | Value                                |
| :------------------- | :----------------------------------- |
| **OS**               | Windows 11 (x86_64)                  |
| **Rust Version**     | 1.80+ / stable-x86_64-pc-windows-msvc|
| **Cargo Version**    | cargo 1.80+                          |
| **Target Version**   | v1.3.0+                              |
| **Execution Date**   | 2026-10-02                           |

---

## 2. Automated Unit Test Results

```
running 20 tests
test app::tests::test_has_next_schedule ... ok
test cli::layout::tests::test_calculate_window_dimensions ... ok
test cli::types::tests::test_parse_icon ... ok
test cli::layout::tests::test_resolve_message_priority ... ok
test cli::time::tests::test_calculate_at_wait ... ok
test cli::time::tests::test_clamp_delay_seconds ... ok
test cli::time::tests::test_parse_delay_with_reference ... ok
test cli::time::tests::test_parse_at_time ... ok
test cli::layout::tests::test_resolve_message_newlines ... ok
test cli::time::tests::test_parse_duration_and_interval ... ok
test cli::types::tests::test_parse_monitor_target ... ok
test cli::args::tests::test_cli_args_parsing_recurring ... ok
test cli::args::tests::test_cli_args_parsing_new_features ... ok
test cli::types::tests::test_parse_theme ... ok
test color::tests::test_parse_color_named_and_typo ... ok
test color::tests::test_resolve_theme_palette ... ok
test session::tests::test_is_interactive_session_callable ... ok
test sound::tests::test_play_sound_does_not_panic ... ok
test color::tests::test_parse_color_hex ... ok
test log::tests::test_append_log_flow ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

**Summary**: 20 of 20 tests passed (100% Pass Rate).

---

## 3. CLI & GUI Manual Verification Evidence

| Verification Target          | Command / Action Invoked              | Observed Outcome                                              | Status  |
| :--------------------------- | :------------------------------------ | :------------------------------------------------------------ | :-----: |
| **Detailed Help Display**    | `MyMsg.exe --help`                    | Formatted help text with `--interval` / `--at` details printed| ✅ PASS |
| **Short Help Display**       | `MyMsg.exe -h`                        | Outputs concise usage summary                                 | ✅ PASS |
| **Version Display**          | `MyMsg.exe --version`                 | Outputs current crate version cleanly                         | ✅ PASS |
| **`--at` Multi-Time Rejection**| `MyMsg.exe --at 09:00,12:00`        | Outputs error message and cleanly exits with exit code 1      | ✅ PASS |
| **Recurring Next Wait**      | Press `Enter` during `--interval 10s` | Dismisses popup and waits for next 10s interval cycle         | ✅ PASS |
| **Recurring Abort & Exit**   | Press `Esc` during `--interval 10s`   | Immediately aborts recurring schedule and exits process       | ✅ PASS |
| **CJK Font Rendering**       | `MyMsg.exe "日本語通知テスト"`        | Clean typography in Meiryo / Yu Gothic without mojibake       | ✅ PASS |
| **Auto-Dismissal Timer**     | `MyMsg.exe "3s auto" --timeout 3`     | Automatically closes window after 3 seconds                   | ✅ PASS |
| **OS Toast Notification**    | `MyMsg.exe "Toast" --toast -i ok`     | OS native toast banner appears without GUI initialization     | ✅ PASS |
| **Time-of-Day Delay**        | `MyMsg.exe "Lunch" -d 12:00`          | Auto-calculates difference, waits with 0% CPU, and triggers   | ✅ PASS |
| **Blink Animation**          | `MyMsg.exe "Alert" -b`                | Pulses opacity on precise 0.5s intervals                      | ✅ PASS |
| **Clipboard Copy**           | `MyMsg.exe "COPY_ME" --copy`          | One-click copy with immediate feedback                        | ✅ PASS |

---

## 4. Conclusion
All automated unit tests (20 of 20) and manual sanity checks passed with zero defects, verifying readiness for production distribution.
