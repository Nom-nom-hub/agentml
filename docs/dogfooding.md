# AgentML Dogfooding

AgentML uses AgentML to govern its own development. This document explains what that means, how it works, and how future agents should use the project's self-contract.

## What is dogfooding for AgentML?

Dogfooding means the AgentML project does not just describe AgentML — it actively uses AgentML as the source of truth for:

- How agents are allowed to modify the repository
- What paths are safe to read and write
- What commands must pass before any change is accepted
- What output format proofs of completion must follow
- What constitutes acceptable risk

If AgentML cannot govern AgentML, it is not ready to govern other projects.

## The project contract: `AGENT.agent`

The root `AGENT.agent` file defines:

### Purpose

Two goals are captured:

- **human_goal** — build an AI-native markup language and CLI for agent execution contracts
- **agent_goal** — modify this repo safely, validate all changes, and report proof of completion

Non-goals explicitly prevent scope creep: AgentML must not become a general programming language, must not execute untrusted skills without validation, and must never store secrets or credentials.

### Stack and architecture

The contract records the Rust CLI stack (Serde, JSON Schema, GitHub Actions) and maps the four major components: parser, validator, reporter, CLI.

### Important files

The contract lists core implementation files so any agent knows which parts of the repo are load-bearing.

### Permissions

Agents working inside this repo may read Rust source, `.agent`, `.skill`, Markdown, JSON, and `Cargo.toml`. They may write to `src/**`, `tests/**`, `docs/**`, `examples/**`, and the contract files themselves. Forbidden files include `.env`, `.git/**`, `target/**`, and anything matching `*secret*` or `*credential*`.

### Safety

The contract requires explicit user approval for destructive actions (`rm -rf`, `git reset --hard`, `git clean -fd`, `cargo publish`). A secrets policy forbids reading `.env`, `*.pem`, and `*.key`, and forbids outputting secret values.

### Workflows

Two standard workflows are defined:

- **default_change** — inspect contract, plan minimal change, edit files, run validation, report result
- **release_check** — run fmt, clippy, tests, validate AGENT.agent, validate all skills

### Validation

Four commands must pass:

1. `cargo fmt --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. `cargo run -- validate AGENT.agent`

### Output

Every proof of completion must include: summary, files changed, commands run, validation result, risks, and next steps.

## Project skills

Reusable capabilities live in `skills/`. Each `.skill` file is itself validated AgentML syntax.

### `agentml-validator.skill`

Defines how the tool validates contracts. Covers parsing, required sections, permissions, safety policy, task validation, risk scoring, and report emission.

### `rust-cli-maintainer.skill`

Guides safe Rust CLI changes. Enforces stable CLI output, test coverage for new rules, panic-free parsing, and structured errors over `unwrap`.

### `spec-writer.skill`

Keeps `docs/spec.md` aligned with the implementation. Requires every required schema field to be documented, every validation rule to have an example, and no unsupported behavior promises.

### `release-auditor.skill`

Defines the pre-release proof. Blocks release if tests fail, if `AGENT.agent` fails validation, if examples fail validation, or if README install commands are wrong.

## The `agentml self-check` command

`agentml self-check` loads `AGENT.agent`, validates it, validates every `.skill` in `skills/`, checks that forbidden paths do not overlap with write permissions, confirms validation commands exist, checks that `docs/spec.md` and `README.md` mention dogfooding, and prints a dogfood report.

Example output:

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
  forbidden_paths: pass
  destructive_actions_policy: pass
  secrets_policy: pass

Validation:
  cargo fmt --check: listed
  cargo clippy: listed
  cargo test: listed
  agentml validate AGENT.agent: listed

Result:
  Dogfood status: PASS
  Risk score: 12/100
```

## Using `.agentml/context.md`

The `agentml context` command generates `.agentml/context.md` from `AGENT.agent`. Agents should read this file before making changes. It contains:

- Project purpose
- Architecture summary
- Important files
- Permissions
- Forbidden actions
- Validation commands
- Final report requirements

## What counts as proof of completion

A change is complete only when:

1. The contract (`AGENT.agent`) is still valid
2. All skills in `skills/` are still valid
3. The relevant validation commands pass (`cargo fmt --check`, `cargo clippy`, `cargo test`)
4. No forbidden file was modified
5. A final report follows the required output format (summary, files changed, commands run, validation result, risks, next steps)

## CI enforcement

GitHub Actions runs `agentml self-check` on every push and pull request. This means the project's contract is enforced continuously, not just when humans remember to run it.

## Future agents

If you are an AI agent working on this repository:

1. Read `.agentml/context.md` first.
2. Read `AGENT.agent` and `skills/*.skill`.
3. Make minimal changes.
4. Run `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo run -- self-check`.
5. Report using the required output format.

The contract is not optional. It is the source of truth.
