# Code Style – Northline

- Language: Rust only.
- Edition: 2021 or 2024.
- Formatting: `cargo fmt` defaults.
- Lints: fix all `cargo clippy` warnings unless explicitly documented.

## General

- No `unsafe` in our own code unless absolutely necessary; isolate any required unsafe (e.g., from bindings) in very small modules.[web:73][web:106]
- Prefer explicit types in public APIs.
- Prefer `&str` and `&[T]` in APIs over owned `String`/`Vec<T>` where possible.

## Errors

- Use `thiserror` for library error enums where convenient.[web:102]
- Each error enum variant should have:
  - A clear name (e.g., `ConfigMissing`, `ProviderUnavailable`).
  - A human‑readable message.

## Modules and files

- One responsibility per module.
- Target ≤ 250 LOC per file; split modules before they get too large.
- Name modules by function: `config`, `providers`, `tools`, `engine`, `errors`, etc.

