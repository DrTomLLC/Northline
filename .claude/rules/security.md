# Security & Privacy – Northline

## Secrets & config

- Use secret/zeroization crates for in‑memory secrets (API keys, tokens).[web:72][web:75][web:78]
- Store config at rest only via `northline-crypto` (encrypted).
- Never log or print secrets or full prompts.

## Ghost Mode

- When `Ghost Mode` is enabled:
  - Do NOT write any files (config, logs, history).
  - Avoid caching data in persistent stores.
  - Log nothing, or log to an in‑memory buffer that is dropped on exit.

## Network

- Default to:
  - Local endpoints (LM Studio, Ollama) where possible.[web:59][web:65]
  - HTTPS for remote endpoints.
- No telemetry, analytics, or hidden network calls.

## Audits

- Run `cargo audit` regularly and treat critical issues as blockers.[web:73][web:76]
- Prefer crates with active maintenance and good security posture.
