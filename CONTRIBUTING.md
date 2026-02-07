# Contributing to Northline

Northline is safety‑critical. All contributions (including AI‑generated code) **must** follow these rules.

## Core principles

- **Safety‑critical:** No panics in production code. All failures must be handled explicitly.
- **Privacy‑first:** No hidden telemetry, no third‑party analytics, no secret logging.
- **Modular and small:** Files should be small and focused (target ≤ 250 LOC, hard max 400).

## Project structure

- `core/` – `northline-core` (kernel: providers, tools, engine, errors).
- `crypto/` – `northline-crypto` (secret handling, encryption, config).
- `desktop/` – `northline-desktop` (Slint UI host).
- `plugins/` – optional extension crates (later).

## Coding rules

1. **No panics in non‑test code**
   - Do not use `unwrap()`, `expect()`, or `panic!()` in `core`, `crypto`, or `desktop` (except truly unreachable enums, and even those should prefer explicit errors).
   - Use `Result<T, CoreError>` (or crate‑specific error types) everywhere.[web:98][web:99][web:104]

2. **Error design**
   - Use explicit error enums (e.g., `ErrorCode`, `CoreError`, `CryptoError`) with clear, user‑facing messages.
   - Do not silently swallow errors or auto‑fallback without user visibility.

3. **File and module size**
   - Each module should have a single responsibility.
   - Keep files ≤ 250 LOC where practical (never exceed 400 LOC by design); split modules when they grow.

4. **Tests**
   - All tests go in `tests/` directories (crate root or module‑specific), not inline with production code.
   - Tests should return `Result<(), TestError>` and use `?`, not `unwrap`.

5. **Security & privacy**
   - Use vetted crates for crypto and secrets; no home‑grown crypto.[web:72][web:73]
   - Never log API keys, secrets, or full prompts.
   - Ghost Mode must never write to disk and must minimize network activity.

6. **AI‑generated code**
   - AI output must be:
     - Reviewed by a human.
     - Conformant with all rules above.
   - No direct copy‑pasting of external proprietary code.

## CI expectations

- `cargo fmt` and `cargo clippy` must pass.
- `cargo test` must pass.
- `cargo audit` (and, if configured, `cargo-deny`) must be clean.[web:73][web:76]

If you’re unsure whether a change breaks the “frozen kernel” guarantee for v1, open an issue or design doc first.
