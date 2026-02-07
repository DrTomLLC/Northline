# Northline – TODO

This is the high‑level implementation checklist. Tasks are ordered so the **kernel** and structure are solid before features, with AI + tools in the loop.

---

## Phase 0 – Imports & Skeleton

- [ ] Add Fyin as a reference project
  - [ ] Create `third_party/fyin/`.
  - [ ] Copy the full Fyin repo into `third_party/fyin/` (keep it buildable as‑is).[page:2]

- [ ] Create Cargo workspace
  - [ ] Root `Cargo.toml` with `workspace` members: `core`, `crypto`, `desktop`.
  - [ ] Add minimal `Cargo.toml` for:
    - [ ] `core/` (northline-core)
    - [ ] `crypto/` (northline-crypto)
    - [ ] `desktop/` (northline-desktop)
  - [ ] Confirm `cargo build` works with empty crates on Rust 1.93.0.

---

## Phase 1 – Kernel API (northline-core)

- [ ] Define core types in `northline-core` (per `docs/ARCHITECTURE.md`)
  - [ ] `AppConfig`
  - [ ] `UserQuery`
  - [ ] `AnswerChunk`
  - [ ] `Source`
  - [ ] `ToolTrace`
  - [ ] `FinalAnswer`
- [ ] Define error types
  - [ ] `ErrorCode` enum
  - [ ] `CoreError` struct with code + message
- [ ] Define traits
  - [ ] `ModelBackend`
  - [ ] `Tool`
  - [ ] (Optional v1) `Plugin` trait for static plugins
- [ ] Define engine entry
  - [ ] `run_query(config: &AppConfig, query: UserQuery) -> Result<FinalAnswer, CoreError>`

- [ ] Add unit / integration tests for:
  - [ ] Basic construction and default behaviors of core types.
  - [ ] Error mapping (each `ErrorCode` has a clear, non‑empty message).

---

## Phase 2 – Crypto & Config (northline-crypto + config module)

- [ ] Implement `northline-crypto` API
  - [ ] `EncryptedConfig`
  - [ ] `CryptoError`
  - [ ] `encrypt_config(plaintext: &[u8]) -> Result<EncryptedConfig, CryptoError>`
  - [ ] `decrypt_config(cipher: &EncryptedConfig) -> Result<Vec<u8>, CryptoError>`
  - [ ] `load_encrypted_config() -> Result<Option<EncryptedConfig>, CryptoError>`
  - [ ] `save_encrypted_config(cfg: &EncryptedConfig) -> Result<(), CryptoError>`
- [ ] Connect config in `northline-core`
  - [ ] `config` module reads/writes `AppConfig` via `northline-crypto`.
  - [ ] Map Fyin’s `.env` / config into `AppConfig` structure as needed.[page:2]

- [ ] Tests
  - [ ] Round‑trip encryption/decryption tests.
  - [ ] Corrupted config file -> `CryptoError::ConfigCorrupted`.
  - [ ] Missing config file -> `Ok(None)` for `load_encrypted_config`.

---

## Phase 3 – Providers (OpenAI, LM Studio, Ollama, Local GGUF)

- [ ] Provider abstractions
  - [ ] Create `core/src/providers/mod.rs`.
  - [ ] Implement `ModelBackend` for:
    - [ ] `OpenAiBackend`
    - [ ] `LmStudioBackend` (OpenAI‑compatible local server).[web:52][web:55][web:59]
    - [ ] `OllamaBackend`
    - [ ] `LocalModelBackend` (GGUF in `models/` directory via llama.cpp‑style crate).[web:94][web:95][web:100]

- [ ] Local model discovery
  - [ ] Scan `models/` directory at startup.
  - [ ] Filter for `.gguf` (and other supported formats if any).
  - [ ] Build a `ModelCatalog` with discovered local models.

- [ ] Error handling tests
  - [ ] Invalid/missing API keys -> `ErrorCode::ConfigInvalid` or `ProviderUnavailable`.
  - [ ] Network failures/timeouts -> `ErrorCode::ProviderError` or `ErrorCode::Timeout`.
  - [ ] Invalid GGUF file / load error -> `ErrorCode::LocalModelLoadFailed`.

---

## Phase 4 – Tools & RAG (SearXNG, engine)

- [ ] Implement SearXNG tool
  - [ ] `core/src/tools/searxng.rs` with `SearxngSearchTool` implementing `Tool`.[web:79][web:82]
  - [ ] Configurable SearXNG endpoint via `AppConfig`.
  - [ ] Parse results into structured `Source` entries.

- [ ] Implement RAG engine in `core/src/engine/`
  - [ ] Use Fyin logic as reference for:
    - [ ] Running search.
    - [ ] Fetching pages.
    - [ ] Chunking and embedding.
    - [ ] Selecting relevant chunks.
    - [ ] Composing final prompt and calling `ModelBackend`.
  - [ ] Emit `ToolTrace` events for search and fetch.

- [ ] `run_query` implementation
  - [ ] Wire `run_query` to the engine:
    - [ ] Load config.
    - [ ] Choose backend + tool(s).
    - [ ] Execute RAG.
    - [ ] Return `FinalAnswer` with sources + trace.

- [ ] Tests
  - [ ] Integration test with mocked provider and SearXNG responses.
  - [ ] Tool trace contains expected search queries and URLs.

---

## Phase 5 – Desktop UI (northline-desktop, Slint)

- [ ] Slint UI skeleton
  - [ ] `desktop/ui/MainWindow.slint` with:
    - [ ] Left sidebar (conversation list placeholder).
    - [ ] Center chat area (messages list).
    - [ ] Right pane (sources / trace placeholder).
    - [ ] Bottom input bar + “Ask” button.

- [ ] App state
  - [ ] `desktop/src/app_state.rs` (or similar) with:
    - [ ] Conversation model(s).
    - [ ] Current query state.
    - [ ] Ghost Mode flag.
    - [ ] Selected backend/model.

- [ ] Wiring to core
  - [ ] On `submit-query`:
    - [ ] Build `UserQuery` and call `northline-core::run_query`.
    - [ ] Stream or append `FinalAnswer.text` to the messages list.
    - [ ] Render citations and `ToolTrace` in the UI.

- [ ] Settings UI
  - [ ] Provider selection (OpenAI, LM Studio, Ollama, Local).
  - [ ] Model selection (remote + discovered local models).
  - [ ] SearXNG endpoint settings.
  - [ ] Ghost Mode toggle.

- [ ] Tests
  - [ ] Minimal integration test that starts the UI and calls a dummy `run_query` (headless or simplified).

---

## Phase 6 – Ghost Mode, Security, and Tooling

- [ ] Implement Ghost Mode
  - [ ] Add ghost‑mode flag to `AppConfig` and UI.
  - [ ] Ensure:
    - [ ] No file writes when enabled.
    - [ ] Logging is disabled or in‑memory only.
    - [ ] Network calls are minimized.

- [ ] Logging & redaction
  - [ ] Add logging with strict rules (no secrets/prompts/URLs).
  - [ ] Confirm logs are silent or minimal in Ghost Mode.

- [ ] Security tools (wire into CI when ready)
  - [ ] Add jobs/targets for:
    - [ ] `cargo audit`
    - [ ] `cargo deny`
    - [ ] `cargo vet`
    - [ ] `cargo geiger`
    - [ ] `cargo careful`
    - [ ] `cargo nextest`, `cargo llvm-cov`, `cargo mutants` on core.

- [ ] Tests
  - [ ] Verify Ghost Mode behavior via tests (no files created, etc.).
  - [ ] Verify error codes and messages for key security/privacy paths.

---

## Phase 7 – Polish & v1 Freeze

- [ ] Review `northline-core` public API vs `docs/ARCHITECTURE.md`.
- [ ] Confirm:
  - [ ] No `unwrap`/`expect`/`panic!` in `core`, `crypto`, `desktop`.
  - [ ] All public errors use `CoreError`/`ErrorCode` or appropriate crate errors.
  - [ ] File sizes stay within target (≤ 250 LOC, max 400).
- [ ] Tag a v1.0.0 of the kernel.
- [ ] Update `ROADMAP.md` with v2+ items (browser host, dynamic plugins, etc.).

---

This list should be updated as you progress (check off items, add new granular tasks as needed), but it’s now aligned with your rules, security posture, AI workflow, and the Fyin‑to‑Northline refactor plan.
