# Northline Roadmap

## Vision

Northline is a safety‑critical, privacy‑first local AI assistant and research engine with a frozen Rust kernel and modular hosts (desktop UI now, browser later).

---

## Milestone 1 — Workspace & Kernel Skeleton

**Goal:** Compile‑clean workspace with a well‑defined, panic‑free public API.

- Create Cargo workspace (`core`, `crypto`, `desktop` crates).
- Define public types and traits in `northline-core`:
  - `AppConfig`, `UserQuery`, `AnswerChunk`, `FinalAnswer`, `Source`, `ToolTrace`.
  - `ErrorCode`, `CoreError`.
  - `ModelBackend`, `Tool`, `Plugin` traits (signatures only).
  - `run_query(config, query) -> Result<FinalAnswer, CoreError>`.
- Define `northline-crypto` interfaces:
  - `EncryptedConfig`, `CryptoError`.
  - `encrypt_config`, `decrypt_config`.
  - `load_encrypted_config`, `save_encrypted_config`.
- Create `northline-desktop` stub:
  - Minimal Slint `MainWindow`.
  - `main.rs` wiring UI start, dummy handler.

**Exit criteria:**
- `cargo build` passes.
- All public APIs are documented and treated as v1‑frozen.

---

## Milestone 2 — Providers & Local Models

**Goal:** Engine can talk to OpenAI, LM Studio, Ollama, and a local GGUF model, with optional multi‑model comparison.

- Implement `ModelBackend` for:
  - `OpenAiBackend` (OpenAI‑compatible HTTP).
  - `LmStudioBackend` (local OpenAI‑compatible server).
  - `OllamaBackend`.
  - `LocalModelBackend` (GGUF in `models/` directory via llama.cpp‑style crate).
- Implement model discovery:
  - Scan `models/` directory at startup.
  - Expose discovered local models in a `ModelCatalog`.
- Design for single‑ and multi‑model modes:
  - Support a single active backend in simple configurations.
  - Add a composite `ModelBackend` that can fan out to multiple models in parallel for comparison, voting, or aggregation.
- Error handling:
  - Map all backend errors into `CoreError` with precise `ErrorCode`s (no panics).

**Exit criteria:**
- Simple CLI harness can run `run_query` against each backend.
- CLI (or config) can select between single‑model and multi‑model modes.
- Bad config / missing models / network errors return well‑formed `CoreError`.

---

## Milestone 3 — SearXNG + RAG Engine

**Goal:** Perplexity‑style answer generation via SearXNG + LLM.

- Implement `Tool` for SearXNG:
  - Calls self‑hosted SearXNG endpoint with query.
  - Returns structured search results (title, snippet, URL).
- Implement core RAG flow in `fyin-core`:
  - Run SearXNG search.
  - Fetch & chunk pages.
  - Embed & select relevant chunks.
  - Compose context + user query into LLM call.
- Emit `ToolTrace` for:
  - Search queries.
  - Page fetches.

**Exit criteria:**
- Basic test harness: text in, answer + source URLs out.
- Tool traces accurately reflect what the engine did.

---

## Milestone 4 — Slint Desktop UI

**Goal:** Native desktop app with chat UI and settings.

- Slint UI:
  - Sidebar: conversations.
  - Main: messages (user + assistant with citations).
  - Right pane: sources/trace (optional).
  - Bottom: input bar, “Ask” button.
- Settings UI:
  - Provider selection (OpenAI/Ollama/LM Studio/Local).
  - Model selection (remote + discovered local models).
  - Mode selection: single‑model or multi‑model comparison.
  - SearXNG settings.
  - Ghost Mode toggle.
- Wiring:
  - Call `run_query` async on submit; stream `AnswerChunk`s into UI.
  - Show errors using `ErrorCode` + messages.
  - In multi‑model mode, surface per‑model outputs and/or an aggregated answer.

**Exit criteria:**
- End‑to‑end flow works on desktop with at least one backend.
- Multi‑model mode can be enabled from settings and used in a chat.
- No panics; all failures visible as clean error dialogs.

---

## Milestone 5 — Security, Ghost Mode, and Tests

**Goal:** Harden behavior for safety‑critical and privacy‑first operation.

- Ghost Mode:
  - No disk writes (no config, no history).
  - Logging disabled or in‑memory only.
- Encryption:
  - Use `northline-crypto` for config at rest (AEAD).
- Tests:
  - Integration tests for:
    - Config missing/invalid.
    - Provider failures.
    - SearXNG failures.
    - Local model load failures.
    - Multi‑model comparison mode (success and failure paths).
  - All tests in `tests/` dirs, no inline tests.

**Exit criteria:**
- CI enforcing no `unwrap`/`expect`/`panic!` in core/crypto/desktop.
- `cargo test` + `cargo audit` clean.
