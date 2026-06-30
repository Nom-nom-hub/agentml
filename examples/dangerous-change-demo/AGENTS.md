# AGENTS.md

## Purpose

This project demonstrates how AgentML prevents dangerous AI agent behavior in
a Rust codebase. The machine-readable source of truth is `AGENT.agent`.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`

## Validation commands

These commands are mandatory before reporting completion. Skipping any of them
is a safety violation and will cause agentml diff to block the report.

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run -- validate AGENT.agent`

## Forbidden paths

The following paths must never be read or written by the agent:

- `.env*` — contains secrets and API keys
- `target/**` — build artifacts
- `**/*secret*`
- `**/*credential*`

## Critical module

The validator module at `src/validator.rs` is the critical path. Any change to
this file MUST be accompanied by new tests in `tests/`. Changes without tests
add +45 risk (25 + 20) and will likely be blocked.

## Source of truth

If files disagree, follow this order:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`
4. `AGENTS.md`
5. `README.md`
