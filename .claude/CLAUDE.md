# Northline – CLAUDE.md

## What this project is

Northline is a **safety‑critical, privacy‑first local AI assistant and research engine**. It has:

- A frozen Rust **kernel** (`northline-core`) with panic‑free APIs.
- A native **Slint** desktop UI (`northline-desktop`).
- Strong **encryption** and **Ghost Mode** (no persistence, no logs).
- Support for **OpenAI‑compatible APIs, Ollama, LM Studio, and local GGUF models** discovered from a `models/` directory.[web:52][web:55][web:59][web:94][web:100]

Future versions add a Comet‑style browser host, but the v1 kernel API must remain stable.

## Tech stack

- Language: Rust (edition 2024).
- UI: Slint.
- Crates:
  - `core/` → `northline-core`
  - `crypto/` → `northline-crypto`
  - `desktop/` → `northline-desktop`
- No C, C++, Zig, Node, or Docker code in this repo.

## Absolute rules

When generating or editing code:

1. **No panics in production code**
   - Do NOT use `unwrap`, `expect`, or `panic!` in `core`, `crypto`, or `desktop`.
   - All operations must return `Result<T, CoreError>` (or crate‑specific error types) and handle failures explicitly.[web:98][web:99][web:104]

2. **Safety‑critical error handling**
   - Every public function that can fail must have a documented error path.
   - Use typed error enums with machine‑readable `ErrorCode` and human‑readable `message`.
   - No silent fallbacks or swallowing errors.

3. **Privacy and security**
   - Never log secrets, prompts, or full URLs; logs may contain only high‑level event info.
   - Use vetted crypto/secrets crates; no home‑grown cryptography.[web:72][web:73]
   - Ghost Mode must:
     - Write nothing to disk.
     - Avoid logging.
     - Minimize network calls.

4. **Modular, small files**
   - Each module does one thing.
   - Target ≤ 250 LOC per file; 400 LOC is a hard ceiling; split files before exceeding.

5. **Tests**
   - All tests live under `tests/` directories, not inline.
   - Tests should return `Result<(), TestError>` and use `?` instead of `unwrap`.

## What to do first in this repo

When asked to “start implementing Northline”:

1. **Set up the Cargo workspace** with crates:
   - `core/`, `crypto/`, `desktop/` as described in `ROADMAP.md` and `docs/ARCHITECTURE.md`.
2. **Define public types and traits** in `northline-core` following `docs/ARCHITECTURE.md`.
3. **Define crypto interfaces** in `northline-crypto`.
4. **Create a minimal Slint UI skeleton** in `northline-desktop`.

Do **not** skip straight to complex features (RAG, browser host) until the kernel API and basic structure compile and pass `cargo fmt` and `cargo clippy`.

## Helpful references in this repo

- `README.md` – high‑level description.
- `ROADMAP.md` – milestones and priorities.
- `CONTRIBUTING.md` – rules for coding and tests.
- `docs/ARCHITECTURE.md` – kernel API spec.
- `docs/SECURITY.md` – security, encryption, Ghost Mode.

Always prefer following these docs over “being clever”.
