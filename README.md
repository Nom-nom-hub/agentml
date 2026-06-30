# AgentML

[![CI](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml/badge.svg)](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-v0.1.2-blue)](https://github.com/Nom-nom-hub/agentml/releases/tag/v0.1.2)

**AgentML is a contract language for AI coding agents.**

AgentML defines how AI agents understand, modify, validate, and report on software projects. It is not Markdown, not YAML config — it is an executable contract that sits between humans and AI agents.

**Website:** https://nom-nom-hub.github.io/agentml/

---

## What AgentML Is

AgentML is a markup language and CLI for **AI execution contracts**. Every project can include an `AGENT.agent` file that tells agents:

- What they are allowed to do
- Which files they can read and write
- Which actions are forbidden
- Which commands must pass before changes are accepted
- What format their reports must follow

AgentML also supports `.skill` files for reusable, installable AI capabilities.

---

## Why It Exists

AI coding agents are powerful but need guardrails. Without contracts:

- Agents edit files they shouldn't
- Destructive commands run without confirmation
- Sensitive files are modified blindly
- Validation steps are skipped
- Reports are inconsistent

AgentML solves this by making the contract **explicit, machine-readable, and enforced by the CLI**.

---

## Quickstart

```bash
# 1. Install
cargo install agentml

# 2. Initialize in your project
cd your-project
agentml init --template generic

# 3. Validate the contract
agentml validate AGENT.agent

# 4. Run self-check
agentml self-check

# 5. Export context for LLMs
agentml context

# 6. Verify project structure
agentml doctor
```

**That is the entire workflow.** You now have an `AGENT.agent` file governing your project.

---

## Use AgentML in any repo

### Install from crates.io (recommended)

```bash
cargo install agentml
```

### Install from source

```bash
git clone https://github.com/Nom-nom-hub/agentml.git
cd agentml
cargo install --path .
```

### Initialize in your project

```bash
cd /path/to/your/project
agentml init --template generic

# Verify the contract
agentml self-check

# Export context for LLMs
agentml context

# Verify project structure
agentml doctor
```

### Next steps

- Read the [adoption guide](docs/adoption.md) to test AgentML in a real repo
- Copy [agent prompts](docs/agent-prompts.md) to make your AI agent respect the contract
- Choose a [template](docs/templates.md) for your stack
- Add [CI enforcement](docs/ci.md) to your pipeline
- Open a [feedback issue](https://github.com/Nom-nom-hub/agentml/issues/new?template=feedback.md)

### Templates

| Template | Command | Best for |
|----------|---------|----------|
| `generic` | `agentml init --template generic` | Any project |
| `rust-cli` | `agentml init --template rust-cli` | Rust CLI apps |
| `nextjs-app` | `agentml init --template nextjs-app` | Next.js + TypeScript |
| `python-package` | `agentml init --template python-package` | Python packages |

Each template generates:
- `AGENT.agent` — the execution contract
- `skills/` — reusable capabilities
- `.agentml/context.md` — LLM-readable context
- `docs/agentml.md` — project-specific docs
- `.github/workflows/agentml-check.yml` — CI enforcement

---

## Install

### From source

```bash
git clone https://github.com/Nom-nom-hub/agentml.git
cd agentml && cargo install --path .
```

### Requirements

- Rust 1.70+
- Cargo

---

## Basic Commands

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

---

## Example: AGENT.agent

```yaml
meta:
  name: my-project
  version: "1.0.0"

purpose: >
  AI agent for building and maintaining a Rust CLI application.

context:
  project_type: rust-cli
  languages: [rust]
  frameworks: [clap, tokio]

permissions:
  read:
    - "**/*.rs"
    - "**/Cargo.toml"
  write:
    - "src/**/*.rs"
    - "Cargo.toml"
  execute:
    - "cargo"

tools: [cargo, rustfmt, clippy, git, bash]

safety:
  forbidden_paths:
    - "target/**"
    - "**/*.rs.bk"
  forbidden_actions:
    - "cargo publish"
    - "rm -rf src"
  require_confirmation:
    - "cargo publish"

validation:
  - name: Format
    command: "cargo fmt -- --check"
  - name: Clippy
    command: "cargo clippy -- -D warnings"
  - name: Test
    command: "cargo test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
```

---

## Example: .skill file

```yaml
skill: rust-cli-maintainer
version: "1.0.0"
description: Maintain a Rust CLI project safely.

requirements:
  - Rust
  - Cargo

actions:
  - inspect_cli_commands
  - update_parser
  - run_cargo_checks

rules:
  - Keep CLI output stable unless intentionally changed.
  - Add tests for every new validation rule.
  - Avoid panics in user-facing parsing paths.

success: >
  cargo fmt passes, cargo clippy passes, cargo test passes.

output: >
  List changed modules, commands run, and test results.
```

---

## Dogfooding Proof

AgentML uses AgentML to govern its own development. This repository includes:

- `AGENT.agent` — the project-level AI execution contract
- `skills/*.skill` — reusable AgentML skills
- `agentml self-check` — validates the project against its own contract
- `.github/workflows/agentml-self-check.yml` — enforces dogfooding in CI

Run it yourself:

```bash
cargo run -- self-check
```

Expected output:

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

---

## CI Usage

Copy this workflow into your repo to enforce AgentML validation on every push:

```yaml
name: AgentML Check
on: [push, pull_request]
jobs:
  agentml:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test
      - run: cargo run -- validate AGENT.agent
      - run: cargo run -- skill validate skills/*.skill
      - run: cargo run -- self-check
```

---

## Security Model

AgentML enforces safety through four mechanisms:

1. **Permissions** — explicit read/write/execute path policies
2. **Forbidden paths** — paths the agent must never touch
3. **Forbidden actions** — commands the agent must never run
4. **Confirmation requirements** — destructive actions that need human approval

### Risk Levels

| Score | Level | Meaning |
|-------|-------|---------|
| 0-20 | Low | Minimal safety concerns |
| 21-50 | Medium | Some policy gaps or minor overlaps |
| 51-80 | High | Dangerous permissions or missing guards |
| 81-100 | Critical | Severe overlap or critical safety gaps |

The validator reports:
- **Errors** — contract is invalid; agent must not run
- **Warnings** — potential issues; agent should review
- **Risk score** — 0-100 numeric assessment
- **Suggested fixes** — how to resolve issues

---

## Limitations

- AgentML is **not** a sandbox or runtime. It is a contract layer.
- The CLI does **not** execute arbitrary commands in MVP. It validates, inspects, and plans.
- YAML is the MVP format. Future versions may add native syntax.
- No built-in agent runtime integration yet.
- No `extends` or skill composition UI yet.

---

## AgentML vs AGENTS.md / CLAUDE.md / Cursor rules

AgentML complements, but does not replace, existing project documentation formats.

| Format | Purpose | Strength | AgentML Difference |
|--------|---------|----------|-------------------|
| `AGENTS.md` | Loose Markdown instructions for agents | Human-readable, flexible | Unstructured; no validation or enforcement |
| `CLAUDE.md` | Model-specific project guidance | Tool-aware | Tied to one model; no portable contract format |
| Cursor rules | Editor-specific guidance | IDE-integrated | Editor-locked; no CLI, CI, or skill system |
| **AgentML** | **Structured agent execution contract** | **Validateable, enforceable, portable** | **Schema, risk scoring, skills, self-check, CI** |

AgentML is not anti-documentation. It is the **enforcement layer** that makes documentation actionable.

---

## Generated Context Example

Running `agentml context` produces `.agentml/context.md`:

```yaml
meta:
  name: my-project
  version: "1.0.0"

purpose: >
  AI agent for building and maintaining a Rust CLI application.

context:
  project_type: rust-cli
  languages: [rust]
  frameworks: [clap, tokio]

permissions:
  read:
    - "**/*.rs"
    - "**/Cargo.toml"
  write:
    - "src/**/*.rs"
  execute:
    - "cargo"

safety:
  forbidden_paths:
    - "target/**"
  forbidden_actions:
    - "cargo publish"
  require_confirmation:
    - "cargo publish"

validation:
  - name: Format
    command: "cargo fmt -- --check"
  - name: Clippy
    command: "cargo clippy -- -D warnings"
  - name: Test
    command: "cargo test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
```

Agents should read this context before making changes. It is the machine-readable version of your project's ground rules.

See [docs/agent-prompts.md](docs/agent-prompts.md) for copy-paste prompts to make your agent respect the contract.

---

## Roadmap

- [ ] Native AgentML syntax (not just YAML)
- [ ] `extends` field for skill composition
- [ ] `agentml run --dry-run` for full simulation
- [ ] `agentml diff` for permission diffing
- [ ] Plugin system for custom validators
- [ ] Integration with popular agent frameworks (Claude, GPT-4, Codex)
- [ ] Registry for community `.skill` packages

---

## Feedback Wanted

AgentML is young and your experience matters. Help us improve by answering:

1. **Did AgentML make your coding agent safer?** Did the contract prevent bad behavior?
2. **Did `.agentml/context.md` help your agent understand the repo?** Was the generated context clear and useful?
3. **Was the `AGENT.agent` file clear?** Were the sections easy to write and understand?
4. **What template do you need next?** We have `generic`, `rust-cli`, `nextjs-app`, and `python-package`. What other stacks should we support?
5. **What command felt confusing?** Which CLI command or output was hard to use?

Open a feedback issue: https://github.com/Nom-nom-hub/agentml/issues/new?template=feedback.md

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security Policy

See [SECURITY.md](SECURITY.md).

## License

MIT — see [LICENSE](LICENSE).
