# AI Usage in Northline

Northline will be implemented with assistance from AI tools:

- Claude Code (cloud + local).
- Local LLMs (via LM Studio / Ollama / others).
- Antigravity.
- Gemini.
- ChatGPT.

## Principles

- AI is a **helper**, not an authority. Project docs (`RULES.md`, `CONTRIBUTING.md`, `CLAUDE.md`, `docs/*`) take precedence over any AI suggestion.
- All AI‑generated code:
  - Must conform to project rules.
  - Must be reviewed by a human before merge.
  - Must include tests for any non‑trivial behavior, especially bug fixes.

## Workflow

For new features:

1. Define the goal and constraints in prose (e.g., in an issue or DEVNOTES).
2. Ask AI to generate code within the existing structure and rules.
3. Run the automated tool pipeline (format, clippy, tests, security tools).
4. Review and adjust code manually as needed.

For bugs or regressions:

1. Write or extend tests that reproduce the failure.
2. Confirm tests fail.
3. Use AI to propose a fix.
4. Confirm tests pass and tools are clean.

AI output is treated as a draft, not as trusted code.

