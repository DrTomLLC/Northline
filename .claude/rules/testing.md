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
  - Happy paths.
  - Error paths (invalid config, network failures, model load failures, etc.).[web:98][web:101]

## Bug‑driven workflow

For any bug or incorrect behavior:

1. Write or extend tests that reproduce the issue.
2. Confirm the new test fails.
3. Fix the implementation.
4. Confirm the test now passes.

No bug should be “fixed” without a corresponding test.

## Safety‑critical focus

- For each public entrypoint (e.g., `run_query`), ensure tests cover:
  - Valid inputs.
  - Each relevant `ErrorCode` variant’s triggering condition.
- Use mutation testing (`cargo mutants`) and coverage (`cargo llvm-cov`) on critical modules where feasible.[web:173]
