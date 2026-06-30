# AgentML

[![CI](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml/badge.svg)](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-v0.2.0-blue)](https://github.com/Nom-nom-hub/agentml/releases/tag/v0.2.0)

**AgentML is a contract language for AI coding agents.**

AgentML defines how AI agents understand, modify, validate, and report on software projects. It is not Markdown, not YAML config — it is an executable contract that sits between humans and AI agents.

**Website:** https://nom-nom-hub.github.io/agentml/

---

## Install

```bash
cargo install agentml
```

Requirements: Rust 1.70+, Cargo

---

## Quickstart

After installing, run these commands in an existing repo:

```bash
# 1. Inspect your project
agentml inspect

# 2. Initialize AgentML (auto-detects project type)
agentml init --detect

# 3. Validate the contract
agentml validate AGENT.agent

# 4. Generate operating brief
agentml brief --write

# 5. Check repo health
agentml doctor

# 6. Run self-check
agentml self-check

# 7. Audit changes
agentml diff

# 8. Use MCP server for agent integration
agentml mcp
```

---

## Basic Commands

```
agentml init [path] [--template <generic|rust-cli|nextjs-app|python-package>] [--detect] [--force] [--no-agents-md] [--no-context] [--no-brief]
agentml validate <file> [--strict]
agentml inspect
agentml run <task> [file]
agentml context [file] [--output <path>]
agentml brief [--format md|json] [--write] [--max-lines N] [--include-diff]
agentml agents-md [--write] [--force]
agentml close [--json] [--require-clean] [--fail-at-risk <N>] [--write-report]
agentml self-check
agentml diff
agentml doctor
agentml completions <bash|zsh|fish>
agentml version
agentml mcp
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

## AGENTS.md generation

AgentML generates both a machine-readable contract and a human-readable guide for coding agents:

- **`AGENT.agent`** — validated machine-readable contract (source of truth)
- **`AGENTS.md`** — human-readable guide for coding agents (generated from `AGENT.agent`)

### `agentml init --detect`

Creates `AGENTS.md` automatically alongside `AGENT.agent`, `.agentml/context.md`, and `.agentml/brief.md`.

Flags:
- `--no-agents-md` — skip AGENTS.md generation
- `--no-context` — skip .agentml/context.md
- `--no-brief` — skip .agentml/brief.md
- `--force` — overwrite existing files

### `agentml agents-md`

Reads `AGENT.agent` and generates `AGENTS.md` content.

```
agentml agents-md          # print to stdout
agentml agents-md --write  # write to AGENTS.md
agentml agents-md --write --force  # overwrite existing
```

Generated `AGENTS.md` includes: purpose, required first steps, stack, important files, allowed work areas, forbidden areas, validation commands, diff audit instructions, final report format, and source of truth hierarchy.

---

## Smart AGENTS.md

AgentML generates `AGENTS.md` as a human-readable guide for coding agents.

It tells agents:

- what files they can touch
- what files are forbidden
- what commands to run
- how to report completion
- when to update README, docs, website, examples, CHANGELOG, and AGENT.agent
- when to update AGENTS.md itself

This helps agents behave like maintainers instead of one-off code generators.

---

## Task closure

```bash
agentml close
```

Run before reporting completion. Runs all final checks and generates a structured closure report:

- Validates the contract
- Runs self-check
- Audits changes via diff
- Checks git status
- Generates a risk-scored completion report

Flags:

- `--json` — output as JSON
- `--require-clean` — fail if working tree is dirty
- `--fail-at-risk <N>` — fail if risk score is N or higher
- `--write-report` — write report to `.agentml/close-report.md`

---

## Use AgentML with coding agents

### Workflow

1. Agent calls `get_agent_brief` (via MCP or file)
2. Agent edits only allowed files
3. Agent calls `audit_diff` (via MCP)
4. Agent runs validation commands
5. Agent returns required final report

### MCP Server

The MCP server exposes AgentML tools to AI agents:

```bash
agentml mcp
```

See [docs/mcp.md](docs/mcp.md) for client configuration.

---

## Adoption Proof

AgentML has been tested on real projects to prove it works in practice.

| Project Type | Example | Status |
|--------------|---------|--------|
| Rust CLI | [examples/rust-cli/](examples/rust-cli/) | ✅ Validated |
| Next.js App | [examples/nextjs-app/](examples/nextjs-app/) | ✅ Validated |
| Node Package | [examples/node-package/](examples/node-package/) | ✅ Validated |
| Python Package | [examples/python-package/](examples/python-package/) | ✅ Validated |

See [docs/adoption-proof](docs/adoption-proof.md) for details.

---

## Examples

See AgentML applied to real projects:

- **[Rust CLI](examples/rust-cli/)** — Parser/validator safety, `cargo publish` prevention, fmt/clippy/test enforcement
- **[Next.js App](examples/nextjs-app/)** — Env secret protection, database safety, build verification
- **[Node Package](examples/node-package/)** — npm token protection, release gates, version bump safety
- **[Python Package](examples/python-package/)** — PyPI protection, ruff/mypy/pytest enforcement, build artifact safety
- **[Dangerous Change Demo](examples/dangerous-change-demo/)** — AgentML catching an AI agent attempting to read `.env`, modify validators without tests, skip validation, and run destructive commands

### See AgentML catch a dangerous AI change

In the [dangerous-change-demo](examples/dangerous-change-demo/), an AI agent attempts to:

1. Read `.env` (forbidden path) — **Blocked**
2. Modify `src/validator.rs` without tests — **Risk +45**
3. Run `rm -rf target/` (destructive action) — **Blocked**
4. Report completion without running validation — **Rejected**

**Result: Risk score 100/100 — BLOCKED**

AgentML prevented secret exposure, safety bypass, destructive cleanup, and false completion reporting.

---

## Security Model

AgentML enforces safety through four mechanisms:

1. **Permissions** — explicit read/write/execute path policies
2. **Forbidden paths** — paths the agent must never touch
3. **Forbidden actions** — commands the agent must never run
4. **Confirmation requirements** — destructive actions that need human approval

See [SECURITY.md](SECURITY.md) for the vulnerability disclosure policy.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

AgentML dogfoods its own contract. This repository includes `AGENT.agent` and `skills/*.skill` that govern the project itself.

## License

MIT — see [LICENSE](LICENSE).