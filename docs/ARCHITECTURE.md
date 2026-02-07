# Northline Architecture (v1 Kernel)

## Overview

Northline v1 defines a frozen kernel in `northline-core`. Hosts (desktop UI, future browser) call the kernel via a small set of types and traits.

## Core types

``rust
pub struct AppConfig {
    // providers, search engine, privacy flags, model selections, paths, etc.
}

pub struct UserQuery {
    pub conversation_id: Option<uuid::Uuid>,
    pub text: String,
    // options: temperature, max_tokens, etc. (optional in v1)
}

pub struct AnswerChunk {
    pub text: String,
    // optional: index, source ids
}

pub struct Source {
    pub url: String,
    pub title: String,
}

pub struct ToolTrace {
    pub tool_name: String,
    pub input: String,
    pub output_summary: String,
}
Errors
rust
pub enum ErrorCode {
    ConfigMissing,
    ConfigInvalid,
    ProviderUnavailable,
    ProviderError,
    SearchUnavailable,
    SearchError,
    LocalModelLoadFailed,
    Timeout,
    Internal,
    // extend only by adding, not changing
}

pub struct CoreError {
    pub code: ErrorCode,
    pub message: String,
}
Traits
ModelBackend
rust
pub trait ModelBackend: Send + Sync {
    fn complete(
        &self,
        query: &UserQuery,
    ) -> Result<Box<dyn Iterator<Item = AnswerChunk> + Send>, CoreError>;

    fn embed(&self, text: &str) -> Result<Vec<f32>, CoreError>;
}
Concrete implementations:

OpenAiBackend

LmStudioBackend

OllamaBackend

LocalModelBackend (GGUF in models/ directory)[web:94][web:95][web:100]

Tool
rust
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn invoke(&self, input: &str) -> Result<String, CoreError>;
}
Concrete implementations in v1:

SearxngSearchTool (calls self‑hosted SearXNG).[web:79][web:82]

Engine entrypoints
rust
pub fn run_query(
    config: &AppConfig,
    query: UserQuery,
) -> Result<FinalAnswer, CoreError>;

pub struct FinalAnswer {
    pub text: String,
    pub sources: Vec<Source>,
    pub tool_trace: Vec<ToolTrace>,
}
This API is treated as frozen after v1: only additive changes are allowed (new error codes, new fields with defaults).

text

***

## 4. docs/SECURITY.md (safety‑critical + Ghost Mode)

```markdown
# Northline Security & Safety Model

Northline is designed as a safety‑critical, privacy‑first system.

## Safety‑critical behavior

- No panics in production code in `core`, `crypto`, or `desktop`.
- All operations return `Result<T, CoreError>` (or crate‑specific errors) with explicit `ErrorCode`s.[web:98][web:99][web:104]
- Error handling:
  - No silent failure or hidden fallbacks.
  - Timeouts and external service failures are surfaced explicitly.

## Secrets and encryption

- API keys and secrets are stored in memory using secret/zeroization types where possible.[web:72][web:75][web:78]
- Configuration at rest is encrypted using a vetted AEAD cipher (e.g., XChaCha20‑Poly1305).[web:72][web:73]
- Keys are stored via OS‑protected storage or user‑provided secret (depending on mode).

## Ghost Mode

Ghost Mode is a runtime flag that enforces:

- No writes to disk:
  - No config updates, no conversation history, no cache.
- Logging:
  - Disabled or kept in non‑persistent memory.
- Network:
  - Restricted to the minimum necessary for the chosen backend (e.g., localhost LM Studio/Ollama, configured SearXNG; no telemetry).

Ghost Mode cannot hide basic facts visible to the OS/ISP (e.g., IP addresses contacted), but it ensures Northline itself leaves no local traces.

## Dependencies & audits

- Dependencies are minimized and vetted for security.
- `cargo audit` and similar tools are used regularly; known critical vulnerabilities must be addressed before releases.
