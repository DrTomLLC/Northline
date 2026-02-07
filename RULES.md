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
   - If bindings to native libraries are needed (e.g., for local models), keep them behind small, well‑tested Rust wrappers.

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
     - Config loading.
     - Encryption/decryption.
     - Network calls.
     - Model loading.
   - Must have tests that cover both success and failure paths.

Breaking these rules is treated as a bug, even if the code compiles.
