# AgentML v0.1.0 — Contract Language for AI Coding Agents

AgentML is a markup language and CLI for defining how AI agents understand, modify, validate, and report on software projects.

## What AgentML Is

AgentML provides:

- **`AGENT.agent`** — a structured contract file that lives in your repo root
- **`.skill` files** — reusable, installable AI capabilities
- **CLI commands** — `init`, `validate`, `inspect`, `run`, `context`, `skill`, `self-check`, `diff`, `doctor`
- **Validation engine** — 9 semantic rules with risk scoring (0-100)
- **JSON Schema** — machine-readable contract definition

## Why It Exists

AI coding agents are powerful but need guardrails. Without contracts:

- Agents edit files they shouldn't
- Destructive commands run without confirmation
- Sensitive files are modified blindly
- Validation steps are skipped
- Reports are inconsistent

AgentML solves this by making the contract **explicit, machine-readable, and enforced by the CLI**.

## Install from Source

```bash
git clone https://github.com/Nom-nom-hub/agentml.git
cd agentml && cargo install --path .
```

Requirements: Rust 1.70+, Cargo.

## Quickstart

```bash
# 1. Install
cargo install --path .

# 2. Initialize in your project
cd your-project
agentml init --template generic

# 3. Validate
agentml validate AGENT.agent

# 4. Self-check
agentml self-check

# 5. Export context
agentml context
```

That is the entire workflow. You now have an `AGENT.agent` file governing your project.

## Main Commands

```bash
agentml init [path] [--template <generic|rust-cli|nextjs-app|python-package>] [--force]
agentml validate <file> [--strict]
agentml inspect <file>
agentml run <task> [file]
agentml context [file] [--output <path>]
agentml skill validate <file>
agentml skill pack <folder>
agentml self-check
agentml diff
agentml doctor
```

## Dogfooding Proof

AgentML uses AgentML to govern its own development.

- `AGENT.agent` — project-level execution contract
- `skills/*.skill` — 4 validated reusable skills
- `agentml self-check` — validates the project against its own contract
- `.github/workflows/agentml-self-check.yml` — enforces dogfooding in CI

Run it:

```bash
cargo run -- self-check
```

Expected result:

```
══ AgentML Self-Check ══

Contract: AGENT.agent
Status: valid

Skills:
  agentml-validator.skill: valid
  rust-cli-maintainer.skill: valid
  spec-writer.skill: valid
  release-auditor.skill: valid

Safety:
  ✔ forbidden_paths
  ✔ destructive_actions_policy
  ✔ secrets_policy

Validation:
  fmt: cargo fmt -- --check
  clippy: cargo clippy --all-targets -- -D warnings
  test: cargo test
  self_validate: cargo run -- validate AGENT.agent

Result:
  Dogfood status: PASS
  Risk score: 5/100
```

## Known Limitations

- AgentML is a contract layer, not a sandbox or runtime
- The CLI does not execute arbitrary commands in MVP. It validates, inspects, and plans.
- YAML is the MVP format. Future versions may add native syntax.
- No built-in agent runtime integration yet.
- No `extends` or skill composition UI yet.

## Links

- **Repository:** https://github.com/Nom-nom-hub/agentml
- **Documentation:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **Quickstart:** [docs/quickstart.md](docs/quickstart.md)
- **Spec:** [docs/spec.md](docs/spec.md)
- **Security:** [docs/security.md](docs/security.md)
- **Templates:** [docs/templates.md](docs/templates.md)
- **CI:** [docs/ci.md](docs/ci.md)

## License

MIT — see [LICENSE](LICENSE).
