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

Never report `Risk score: N/A` if `agentml diff` was successfully run. Include the actual score and status.

## Final report format

Every agent task should end with:

```
Summary:
Files changed:
Commands run:
Validation result:
Risk score:
Commit:
Risks:
Next steps:
```

The `Commit:` field should contain the commit hash and short message. If not committed, state why:

## Source of truth

If files disagree, follow this order:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`
4. `AGENTS.md`
5. `README.md`

## Maintenance Intelligence

Agents must keep the project synchronized. When behavior changes, update every surface affected by that behavior.

Before reporting completion, ask:

1. Did CLI behavior change?
2. Did validation behavior change?
3. Did MCP behavior change?
4. Did generated file behavior change?
5. Did project templates change?
6. Did public documentation become outdated?
7. Did the website need a matching update?
8. Did examples need to be updated?
9. Did tests cover the new behavior?
10. Did CHANGELOG.md need an entry?

If yes, update the matching files before reporting completion.

## Documentation Sync Rule

Code changes are incomplete if user-facing documentation is stale.

When user-facing behavior changes, update documentation in the same task.

User-facing behavior includes:

- CLI commands
- command flags
- command output
- validation errors
- generated files
- templates
- MCP tools
- install flow
- examples
- website copy
- security behavior
- release process

Do not report completion if docs are knowingly outdated.

## Agent Self-Update Rule

Agents may update `AGENTS.md` when project workflow guidance changes.

Update `AGENTS.md` when:

- new commands are added
- agent workflow changes
- validation requirements change
- MCP usage changes
- final report expectations change
- source-of-truth order changes
- docs synchronization rules change
- release process changes
- project maintenance expectations change

Do not update `AGENTS.md` for unrelated code changes.

When updating `AGENTS.md`, keep it concise, accurate, and aligned with `AGENT.agent`.

## Pre-Final Checklist

Before reporting completion, verify:

- Code changes are complete.
- Tests were added or updated when needed.
- Validation commands were run (or `agentml close`).
- `agentml diff` was run (or `agentml close`).
- Risk score is included in the final report.
- `README.md` is updated if public behavior changed.
- `docs/` are updated if behavior or architecture changed.
- `CHANGELOG.md` is updated if user-facing behavior changed.
- Website content is updated if public messaging changed.
- Examples are updated if templates or expected usage changed.
- `AGENT.agent` is updated if contract rules changed.
- `AGENTS.md` is updated if agent workflow changed.
- `git status --short` was checked.
- Unrelated user changes were not included.
- All intended changes were committed unless the user explicitly said not to commit.
- Final report includes the commit hash.

## Git Workflow

Before finishing, check:

```bash
git status --short
agentml diff
```

If the task is complete and validation passes, commit the intended changes unless the human asked you not to.

Do not commit secrets, build artifacts, dependency folders, or unrelated user changes.

Final reports should include the commit hash when a commit is created.

## Task Closure Rule

A task is not complete until the repository is left in a clear final state.

For normal implementation tasks, that means:

1. Code/docs/tests are updated.
2. Required validation commands pass.
3. `agentml diff` has been run.
4. Changes are committed.
5. Final report includes commit hash and risk score.

After committing changes, run:

```bash
agentml close
git status --short
```

Final reports must reflect the post-commit repository state. If `agentml close` was run before committing, run it again after committing.

If changes are intentionally left uncommitted, the final report must clearly say:

- why they were not committed
- which files remain modified
- what command the user should run next
