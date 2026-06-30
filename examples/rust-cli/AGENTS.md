# AGENTS.md

## Purpose

This project uses AgentML to define how AI coding agents should safely work in
this Rust CLI repository. The machine-readable source of truth is `AGENT.agent`.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`

Recommended command:

```bash
agentml brief
```

If AgentML is not installed, read `AGENT.agent` directly.

## Project context

### Stack

- **Language**: Rust
- **Frameworks**: Clap (CLI parsing), Anyhow (error handling), Thiserror (error
  derive macros)

### Important files

- `**/*.rs` — Rust source and test files
- `**/Cargo.toml` — Project manifests
- `**/*.md` — Documentation
- `**/*.toml` — Config files
- `**/*.lock` — Lock files (Cargo.lock)
- `tests/**` — Integration tests

## Allowed work areas

Agents may modify:

- `src/**/*.rs`
- `tests/**/*.rs`
- `Cargo.toml`
- `docs/**`

## Forbidden areas

Agents must not modify or expose:

- `.env*` — Secrets and environment variables
- `target/**` — Build artifacts
- `**/*.rs.bk` — Rust backup files

## Validation commands

Before reporting completion, run:

- `cargo fmt -- --check` — Format check
- `cargo clippy --all-targets -- -D warnings` — Lint check
- `cargo test` — Test suite
- `cargo doc --no-deps` — Documentation check

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
