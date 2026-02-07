# Testing – Northline

## General

- All tests live under `tests/` directories:
  - `core/tests/`
  - `crypto/tests/`
  - `desktop/tests/`
- No inline `#[cfg(test)]` modules in production files.

## Test style

- Test functions should return `Result<(), TestError>` or `Result<(), Box<dyn std::error::Error>>`.
- Use `?` instead of `unwrap`/`expect` inside tests for failure reporting.
- Cover both:
  - “Happy path” behavior.
  - Error paths (invalid config, network failures, model load failures, etc.).[web:98][web:101]

## Safety‑critical focus

- For each public entrypoint (e.g., `run_query`), ensure tests cover:
  - Valid inputs.
  - Each `ErrorCode` variant’s triggering condition.
- Add regression tests when fixing any bug related to safety, privacy, or error handling.
