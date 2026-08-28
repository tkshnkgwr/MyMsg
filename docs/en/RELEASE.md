# MyMsg Release Procedure (RELEASE)

This document outlines the procedures for building optimized release binaries, minimizing binary footprint, and packaging distributions for `MyMsg`.

---

## 1. Building Release Binaries

### Standard Release Build
```bash
cargo build --release
```
Target binary: `target/release/MyMsg.exe` (Windows) or `target/release/MyMsg` (Unix-like).

---

## 2. Binary Size Optimization

To produce a minimal, highly optimized binary, ensure the following profile configuration is present in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3          # Maximum optimizations
lto = true             # Link-Time Optimization
codegen-units = 1      # Reduce code generation units to maximize optimization scope
panic = "abort"        # Drop panic unwinding stack tables
strip = true           # Strip all debug symbols
```

### Additional Compression with UPX
Optionally compress the executable using UPX to reduce size by another 30%–60%:

```bash
upx --best --lzma target/release/MyMsg.exe
```

---

## 3. Pre-Release Quality Checklist

- [ ] All unit tests pass via `cargo test`.
- [ ] No warnings from `cargo fmt --check` or `cargo clippy`.
- [ ] Console help works cleanly on `MyMsg.exe --help`.
- [ ] CJK / Japanese fonts render cleanly without mojibake.
- [ ] `docs/en/CHANGELOG.md` and `docs/ja/CHANGELOG.md` reflect all recent changes under the target version.
- [ ] `Cargo.toml` version matches release tag.

---

## 4. Distribution Packaging

1. Package `MyMsg.exe`, `README.md`, and `LICENSE` into a ZIP archive.
2. Publish release assets to GitHub Releases under the version tag (e.g. `v0.1.0`).
