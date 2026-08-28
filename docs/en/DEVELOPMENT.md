# MyMsg Development Guide (DEVELOPMENT)

This document covers local development environment setup, building, running unit tests, formatting, and debugging for `MyMsg`.

---

## 1. Prerequisites & Setup

### Prerequisites
- **Rust Toolchain**: Rust 1.80 or later (stable channel recommended)
- **Git**: Source version control

```bash
rustc --version
cargo --version
```

### Clone Repository
```bash
git clone https://github.com/tkshnkgwr/MyMsg.git
cd MyMsg
```

---

## 2. Local Build & Run

### Debug Build & Run
```bash
# Debug compilation
cargo build

# Run with arguments
cargo run -- "Hello World" -c cyan

# Run help output
cargo run -- --help
```

---

## 3. Running Unit Tests

Run the built-in test suite:

```bash
# Run all unit tests
cargo test

# Run a specific test
cargo test test_parse_color

# Run with stdout visible
cargo test -- --nocapture
```

---

## 4. Code Quality & Linting

```bash
# Format check
cargo fmt --check

# Auto-format
cargo fmt

# Clippy linter
cargo clippy -- -D warnings
```

---

## 5. Troubleshooting & Debugging

- **Japanese Characters Displaying as Boxes (□)**:
  Verify that `%WINDIR%\Fonts\meiryo.ttc` or `msgothic.ttc` is present and accessible.
- **`--help` Not Displaying in Console**:
  Ensure `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` is not present in `src/main.rs`.
