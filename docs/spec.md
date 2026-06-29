---
layout: default
title: Specification
description: AgentML language specification v0.1.0.
---

# AgentML Specification v0.1.0

AgentML (Agent Markup Language) is an AI-native language for defining execution contracts between humans and AI agents working on software projects.

## Design Principles

1. **Executable semantics** — not just documentation, but enforceable rules
2. **AI-native** — optimized for LLM consumption and compliance
3. **Human-readable** — still inspectable by developers
4. **Portable** — works with any agent runtime, not vendor-specific
5. **Composable** — `.skill` files make capabilities reusable across projects

## File Types

### `AGENT.agent`

The primary execution contract for a project. Must be placed at the repository root.

### `.skill`

A reusable, installable capability. Examples: `auth.skill`, `testing.skill`, `rust-cli.skill`, `nextjs-app.skill`, `payment.skill`, `db-migration.skill`.

## AGENT.agent Schema

```yaml
meta:
  name: string
  version: string
  description?: string

purpose: string

context:
  project_type?: string
  languages?: string[]
  frameworks?: string[]

permissions:
  read: string[]
  write?: string[]
  execute?: string[]

tools?: string[]

workflows?:
  - name: string
    description?: string
    steps:
      - name: string
        description?: string
        commands?: string[]
        success?: string
        on_failure?: string

tasks?:
  - name: string
    description?: string
    workflow?: string
    inputs?:
      key:
        description: string
        required?: boolean
        default?: string
    success?: string

memory?: string # Policy for agent memory management

safety:
  policy?: string
  forbidden_paths: string[]
  forbidden_actions: string[]
  require_confirmation?: string[]

validation:
  - name: string
    command: string
    description?: string

output:
  format: string
  required_sections?: string[]
```

### Field Descriptions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meta` | object | yes | Name, version, optional description |
| `purpose` | string | yes | What the agent is allowed/expected to do |
| `context` | object | no | Project type, languages, frameworks |
| `permissions.read` | string[] | yes | Glob patterns the agent may read |
| `permissions.write` | string[] | no | Glob patterns the agent may write; broad patterns require `dangerous` marker |
| `permissions.execute` | string[] | no | Glob patterns for executable commands |
| `tools` | string[] | no | Available tools (e.g. `npm`, `git`, `cargo`) |
| `workflows` | array | no | Multi-step procedures with success/failure gates |
| `tasks` | array | no | Reusable named tasks |
| `memory` | string | no | Policy for what the agent should remember |
| `safety.policy` | string | no* | Overarching safety policy |
| `safety.forbidden_paths` | string[] | yes | Paths the agent must never touch |
| `safety.forbidden_actions` | string[] | yes | Actions the agent must never take |
| `safety.require_confirmation` | string[] | no | Destructive actions requiring human approval |
| `validation` | array | yes | Commands the agent must run after changes |
| `output.format` | string | yes | Expected output format (`markdown`, `json`, `yaml`) |
| `output.required_sections` | string[] | no | Required sections in the agent's report |

## `.skill` Schema

```yaml
skill: string
version: string
description: string
requirements?: string[]
inputs?:
  - name: string
    description: string
    required?: boolean
actions: string[]
rules?: string[]
success: string
output: string
```

## Validation Model

AgentML files are validated against:

1. **Structural validity** — YAML parseability and schema conformance
2. **Semantic completeness** — required fields present
3. **Safety compliance** — permissions and forbidden patterns are reasonable
4. **Risk assessment** — scoring based on permission breadth, missing guards, and policy gaps

### Error Codes

| Code | Severity | Description |
|------|----------|-------------|
| `MISSING_PURPOSE` | Error | No purpose field |
| `MISSING_PERMISSIONS` | Error | No permissions block |
| `UNSAFE_WRITE_PERMISSION` | Error | Broad write permission without `dangerous` marker |
| `MISSING_SAFETY` | Error | No safety block |
| `MISSING_VALIDATION` | Error | No validation commands |
| `STRICT_*` | Error | Violation in strict mode |
| `MISSING_SECRETS_POLICY` | Warning | No secrets handling policy |
| `DESTRUCTIVE_NO_CONFIRMATION` | Warning | Destructive action lacks confirmation gate |
| `MISSING_SUCCESS_CRITERIA` | Warning | No success criteria defined |
| `MISSING_OUTPUT` | Warning | No output requirements |

## Risk Scoring

| Factor | Points |
|--------|--------|
| Broad unsafe write permissions | +25 |
| Destructive action without confirmation | +5 |
| Missing secrets policy | +10 |
| Maximum score | 100 |

## Execution Model

The AgentML CLI does **not** execute arbitrary commands in MVP. It validates, inspects, and plans execution. Runtime execution is reserved for trusted, sandboxed agent runtimes.

### Workflow Execution

```
task -> workflow -> steps -> commands -> validation -> report
```

### Task Lifecycle

1. Agent receives task name
2. Resolves task → workflow mapping
3. Enumerates steps and prerequisites
4. Runs validation commands
5. Generates report matching `output.required_sections`
6. Enforces safety gates at each step

## Composability with `.skill`

Skills are self-contained capabilities:

```bash
agentml skill validate auth.skill
agentml skill pack ./my-skill-dir
```

Skills can specify:
- Required environment (`Next.js 14+`, `Rust 1引入`)
- Inputs with types and defaults
- Actions the skill performs
- Rules that must hold
- Success criteria
- Expected output format

When an `AGENT.agent` file references a skill (future: via `extends` or `skills` array), the agent inherits the skill's rules and action patterns.

## Future Extensions

- `extends` field in `AGENT.agent` to compose skills
- `AGENT.tasks` for project-specific task definitions
- `AGENT.state` for persistent agent memory
- `agentml run --dry-run` for full simulation
- `agentml diff` for permission diffing
- Plugin system for custom validators
- Integration with popular agent frameworks

## Compatibility

AgentML YAML is a strict subset of standard YAML. Any YAML parser can read it; only AgentML-aware tools enforce the semantic rules.
