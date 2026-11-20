# MyMsg Security Policy (SECURITY)

This document describes the security model, offline execution guarantees, input validation boundaries, and vulnerability disclosure procedures for `MyMsg`.

---

## 1. Security Architecture & Zero Network Access

- **Zero Network Ingestion & Telemetry**:
  `MyMsg` contains zero network dependencies, analytics, telemetry, or remote capabilities. It runs purely within the boundary of the local operating system.
- **Unprivileged Execution**:
  Does not require elevated administrative privileges (Administrator or root) to operate.
- **Memory Safety Guarantee**:
  Core business logic is written in safe Rust, with `unsafe` blocks strictly isolated to minimal Windows native API bindings (monitor enumeration, session detection, notification chime, and message pump flush).

---

## 2. Input Validation & Safe Boundaries

- **Delay & Duration Overflow Guard (`clamp_delay_seconds`, `parse_duration_to_seconds`)**:
  `--delay` values are strictly capped at 86400 seconds (24 hours), and duration unit multipliers use `checked_mul` to prevent integer overflows and runaway sleep processes.
- **Font Size Boundary Guard (`calculate_window_dimensions`)**:
  Non-finite (`NaN`, `Infinity`) or non-positive `--font-size` values fall back safely to the preset default, and extreme values are clamped to a safe range (8.0–200.0 pt).
- **Fallback Color Parsing & Multi-Byte Safety (`parse_color`)**:
  Unrecognized color strings, malformed HEX codes, and non-ASCII multi-byte inputs fail gracefully to preset safe UI defaults without panicking.
- **Safe Dynamic Font Loading**:
  Missing system font files or file permission errors are safely handled, falling back to default built-in fonts without crashing.

---

## 3. Reporting a Vulnerability

If you discover a security vulnerability within `MyMsg`, please report it confidentially:

1. Use GitHub's private **Security Advisories** tab on the repository.
2. A maintainer will review the disclosure within 48 hours and coordinate a fix.
