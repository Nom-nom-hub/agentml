# AgentML

[![CI](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml/badge.svg)](https://github.com/Nom-nom-hub/agentml/actions/workflows/agentml-self-check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-v0.1.3-blue)](https://github.com/Nom-nom-hub/agentml/releases/tag/v0.1.3)

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
agentml init [path] [--template <generic|rust-cli|nextjs-app|python-package>] [--detect] [--force]
agentml validate <file> [--strict]
agentml inspect
agentml run <task> [file]
agentml context [file] [--output <path>]
agentml brief [--format md|json] [--write] [--max-lines N] [--include-diff]
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