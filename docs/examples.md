---
layout: default
title: Examples
---

# AgentML Examples

Real-world project contracts demonstrating what AgentML protects and how.

---

## Example Catalog

### [Rust CLI](../examples/rust-cli/)

A Rust command-line tool for file processing. Demonstrates:

- Protecting parser/validator changes with test requirements
- Blocking accidental `cargo publish`
- Forcing `cargo fmt`, `clippy`, and `test` before completion
- Forbidden build artifacts (`target/`, `.rs.bk`)

**Copy:** `cp examples/rust-cli/AGENT.agent ./AGENT.agent`

### [Next.js App](../examples/nextjs-app/)

A Next.js blog platform with authentication and CMS. Demonstrates:

- Safe AI frontend changes with typed validation
- Protecting `.env.local` and Vercel secrets
- Blocking destructive database migrations
- Requiring build verification before completion

**Copy:** `cp examples/nextjs-app/AGENT.agent ./AGENT.agent`

### [Node Package](../examples/node-package/)

An open-source Node.js utility library published on npm. Demonstrates:

- Preventing accidental `npm publish` without confirmation
- Protecting `.npmrc` and npm tokens
- Blocking version bumps without review
- Requiring lint, typecheck, test, and build

**Copy:** `cp examples/node-package/AGENT.agent ./AGENT.agent`

### [Python Package](../examples/python-package/)

A Python ML utility package published on PyPI. Demonstrates:

- Preventing accidental PyPI releases
- Protecting `.venv`, `dist/`, `build/`, and `.pypirc`
- Enforcing ruff, mypy, and pytest
- Blocking destructive commands

**Copy:** `cp examples/python-package/AGENT.agent ./AGENT.agent`

### [Dangerous Change Demo](../examples/dangerous-change-demo/)

A demonstration of AgentML catching dangerous AI agent behavior. Shows:

- An AI agent attempting to read `.env` secrets
- Modifying `src/validator.rs` without tests
- Running destructive commands
- Skipping validation
- Reporting incomplete results

**Final risk score: 100/100 (BLOCKED)**

See the full scenario in [examples/dangerous-change-demo/README.md](../examples/dangerous-change-demo/README.md).

---

## How to Use an Example

1. **Copy the contract** into your project:

```bash
cp examples/rust-cli/AGENT.agent ./AGENT.agent
```

2. **Customize** the `purpose`, `permissions`, and `validation` sections.

3. **Validate** the contract:

```bash
agentml validate AGENT.agent
```

4. **Generate AGENTS.md**:

```bash
agentml agents-md --write
```

5. **Generate the brief** for AI agents:

```bash
agentml brief --write
```

6. **Run a diff audit** after changes:

```bash
agentml diff
```

7. **Check project health**:

```bash
agentml doctor
```

---

## What Each Example Demonstrates

| Example | Protection | Risk If Missing |
|---------|-----------|-----------------|
| Rust CLI | Parser/validator safety, publish prevention | Accidental cargo publish, untested refactors |
| Next.js App | Env secrets, DB safety, build verification | Exposed API keys, broken builds |
| Node Package | npm token protection, release gates | Accidental package publication |
| Python Package | PyPI protection, build artifact safety | Accidental PyPI release, corrupted builds |
| Dangerous Demo | Full safety chain enforcement | Catastrophic AI agent failures |

---

## Running Validation

Each example includes a valid `AGENT.agent` contract. Validate any of them:

```bash
agentml validate examples/rust-cli/AGENT.agent
agentml validate examples/nextjs-app/AGENT.agent
agentml validate examples/node-package/AGENT.agent
agentml validate examples/python-package/AGENT.agent
agentml validate examples/dangerous-change-demo/AGENT.agent
```
