# Northline – CLAUDE.md

## What this project is

Northline is a **safety‑critical, privacy‑first local AI assistant and research engine**. It has:

- A frozen Rust **kernel** (`northline-core`) with panic‑free APIs.
- A native **Slint** desktop UI (`northline-desktop`).
- Strong **encryption** and **Ghost Mode** (no persistence, no logs).
- Support for **OpenAI‑compatible APIs, Ollama, LM Studio, and local GGUF models** discovered from a `models/` directory.[web:52][web:55][web:59][web:94][web:100]

Future versions add a Comet‑style browser host, but the v1 kernel API must remain stable.

## Tech stack & toolchain

- Language: Rust.
- Edition: 2024.
- Minimum supported Rust version (MSRV): **1.93.0**.
- Toolchain pinned in `rust-toolchain.toml` (1.93.0 with `rustfmt`, `clippy`).

## AI tools allowed

This repo will be edited using a combination of:

- Claude Code (cloud + local).
- Local LLMs (via LM Studio / Ollama / other).
- Antigravity.
- Gemini.
- ChatGPT.

**Preferences:**

- Use **local AI** as much as possible (no limits, free once the stack is working).
- Cloud tools are allowed but must still respect all project rules.

## Absolute rules for generated code

When generating or editing code:

1. **No panics in production code**
   - Do NOT use `unwrap`, `expect`, or `panic!` in `core`, `crypto`, or `desktop`.
   - All operations must return `Result<T, CoreError>` (or crate‑specific error types) and handle failures explicitly.[web:98][web:99][web:104]

2. **Safety‑critical error handling**
   - Every public function that can fail must have a documented error path.
   - Use typed error enums with machine‑readable `ErrorCode` and human‑readable `message`.
   - No silent fallbacks or swallowed errors.

3. **Privacy and security**
   - Never log secrets, prompts, or full URLs.
   - Use vetted crypto/secrets crates; no home‑grown cryptography.[web:72][web:73]
   - Ghost Mode must:
     - Write nothing to disk.
     - Avoid logging.
     - Minimize network calls.

4. **Modular, small files**
   - Each module does one thing.
   - Target ≤ 250 LOC per file; 400 LOC is a hard ceiling; split before exceeding.

5. **Tests and bug workflow**
   - For ANY bug or incorrect behavior:
     1. Write or extend tests that reproduce the failure.
     2. Verify tests fail.
     3. Fix the code.
     4. Verify tests pass.
   - This applies to logic errors, edge cases, and runtime failures.

6. **Rust‑only implementation**
   - No C, C++, Zig, Node, or Docker code in this repo.
   - If bindings to native libraries are needed (e.g., llama.cpp for GGUF), keep them behind small, well‑tested Rust wrappers.

## Automated tool pipeline

When instructed to “run tools” (locally or in CI), the expected sequence is:

1. `cargo fmt`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo test --workspace`
4. `cargo nextest run` (if configured)[web:173]
5. `cargo llvm-cov` (coverage, if configured)
6. `cargo audit`
7. `cargo deny check`
8. `cargo vet`
9. `cargo geiger` (unsafe usage report)
10. `cargo careful` (where applicable)
11. `cargo udeps`
12. `cargo outdated`
13. `cargo machete`
14. `cargo semver-checks` (for API changes)
15. `cargo mutants` (for mutation testing, on selected crates)[web:73][web:76][web:173]

**Behavior:**

- Run tools in the above order where configured.
- If any tool is not installed, fails to run, or is temporarily broken:
  - **Do not stop the pipeline.**
  - Skip that tool, record the failure in logs and/or a TODO comment, and continue with the next tool.
- The goal is: **no single tool failure blocks forward progress**, but all failures are visible and documented.

## What to do first in this repo

When asked to “start implementing Northline”:

1. Set up or confirm the Cargo workspace (`core`, `crypto`, `desktop`).
2. Define public types and traits in `northline-core` following `docs/ARCHITECTURE.md`.
3. Define crypto interfaces in `northline-crypto`.
4. Create a minimal Slint UI skeleton in `northline-desktop`.

Do NOT jump directly to complex features (RAG, browser host) until the kernel API compiles and passes `cargo fmt` and `cargo clippy`.

## Internal references

- `RULES.md`
- `CONTRIBUTING.md`
- `docs/ARCHITECTURE.md`
- `docs/SECURITY.md`
- `.claude/rules/*`

When in doubt, these docs override any AI suggestion.
