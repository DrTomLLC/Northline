# Northline Repository Structure (Planned)

## Crates

- `core/` → `northline-core`
  - `src/lib.rs` – public kernel exports.
  - Modules:
    - `config/` – configuration types & loading.
    - `providers/` – OpenAI, LM Studio, Ollama, LocalModel backends.
    - `tools/` – SearXNG search and other tools.
    - `engine/` – RAG engine and `run_query`.
    - `errors.rs` – `ErrorCode`, `CoreError`.
- `crypto/` → `northline-crypto`
  - `src/lib.rs` – crypto API.
  - `src/config_store.rs` – encrypted config load/save.
  - `src/errors.rs` – `CryptoError`.
- `desktop/` → `northline-desktop`
  - `src/main.rs` – Slint app entry.
  - `ui/MainWindow.slint` – main UI definition.
  - `src/app_state.rs` – state models, Ghost Mode flag.

## Tests

- `core/tests/` – integration tests for kernel.
- `crypto/tests/` – tests for encryption/config.
- `desktop/tests/` – UI integration tests (where feasible).

All new code must fit into this structure or extend it in a consistent way.
