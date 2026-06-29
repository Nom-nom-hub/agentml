---
layout: default
title: Adoption Guide
description: How to test AgentML in a real project and give feedback.
---

# Adoption Guide

How to test AgentML in a real project and give feedback.

## Prerequisites

- Rust 1.70+ and Cargo installed
- A project you want to protect with an AgentML contract

## Step 1: Install AgentML

```bash
cargo install agentml
```

Verify:

```bash
agentml --version
# Should print agentml 0.1.2
```

## Step 2: Choose your flow

### A. Existing project

```bash
cd your-existing-project
agentml init --template generic
agentml validate AGENT.agent
agentml self-check
agentml context
agentml doctor
```

Edit `AGENT.agent` to match your stack:
- Update `permissions.read` and `permissions.write` for your file layout
- Add your validation commands (`npm test`, `pytest`, `cargo test`, etc.)
- Add your sensitive paths to `safety.forbidden_paths`
- Add destructive commands to `safety.forbidden_actions`

### B. Rust project

```bash
cd your-rust-project
agentml init --template rust-cli
agentml validate AGENT.agent
agentml self-check
```

The `rust-cli` template pre-fills:
- Permissions for `src/**/*.rs`, `Cargo.toml`, and Markdown
- Validation: `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`
- Safety rules for `target/**` and build artifacts

### C. Next.js project

```bash
cd your-nextjs-project
agentml init --template nextjs-app
agentml validate AGENT.agent
agentml self-check
```

The `nextjs-app` template pre-fills:
- Permissions for `app/**`, `src/**`, `*.ts`, `*.tsx`
- Validation: `npm run lint`, `npm run typecheck`, `npm test`
- Safety rules for `.env*.local`, `node_modules/**`

## Step 3: Give your agent the contract

The most important step: actually make your AI agent use the contract.

### Option 1: Paste context into the agent

Open `.agentml/context.md` and paste its contents at the start of every coding session.

### Option 2: Add a system prompt

Add this prompt to your agent's configuration (Cursor, Claude, etc.):

> Read `AGENT.agent` and `.agentml/context.md` before making changes.

See [docs/agent-prompts.md](agent-prompts.md) for more copy-paste prompts.

### Option 3: CI enforcement

Add AgentML validation to your CI pipeline. See [docs/ci.md](ci.md) for a copy-paste GitHub Actions workflow.

```yaml
- run: cargo run -- validate AGENT.agent
- run: cargo run -- self-check
```

## Step 4: Iterate

As your project changes:

1. Update `AGENT.agent` when you add new sensitive directories
2. Run `agentml validate AGENT.agent` after every contract change
3. Run `cargo run -- self-check` before merging
4. Watch the risk score. If it goes above 20, review your permissions.

## Step 5: Give feedback

Your experience makes AgentML better. Please share:

- Did AgentML make your agent safer?
- Did `.agentml/context.md` help?
- Was the contract clear?
- What template do you need?
- What was confusing?

Open a feedback issue: https://github.com/Nom-nom-hub/agentml/issues/new?template=feedback.md

## Common pitfalls

| Problem | Cause | Fix |
|---------|-------|-----|
| `agentml doctor` fails | Missing AgentML-managed files | Run `agentml init` or create the missing files manually |
| High risk score | Broad write permissions or overlaps | Narrow `write` patterns; ensure no overlap with `forbidden_paths` |
| Agent ignores contract | Context not loaded | Paste `.agentml/context.md` into every session or add a system prompt |
| Validation fails | Contract has errors | Read the error message; add missing required sections |
| Template not for my stack | Use `generic` and customize | Template are starting points, not limitations |

## Uninstall

```bash
cargo uninstall agentml
```
