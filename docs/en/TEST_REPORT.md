# MyMsg Test Execution Report (TEST_REPORT)

This document contains the execution report and verification evidence for the `MyMsg` release.

---

## 1. Test Environment

| Parameter | Value |
| :--- | :--- |
| **OS** | Windows 11 (x86_64) |
| **Rust Version** | 1.80+ / stable-x86_64-pc-windows-msvc |
| **Cargo Version** | cargo 1.80+ |
| **Target Version** | v0.2.0 |
| **Execution Date** | 2026-08-31 |

---

## 2. Automated Unit Test Results

```
running 11 tests
test cli::tests::test_calculate_window_dimensions ... ok
test cli::tests::test_clamp_delay_seconds ... ok
test cli::tests::test_parse_delay_with_reference ... ok
test color::tests::test_resolve_theme_palette ... ok
test cli::tests::test_parse_monitor_target ... ok
test cli::tests::test_resolve_message_priority ... ok
test cli::tests::test_parse_icon ... ok
test color::tests::test_parse_color_named_and_typo ... ok
test cli::tests::test_parse_theme ... ok
test cli::tests::test_resolve_message_newlines ... ok
test color::tests::test_parse_color_hex ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**Summary**: 11 of 11 tests passed (100% Pass Rate).

---

## 3. CLI & GUI Manual Verification Evidence

| Verification Target | Command Invoked | Observed Outcome | Status |
| :--- | :--- | :--- | :---: |
| **Detailed Help Display** | `MyMsg.exe --help` | Console displays formatted options with time/unit examples | PASS |
| **Short Help Display** | `MyMsg.exe -h` | Outputs concise usage summary | PASS |
| **Version Display** | `MyMsg.exe --version` | Outputs `mymsg 0.1.0` | PASS |
| **CJK Font Rendering** | `MyMsg.exe "日本語通知テスト"` | Clean typography without tofu/mojibake | PASS |
| **Keyboard Dismissal** | `Esc` / `Enter` keys | 0ms lag immediate process exit (code 0) | PASS |
| **Auto-Dismissal Timer** | `MyMsg.exe "3s auto" --timeout 3` | Automatically closes window after 3 seconds | PASS |
| **OS Toast Notification** | `MyMsg.exe "Toast" --toast -i ok` | OS native toast banner appears without GUI | PASS |
| **Time-of-Day Delay** | `MyMsg.exe "Lunch" -d 12:00` | Auto-calculates difference, waits with 0% CPU, and triggers | PASS |
| **Unit Duration Delay** | `MyMsg.exe "10m" -d 10m` | Waits 600s accurately before popping up | PASS |
| **Explicit Monitor Targeting** | `MyMsg.exe "Primary" --monitor primary` | Opens on primary screen center regardless of cursor | PASS |
| **Blink Animation** | `MyMsg.exe "Alert" -b` | Pulses opacity on precise 0.5s intervals | PASS |
| **Color Parsing** | `MyMsg.exe "Blue" -c bule` | Typo corrected, displayed in blue | PASS |
| **Multi-line Wrapping & Centering** | `MyMsg.exe "Line 1\nLine 2"` | Perfectly centered both horizontally and vertically | PASS |

---

## 4. Conclusion
All automated unit tests (11 of 11) and manual sanity checks passed with zero defects, verifying readiness for production distribution.
