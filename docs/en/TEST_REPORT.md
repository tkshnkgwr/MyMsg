# MyMsg Test Execution Report (TEST_REPORT)

This document contains the execution report and verification evidence for the `MyMsg` v0.1.0 release.

---

## 1. Test Environment

| Parameter | Value |
| :--- | :--- |
| **OS** | Windows 11 (x86_64) |
| **Rust Version** | 1.80+ / stable-x86_64-pc-windows-msvc |
| **Cargo Version** | cargo 1.80+ |
| **Target Version** | v0.1.0 |
| **Execution Date** | 2026-08-28 |

---

## 2. Automated Unit Test Results

```
running 9 tests
test tests::test_calculate_window_dimensions ... ok
test tests::test_clamp_delay_seconds ... ok
test tests::test_resolve_message_priority ... ok
test tests::test_parse_color_named_and_typo ... ok
test tests::test_parse_icon ... ok
test tests::test_parse_theme ... ok
test tests::test_resolve_message_newlines ... ok
test tests::test_resolve_theme_palette ... ok
test tests::test_parse_color_hex ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**Summary**: 9 of 9 tests passed (100% Pass Rate).

---

## 3. CLI & GUI Manual Verification Evidence

| Verification Target | Command Invoked | Observed Outcome | Status |
| :--- | :--- | :--- | :---: |
| **Help Display** | `MyMsg.exe --help` | Console displays formatted options instantly | PASS |
| **Version Display** | `MyMsg.exe --version` | Outputs `mymsg 0.1.0` | PASS |
| **CJK Font Rendering** | `MyMsg.exe "日本語通知テスト"` | Clean typography without tofu/mojibake | PASS |
| **Keyboard Dismissal** | `Esc` / `Enter` keys | 0ms lag immediate process exit (code 0) | PASS |
| **Blink Animation** | `MyMsg.exe "Alert" -b` | Pulses opacity on precise 0.5s intervals | PASS |
| **Delay Timer** | `MyMsg.exe "Delayed" -d 3` | Sleeps 3 seconds before opening window | PASS |
| **Color Parsing** | `MyMsg.exe "Blue" -c bule` | Typo corrected, displayed in blue | PASS |
| **Status Icons** | `MyMsg.exe "Warning" -i warn` | Displays yellow ⚠ warning symbol before text | PASS |
| **Theme Selection** | `MyMsg.exe "Light" -t light` | Renders in light background and navy text | PASS |
| **Multi-line Wrapping & Centering** | `MyMsg.exe "Line 1\nLine 2"` | Perfectly centered both horizontally and vertically | PASS |
| **Multi-Monitor Follow** | Sub-monitor `MyMsg.exe "Alert"` | Opens centered on the active display containing the cursor | PASS |

---

## 4. Conclusion
All automated unit tests (9 of 9) and manual sanity checks passed with zero defects, verifying v0.1.0 readiness for production distribution.
