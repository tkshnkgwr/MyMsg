# MyMsg Contribution Guidelines (CONTRIBUTING)

Thank you for your interest in contributing to `MyMsg`!
This document provides guidelines for reporting issues, proposing features, opening pull requests, and adhering to our branching and commit standards.

---

## 1. How to Contribute

### 1.1 Reporting Bugs
- Search existing issues before creating a new report.
- Include your operating system, command executed, expected behavior, and observed output.

### 1.2 Feature Requests
- Proposals aligned with the core philosophy of `MyMsg` ("lightweight, zero-overhead, single-binary") are warmly welcomed.

---

## 2. Branching Strategy & Workflow

1. Fork the repository and create a feature branch off `main`:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```
2. Implement your changes and add/update unit tests under `#[cfg(test)]`.
3. Verify that formatting, linting, and tests pass:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
4. Commit your changes and submit a Pull Request to `main`.

---

## 3. Commit Message Standards

We recommend following the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification:

- `feat: ...` : New feature
- `fix: ...` : Bug fix
- `docs: ...` : Documentation changes
- `refactor: ...` : Code improvement without behavioral change
- `test: ...` : Adding or improving tests
- `chore: ...` : Tooling or build configuration updates
