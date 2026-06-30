# AGENTS.md

## Purpose

This project uses AgentML to define how AI coding agents should safely work in this repository.

The machine-readable source of truth is `AGENT.agent`.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`

Recommended command:

```bash
agentml brief
```

If AgentML is not installed, read AGENT.agent directly.

## Project context

### Stack

- Generic

### Important files

- **/*.rs
- **/*.agent
- **/*.skill
- **/*.md
- **/*.json
- Cargo.toml

## Allowed work areas

Agents may usually modify:

- src/**
- tests/**
- docs/**
- examples/**
- AGENT.agent
- agentml.schema.json
- README.md
- .github/workflows/*.yml
- Gemfile
- Cargo.toml
- Cargo.lock
- tests/**/*.rs

Always check AGENT.agent for the authoritative list.

## Forbidden areas

Agents must not modify or expose:

- .env
- .git/**
- target/**
- **/*secret*
- **/*credential*
- ~/.ssh/**

## Validation commands

Before reporting completion, run:

- `cargo fmt -- --check` — fmt
- `cargo clippy --all-targets -- -D warnings` — clippy
- `cargo test` — test
- `cargo run -- validate AGENT.agent` — self_validate

## Diff audit

After making changes, run:

```bash
agentml diff
```

Include the risk score in the final report.

## Final report format

Every agent task should end with:

```
Summary:
Files changed:
Commands run:
Validation result:
Risk score:
Risks:
Next steps:
```

## Source of truth

If files disagree, follow this order:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`
4. `AGENTS.md`
5. `README.md`
