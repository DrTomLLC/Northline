# Security & Privacy – Northline

## Secrets & config

- Use secret/zeroization crates for in‑memory secrets (API keys, tokens).[web:72][web:75][web:78]
- Store config at rest only via `northline-crypto` (encrypted).
- Never log or print secrets, prompts, or full URLs.

## Ghost Mode

When Ghost Mode is enabled:

- Do NOT write any files (config, logs, history).
- Avoid caching data in persistent stores.
- Log nothing, or log only to an in‑memory buffer that is dropped on exit.
- Network calls restricted to what is strictly necessary (e.g., local LM Studio/Ollama or configured SearXNG).[web:59][web:65][web:79]

## Network

- Default to:
  - Local endpoints (LM Studio, Ollama) where possible.[web:59][web:65]
  - HTTPS for remote endpoints.
- No telemetry, analytics, or hidden network calls.

## Audits & tools

- Use:
  - `cargo audit`
  - `cargo deny`
  - `cargo vet`
- Treat critical issues as blockers.[web:73][web:76]

Tool failures:

- If a security tool fails to run (missing, broken), do **not** block progress; record the failure and continue with other tools, but mark the run as “security checks incomplete.”
