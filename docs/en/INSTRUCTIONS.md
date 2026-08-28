# MyMsg AI Agent Instructions (INSTRUCTIONS)

This document establishes the architecture principles, coding standards, interaction etiquette, and SemVer governance rules for AI agents and human contributors working on `MyMsg`.

---

## 1. Core Design Tenets

1. **Ultra-Lightweight & Minimal Overhead**:
   - Maintain 0.0% CPU consumption during idle state (strict event-driven rendering).
   - Never introduce heavy third-party crates without strict architectural necessity.
2. **Instant Response & Exit**:
   - Preserve zero-lag dismissal via `Esc` / `Enter` keys under all UI conditions.
3. **Robust Internationalization (i18n)**:
   - Always verify CJK font detection to prevent mojibake/tofu glyph rendering.
4. **Strict Backward Compatibility**:
   - Do not break existing CLI options, short aliases, or default behaviors.

---

## 2. Rust Coding Standards

- **Naming Conventions**:
  - Functions & Variables: `snake_case`
  - Structs & Enums: `PascalCase`
  - Constants: `SCREAMING_SNAKE_CASE`
- **Documentation**:
  - Provide RustDoc (`///`) comments on all public types and functions explaining intent, parameters, and return values.
  - Prefix changes with `// UPDATE YYYY-MM-DD: [Reason & Summary]`.
- **Test-Driven Modifications**:
  - Accompany every parser addition or mathematical logic change with unit tests under `#[cfg(test)]`.

---

## 3. Versioning (SemVer)

- **MAJOR (x.0.0)**: Breaking changes to CLI options or architecture.
- **MINOR (0.x.0)**: Non-breaking feature additions (new options, color palettes).
- **PATCH (0.0.x)**: Bug fixes, performance tuning, and documentation.

---

## 4. AI Agent Guidelines
- Run `cargo test` and `cargo build` before and after making modifications.
- Ensure all output files and documents are fully populated without placeholders or truncation.
