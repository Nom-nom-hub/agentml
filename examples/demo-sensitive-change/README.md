# AgentML Sensitive Change Demo

This demo shows how AgentML protects sensitive files from AI agents.

## Setup

```bash
cd examples/demo-sensitive-change
cargo run -- validate AGENT.agent
```

## The Scenario

This repo contains:

- `main.rs` — normal application code
- `config/auth.rs` — **sensitive authentication logic**
- `migrations/` — database migrations
- `AGENT.agent` — the AgentML contract

## Without AgentML

An AI agent could:

1. Edit `config/auth.rs` blindly
2. Run `rm -rf migrations/` without confirmation
3. Skip validation steps
4. Output inconsistent reports

## With AgentML

The `AGENT.agent` contract:

- **Restricts writes** to `src/main.rs` only
- **Forbids writes** to `config/**` and `migrations/**`
- **Forbids destructive actions** like `rm -rf`
- **Requires validation** after every change
- **Defines output format** for reports

## Expected Behavior

When you run:

```bash
cargo run -- validate AGENT.agent
```

You should see:

```
⚠  Warnings about missing validation commands (intentional for demo)
✔  No unsafe write permissions detected
✔  Sensitive paths are protected
```

## Key Contracts

Look at `AGENT.agent` and notice:

```yaml
permissions:
  write:
    - "src/main.rs"     # Only normal code is writable

safety:
  forbidden_paths:
    - "config/**"       # Auth logic is protected
    - "migrations/**"   # DB changes need human review
  forbidden_actions:
    - "rm -rf"
    - "git push --force"
```

## Try It

1. Run `agentml inspect AGENT.agent` — see the restrictions
2. Run `agentml run deploy` — see the workflow plan
3. Run `agentml self-check` — see the risk assessment

## Lesson

AgentML does not replace code review. It **enables** review by making the contract explicit.
