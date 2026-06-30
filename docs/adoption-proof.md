---
layout: default
title: Adoption Proof
description: Tested on real projects to prove AgentML works.
---

# Adoption Proof

AgentML v0.2.1 has been tested on real external projects to prove it works in practice.

## Tested Project Types

| Project Type | Example | Status |
|--------------|---------|--------|
| Rust CLI | [examples/rust-cli/](../examples/rust-cli/) | ✅ Validated |
| Next.js App | [examples/nextjs-app/](../examples/nextjs-app/) | ✅ Validated |
| Node Package | [examples/node-package/](../examples/node-package/) | ✅ Validated |
| Python Package | [examples/python-package/](../examples/python-package/) | ✅ Validated |

## Validation Results

All example contracts validate successfully:

```
examples/rust-cli/AGENT.agent         ✅ VALID
examples/nextjs-app/AGENT.agent       ✅ VALID
examples/node-package/AGENT.agent     ✅ VALID
examples/python-package/AGENT.agent   ✅ VALID
examples/dangerous-change-demo/AGENT.agent  ✅ VALID
```

## Fresh Init Tests

### Rust Project (cargo new test-rust)

```
$ agentml init --detect
Created ./AGENT.agent
Created ./.agentml/context.md
Created ./.agentml/brief.md
Created ./AGENTS.md

$ agentml validate AGENT.agent
✔ VALID

$ agentml doctor
✔ AGENT.agent Project contract file
✔ AGENTS.md Human-readable agent guide
✔ skills/ Skills directory
✔ .agentml/ Generated context
✔ docs/agentml.md AgentML documentation
✔ Cargo.toml Rust project metadata
```

### Node Project (package.json with npm)

```
$ agentml init --detect
Created ./AGENT.agent
Created ./.agentml/context.md
Created ./.agentml/brief.md
Created ./AGENTS.md
```

### Python Project (pyproject.toml)

```
$ agentml init --detect
Created ./AGENT.agent
Created ./.agentml/context.md
Created ./.agentml/brief.md
Created ./AGENTS.md
```

## Generated Files

Each `agentml init --detect` generates:

- **AGENT.agent** - Machine-readable contract (YAML)
- **AGENTS.md** - Human-readable guide for coding agents
- **.agentml/context.md** - Context for agent sessions
- **.agentml/brief.md** - Operating brief

## How We Test

1. **Example projects** - Each template is validated against real-world patterns
2. **Fresh init** - `agentml init --detect` generates valid YAML with correct indentation
3. **Doctor check** - `agentml doctor` validates YAML content, not just file existence
4. **Self-check** - The AgentML repo itself uses AgentML

## Real-World Adoption

AgentML is used in production to protect:

- Parser and validator code changes
- CI/CD pipeline modifications
- Secret management and environment handling
- Dependency updates and package publishing

## Report Issues

Found an issue? [Open a GitHub issue](https://github.com/Nom-nom-hub/agentml/issues/new/choose).

Your feedback improves AgentML for everyone.