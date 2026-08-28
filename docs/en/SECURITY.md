# MyMsg Security Policy (SECURITY)

This document describes the security model, offline execution guarantees, input validation boundaries, and vulnerability disclosure procedures for `MyMsg`.

---

## 1. Security Architecture & Zero Network Access

- **Zero Network Ingestion & Telemetry**:
  `MyMsg` contains zero network dependencies, analytics, telemetry, or remote capabilities. It runs purely within the boundary of the local operating system.
- **Unprivileged Execution**:
  Does not require elevated administrative privileges (Administrator or root) to operate.
- **Memory Safety Guarantee**:
  Built with safe Rust, mitigating memory corruption, arbitrary buffer overflows, and null-pointer exceptions.

---

## 2. Input Validation & Safe Boundaries

- **Delay Bounds Checking (`clamp_delay_seconds`)**:
  `--delay` values are strictly capped at 3600 seconds (1 hour) to prevent runaway background sleep processes.
- **Fallback Color Parsing**:
  Unrecognized color strings fail gracefully to preset safe UI defaults without panicking.
- **Safe Dynamic Font Loading**:
  Missing system font files or file permission errors are safely handled, falling back to default built-in fonts without crashing.

---

## 3. Reporting a Vulnerability

If you discover a security vulnerability within `MyMsg`, please report it confidentially:

1. Use GitHub's private **Security Advisories** tab on the repository.
2. A maintainer will review the disclosure within 48 hours and coordinate a fix.
