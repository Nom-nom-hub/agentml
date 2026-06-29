---
layout: default
title: Quickstart
description: Get AgentML running in 5 minutes.
---

# Quick Start

Get AgentML running in 5 minutes.

## 1. Install

### From crates.io (recommended)

```bash
cargo install agentml
```

### From source

```bash
git clone https://github.com/Nom-nom-hub/agentml.git
cd agentml && cargo install --path .
```

## 2. Initialize your project

```bash
cd your-project
agentml init --template generic
```

This creates `AGENT.agent` in your project root.

## 3. Validate the contract

```bash
agentml validate AGENT.agent
```

Expected output:

```
══ AgentML Validation Report ══

✔ VALID

Warnings:
  [MISSING_SECRETS_POLICY] No secrets policy defined in safety block
    Add policy describing how secrets are handled (env vars, vault, etc)

Risk Score: 10/100
```

## 4. Inspect the contract

```bash
agentml inspect AGENT.agent
```

Shows a human-readable summary of permissions, workflows, tasks, and safety rules.

## 5. Run a task

```bash
agentml run <task-name>
```

Displays the execution plan (workflow steps, commands, success criteria).

## 6. Export context for LLMs

```bash
agentml context
```

Generates `.agentml/context.md` — an LLM-readable version of your contract.

## 7. Validate skills

```bash
agentml skill validate skills/*.skill
```

## 8. Run self-check

```bash
agentml self-check
```

Validates the project against its own contract, checks all skills, and prints a dogfood report.

## What Just Happened?

You now have an `AGENT.agent` file that governs how AI agents work in your project. The file defines:

- What the agent is allowed to do
- Which files it can read/write/execute
- Which actions are forbidden
- Which validation commands must pass
- What output format reports must follow

Agents that respect AgentML contracts will read `AGENT.agent` before making changes, validate their work, and report in the required format.

## Next Steps

- Read the [spec](spec.md)
- Choose a [template](templates.md) for your stack
- Add [skills](examples/) to your project
- Set up [CI](ci.md) enforcement
