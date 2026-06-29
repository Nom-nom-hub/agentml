# Security Model

AgentML is designed to make AI agent behavior **predictable, auditable, and safe**. This document explains the security model.

## Threat Model

AgentML assumes the following:

- **AI agents are trusted but fallible.** They follow instructions but can misinterpret context.
- **Human operators want guardrails.** They want to approve changes before they happen.
- **Secrets and sensitive files must never leak.** Contracts must prevent credentials from being read or output.
- **Destructive actions need gates.** `rm -rf`, `git push --force`, and similar commands require explicit confirmation.

## Permission Model

### Three-Tier Access Control

Every `AGENT.agent` defines three permission tiers:

| Tier | Purpose | Example |
|------|---------|---------|
| `read` | Files the agent may read | `**/*.rs`, `**/*.md` |
| `write` | Files the agent may modify | `src/**/*.rs`, `docs/**` |
| `execute` | Commands the agent may run | `cargo`, `npm run` |

### Path Patterns

AgentML uses glob patterns for path matching:

- `**/*.rs` — all Rust files anywhere in the project
- `src/**` — everything under `src/`
- `docs/**/*.md` — all Markdown files under `docs/`
- `!` negation is not supported in MVP; use `forbidden_paths` instead

### Broad Write Block

Writing to broad paths like `/**`, `**/*`, or `.` is **forbidden** unless the write permission is explicitly marked `dangerous`. This prevents agents from accidentally trashing a project.

## Safety Rules

### Forbidden Paths

Paths listed in `safety.forbidden_paths` are **never writable**. If a write permission overlaps with a forbidden path, validation fails with `FORBIDDEN_PATH_WRITE_OVERLAP`.

Common forbidden paths:

```yaml
safety:
  forbidden_paths:
    - ".env"           # Environment files with secrets
    - ".git/**"        # Git internals
    - "target/**"      # Build artifacts
    - "node_modules/**" # Dependencies
    - "dist/**"        # Generated output
    - "**/*secret*"    # Any file with "secret" in the name
    - "**/*credential*" # Any file with "credential" in the name
```

### Forbidden Actions

Actions listed in `safety.forbidden_actions` are commands the agent must never execute. The validator checks for destructive patterns:

```yaml
safety:
  forbidden_actions:
    - "rm -rf"
    - "git reset --hard"
    - "git clean -fd"
    - "cargo publish"
    - "docker system prune"
```

### Confirmation Requirements

Destructive commands that are forbidden should also have confirmation requirements:

```yaml
safety:
  forbidden_actions:
    - "rm -rf"
    - "git push --force"
  require_confirmation:
    - "git push"
    - "npm run db:migrate"
```

The validator warns if a destructive action lacks a confirmation gate.

### Secrets Policy

Every contract should define how secrets are handled:

```yaml
safety:
  secrets_policy:
    never_read:
      - ".env"
      - "*.pem"
      - "*.key"
    never_output_secret_values: true
```

The validator warns if no secrets policy is present.

## Risk Scoring

Risk is calculated as a 0-100 score based on:

| Factor | Points |
|--------|--------|
| Broad unsafe write permission | +25 |
| Forbidden path write overlap | +15 |
| Missing secrets policy | +10 |
| Destructive action without confirmation | +5 |
| Current risk | `min(total, 100)` |

### Risk Levels

| Range | Level | Action |
|-------|-------|--------|
| 0-20 | Low | Proceed with normal review |
| 21-50 | Medium | Require extra validation |
| 51-80 | High | Block automated changes; require human sign-off |
| 81-100 | Critical | Do not run; contract must be fixed |

## What AgentML Does NOT Do

- AgentML is **not** a sandbox. It does not execute commands.
- AgentML is **not** a secrets scanner. It checks policies but does not scan file contents.
- AgentML is **not** an IaC tool. It does not enforce infrastructure boundaries.
- AgentML is **not** a replacement for code review. It is a contract layer that augments review.

## Best Practices

1. **Keep write permissions narrow.** Prefer `src/**/*.rs` over `**/*`.
2. **List all sensitive paths.** If it contains secrets, put it in `forbidden_paths`.
3. **Add confirmation requirements for destructive actions.** Even if the action is forbidden, require confirmation for the pattern.
4. **Document your secrets policy.** Tell agents how to handle credentials.
5. **Run `self-check` in CI.** Make the contract enforceable.
6. **Review risk scores.** A score above 20 means your contract has gaps.

## Reporting Security Issues

See [SECURITY.md](SECURITY.md).
