# Northline Rules

These rules are non‑negotiable. Northline is a **safety‑critical, privacy‑first** system.

1. **No panics in production code**
   - No `unwrap`, `expect`, or `panic!` in `core`, `crypto`, or `desktop`.
   - Failures must be represented by explicit error types and handled at the edges.

2. **Kernel API is frozen after v1**
   - Public types and traits in `northline-core` (see `docs/ARCHITECTURE.md`) are treated as stable.
   - After v1, only additive changes are allowed (new fields with defaults, new error codes, new traits).

3. **Privacy and Ghost Mode**
   - No telemetry, no third‑party analytics, no hidden logging.
   - Ghost Mode must:
     - Not persist anything.
     - Avoid logging.
     - Minimize network calls.

4. **Rust‑only implementation**
   - No C, C++, Zig, Node, or Docker in this repo.
   - If bindings to native libraries are needed (e.g., for GGUF), keep them behind small, well‑tested Rust wrappers.

5. **Modular, small, focused files**
   - Each module has a single responsibility.
   - Target ≤ 250 LOC per file, hard max 400; split when approaching this.

6. **Explicit error codes and messages**
   - Every error that can reach the UI must carry:
     - An `ErrorCode` variant.
     - A clear, user‑facing message.
   - No generic “something went wrong” without context.

7. **Tests are mandatory for safety‑critical paths**
   - Any code dealing with:
     - Config loading/saving.
     - Encryption/decryption.
     - Network calls.
     - Model loading and inference.
   - Must have tests covering both success and failure paths.

8. **Automated tool pipeline**
   - The preferred order for checks is:
     1. `cargo fmt`
     2. `cargo clippy --workspace -- -D warnings`
     3. `cargo test --workspace`
     4. `cargo nextest run` (if configured)
     5. `cargo llvm-cov`
     6. `cargo audit`
     7. `cargo deny check`
     8. `cargo vet`
     9. `cargo geiger`
     10. `cargo careful`
     11. `cargo udeps`
     12. `cargo outdated`
     13. `cargo machete`
     14. `cargo semver-checks`
     15. `cargo mutants` on critical modules[web:73][web:76][web:173]

   - If any tool is unavailable or fails to run, it must be:
     - Skipped for that run.
     - Documented (log, TODO, or CI note).
   - No single tool failure should prevent forward progress, but incomplete tool runs must be visible.

Breaking these rules is treated as a bug, even if the code compiles.
