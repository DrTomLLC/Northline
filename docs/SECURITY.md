
***

## 4. docs/SECURITY.md (safety‑critical + Ghost Mode)

``markdown
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
- `cargo audit` and similar tools are used regularly; known critical vulnerabilities must be addressed before releases.[web:73][web:76]
