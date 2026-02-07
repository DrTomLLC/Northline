# Northline

Northline is a safety‑critical, privacy‑first local AI assistant and research engine. It combines a modular Rust core with a native Slint desktop UI, Perplexity‑style answers, and support for local and remote LLMs (OpenAI, Ollama, LM Studio, and drop‑in GGUF models).[web:52][web:55][web:59][web:94][web:100]

## Vision

Northline is designed as a frozen, safety‑critical **kernel** you can build on:

- A panic‑free Rust core with explicit, fully coded errors.
- Strong privacy and encryption by default.
- Modular, plugin‑friendly architecture.
- A Perplexity‑class UX today, and a Comet‑style AI browser in future versions.[web:58][web:61][web:64]

## Key features (planned v1)

- **Native desktop UI (Slint + Rust)**  
  Perplexity‑style chat interface with conversations, answers, and citations, no Docker or terminal glue.[web:46][web:35]

- **Multiple model backends**  
  - OpenAI‑compatible APIs (OpenAI, LM Studio, others).[web:52][web:53][web:59]  
  - Ollama for local models.  
  - Drop‑in local GGUF models from a `models/` directory.[web:94][web:100]

- **Deep web search with SearXNG**  
  Self‑hosted SearXNG as the primary search engine for RAG, configurable and privacy‑respecting.[web:79][web:82]

- **Ghost Mode**  
  “Leave no trace” mode with:
  - No persistence (no history or config writes).
  - No local logging.
  - Network calls minimized to only what’s strictly required.

- **Safety‑critical design**  
  - No panics in production code; all operations return structured `Result` types.[web:98][web:99][web:104]  
  - Strong encryption for stored secrets and configuration.[web:72][web:73]  
  - Clear error codes and human‑readable error messages.

## Architecture

Northline uses a single Cargo workspace:

- `core/` – `northline-core`: frozen safety‑critical kernel (providers, tools, RAG engine, error types).  
- `crypto/` – `northline-crypto`: secret handling and encrypted configuration.  
- `desktop/` – Slint desktop host (UI, settings, Ghost Mode toggle).  
- `plugins/` – optional plugin crates extending tools and capabilities.

Future versions can add a browser host (Comet‑style) and/or WASM targets without changing the v1 core.

## Status

Northline is in early design and refactor phase:

- Target: v1 = desktop assistant + RAG with a stable kernel.  
- v2+: AI‑first browser host and richer plugin ecosystem.

Contributions, design discussions, and security reviews are welcome.
