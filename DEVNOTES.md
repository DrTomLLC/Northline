# Northline – DEVNOTES

These notes describe how to **develop** Northline day‑to‑day, especially with AI tools and the safety‑critical+privacy‑first requirements.

Read this together with:

- `RULES.md`
- `CONTRIBUTING.md`
- `.claude/CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/SECURITY.md`
- `docs/STRUCTURE.md`

---

## 1. Repository layout (intended)

Root:

- `core/` – `northline-core`  
  Frozen kernel: config, providers (OpenAI/LM Studio/Ollama/Local), tools (SearXNG), engine, error types.
- `crypto/` – `northline-crypto`  
  Encrypted config storage, secret handling.
- `desktop/` – `northline-desktop`  
  Slint UI host and app state.
- `third_party/fyin/`  
  Original Fyin project, used as a reference/code mine, not edited as main code.
- `docs/`  
  Architecture, security, structure, roadmap.
- `.claude/`  
  AI guidance and rules.
- `.github/`  
  CI and issue/PR templates.

MSRV is **Rust 1.93.0 / edition 2024**.

---

## 2. Development flow (human + AI)

### 2.1 General feature workflow

For any new feature or refactor:

1. **Clarify the goal**
   - Write a short note in `TODO.md` or a GitHub issue:
     - What you want.
     - Constraints (safety, privacy, no panics, etc.).

2. **Update or confirm design**
   - If the change affects the kernel, check `docs/ARCHITECTURE.md`.
   - If it does, update the docs first; v1 kernel API changes must be deliberate.

3. **Use AI to generate code _within_ the design**
   - Tell the AI:
     - Which crate/module to work in.
     - Which public signatures must be respected.
     - That it must obey `RULES.md` and `.claude/*`.

4. **Run the tool pipeline**
   - At minimum:
     - `cargo fmt`
     - `cargo clippy --workspace -- -D warnings`
     - `cargo test --workspace`
   - When configured, also:
     - `cargo nextest run`
     - `cargo llvm-cov`
     - `cargo audit`
     - `cargo deny check`
     - `cargo vet`
     - `cargo geiger`
     - `cargo careful`
     - `cargo udeps`
     - `cargo outdated`
     - `cargo machete`
     - `cargo semver-checks`
     - `cargo mutants` on critical code

   - If a tool is missing or fails to run:
     - Do **not** block progress.
     - Note it in a comment, TODO, or CI log and move on.

5. **Review manually**
   - Check for:
     - Any hidden `unwrap`/`expect`/`panic!`.
     - Logging of secrets or prompts.
     - Violations of file size / module responsibility.

6. **Commit**
   - Small, focused commits per feature or bug.

### 2.2 Bug‑first / test‑first flow

For every bug or incorrect behavior:

1. **Write a test that reproduces it**
   - Place in `core/tests/`, `crypto/tests/`, or `desktop/tests/`.
   - Use `Result<(), _>` tests, no `unwrap` in tests either.

2. **Confirm the test fails**
   - `cargo test --workspace` should show failure.

3. **Fix the code**
   - Use AI to propose a change if useful.
   - Keep changes small and in the correct module.

4. **Re‑run tests and tools**
   - Tests must pass.
   - `cargo clippy` must be clean.
   - Security tools (audit/deny/vet) should be run regularly.

5. **Treat lack of a test as a bug**
   - No bug fix is “done” without a test.

---

## 3. Using Fyin as a reference

- Fyin lives under `third_party/fyin/`.
- It is **not** part of the Northline workspace and should not be edited as the main codebase.
- Use Fyin to:
  - Understand RAG flow, config, and search behavior.
  - Copy logic **into** Northline modules, refactoring to fit the new API and rules.

Migration strategy:

- Implement Northline’s `northline-core` types and traits per `docs/ARCHITECTURE.md`.
- Port Fyin functionality into:
  - `core/src/config/` (config/env translation).
  - `core/src/providers/` (HTTP/LLM calls).
  - `core/src/tools/searxng.rs` (search).
  - `core/src/engine/` (RAG and orchestration).
- Keep Fyin’s CLI runnable separately for behavioral comparison if useful.

---

## 4. Kernel and API notes

- `northline-core` is the **kernel**:
  - It exposes `AppConfig`, `UserQuery`, `FinalAnswer`, `Source`, `ToolTrace`, `ErrorCode`, `CoreError`.
  - It defines `ModelBackend` and `Tool` traits.
  - It provides `run_query(config, query) -> Result<FinalAnswer, CoreError>` (and possibly streaming variants).

- After v1:
  - These APIs are **frozen**:
    - Only additive changes allowed (new variants/fields with defaults).
    - No breaking changes to signatures or semantics without a major version bump.

---

## 5. Safety‑critical practices

- **No panics** in production code:
  - No `unwrap`, `expect`, `panic!` in `core`, `crypto`, `desktop`.
- **Error codes and messages**:
  - Every error reaching the UI must carry:
    - An `ErrorCode`.
    - A clear message (no meaningless “Something went wrong”).
- **Unsafe code**:
  - Avoid `unsafe` in our own code.
  - If absolutely needed (e.g., for FFI/bindings), isolate in:
    - A tiny module.
    - With explicit safety docs and tests.

---

## 6. Security & privacy practices

- Secrets:
  - Only stored via `northline-crypto` in encrypted form.
  - In memory, use secret types that zeroize on drop where possible.

- Ghost Mode:
  - Implemented as a runtime flag exposed to:
    - `core` (e.g., disable persistence and extra network calls).
    - `desktop` (UI toggle, warnings).
  - When enabled:
    - No writes to disk.
    - No logging to persistent storage.
    - Network calls limited to user‑selected providers.

- No telemetry:
  - No hidden network requests.
  - No third‑party analytics libraries.

---

## 7. CI and tools

Planned CI steps (see `.github/workflows/ci.yml` once extended):

- Build: `cargo build --workspace`
- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --workspace -- -D warnings`
- Test: `cargo test --workspace`
- (Later) Security and quality:
  - `cargo audit`
  - `cargo deny check`
  - `cargo vet`
  - `cargo geiger`
  - `cargo careful`
  - `cargo nextest run`
  - `cargo llvm-cov`
  - `cargo udeps`
  - `cargo outdated`
  - `cargo machete`
  - `cargo semver-checks`
  - `cargo mutants` (targeted)

CI behavior:

- If any tool is missing or temporarily broken:
  - Mark that job as “skipped” or “incomplete”.
  - Do not block all development, but do not hide the failure.

---

## 8. Short “getting started” checklist for devs/AI

1. Ensure Rust 1.93.0 toolchain is active (via `rust-toolchain.toml`).
2. Clone repo; check `README.md`, `ROADMAP.md`, `docs/ARCHITECTURE.md`.
3. If crates are not yet set up:
   - Create `core`, `crypto`, `desktop` per `docs/STRUCTURE.md`.
4. Implement or adjust code **only** inside Northline crates, using Fyin under `third_party/` as reference.
5. For each change:
   - Add/adjust tests.
   - Run `cargo fmt`, `cargo clippy`, `cargo test`.
   - Optionally run additional tools.
6. Commit with clear messages and small, focused diffs.

---

These notes are for **practical work** on Northline. If a decision or behavior isn’t covered here, check the other docs; if it’s still unclear, write it down here before coding.

