# AgentML

**AI-native markup language and CLI for defining how AI agents understand, modify, validate, and report on software projects.**

AgentML is not Markdown. It is not plain YAML. Markdown is for humans. YAML is for configuration. AgentML is for **AI execution contracts**.

Every project can include an `AGENT.agent` file. This file defines:

- **Purpose** — what the agent is allowed to do
- **Context** — project type, languages, frameworks
- **Permissions** — read/write/execute path policies
- **Tools** — available CLIs and runtimes
- **Workflows** — multi-step procedures with success criteria
- **Tasks** — reusable named tasks with inputs and gates
- **Safety** — forbidden paths, forbidden actions, confirmation requirements
- **Validation** — commands the agent must run after changes
- **Output** — required sections and format for reports

AgentML also supports reusable `.skill` files for installable AI capabilities (e.g. `auth.skill`, `testing.skill`, `rust-cli.skill`).

---

## Install

```bash
cargo install --path agentml
```

Or clone and build:

```bash
git clone https://github.com/agentml/agentml.git
cd agentml && cargo build --release
```

---

## Quick Start

```bash
# Initialize a new project
agentml init

# Validate an AGENT.agent file
agentml validate

# Inspect an AGENT.agent file
agentml inspect

# Run a specific task
agentml run <task>

# Export agent context for LLMs
agentml context

# Work with skills
agentml skill validate auth.skill
agentml skill pack ./my-skill-folder
```

---

## CLI Reference

### `agentml init`

Creates a new `AGENT.agent` file in the current directory.

```bash
agentml init [path] [--template <basic|nextjs|rust>]
```

### `agentml validate`

Validates an `AGENT.agent` file against semantic rules and prints a report.

```bash
agentml validate <file> [--strict]
```

**Report includes:**
- `valid` / `invalid`
- `errors` with suggested fixes
- `warnings` with suggestions
- `risk_score` (0-100)
- `risk_factors`

### `agentml inspect`

Displays a human-readable summary of an `AGENT.agent` file.

```bash
agentml inspect <file>
```

### `agentml run <task>`

Shows the execution plan for a specific task defined in `AGENT.agent`.

```bash
agentml run <task> [file=AGENT.agent]
```

### `agentml context`

Exports the agent file as YAML context for consumption by LLMs.

```bash
agentml context [file=AGENT.agent] [--output <path>]
```

### `agentml skill validate <file>`

Validates a `.skill` file.

```bash
agentml skill validate <file>
```

### `agentml skill pack <folder>`

Packages all `.skill` files in a folder into a `.skill.tar.gz` archive.

```bash
agentml skill pack <folder>
```

---

## Agent File Example

```yaml
meta:
  name: my-nextjs-app
  version: "1.0.0"

purpose: >
  AI agent for building and maintaining a Next.js application with Supabase auth.

context:
  project_type: nextjs
  languages: [typescript, javascript]
  frameworks: [nextjs, react]

permissions:
  read:
    - "**/*.ts"
    - "**/*.tsx"
    - "**/*.json"
  write:
    - "src/**/*.ts"
    - "app/**/*.tsx"
  execute:
    - "npm run"
    - "npx"

safety:
  policy: "Never commit secrets. Use environment variables."
  forbidden_paths:
    - ".env*.local"
    - "node_modules/**"
  forbidden_actions:
    - "git push --force"
    - "rm -rf src"
  require_confirmation:
    - "git push"

validation:
  - name: Lint
    command: "npm run lint"
  - name: Type Check
    command: "npm run typecheck"
  - name: Test
    command: "npm test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
```

---

## Skill File Example

```yaml
skill: nextjs-auth
version: "1.0.0"
description: >
  Reusable skill for adding Supabase authentication to Next.js applications.

requirements:
  - "Next.js 14+"
  - "Supabase project"

inputs:
  - name: supabase_url
    description: "Supabase project URL"
    required: true
  - name: supabase_anon_key
    description: "Public anon key"
    required: true

actions:
  - "Create src/middleware.ts with auth guard"
  - "Create app/login/page.tsx"
  - "Update app/layout.tsx with SessionProvider"

rules:
  - "Never expose service_role key in client code"
  - "Use SSR cookies for server-side auth"

success: >
  User can sign up, log in, and access protected routes.

output: >
  Summary of files created, env vars required, and verification steps.
```

---

## Validation Rules

AgentML enforces these semantic rules:

1. **Must have `purpose`** — every file needs a clear purpose statement
2. **Must define `permissions`** — at minimum, `read` must be specified
3. **Must define `safety`** — including `forbidden_paths` and `forbidden_actions`
4. **Must define at least one `validation` command** — so the agent proves its work
5. **Must define success criteria** — in tasks or workflows
6. **Must define `output` requirements** — so the agent knows the expected report format
7. **Reject broad unsafe write permissions** — unless explicitly marked `dangerous`
8. **Warn if secrets policy is missing** — agents need guidance on handling credentials
9. **Warn if destructive command has no confirmation** — destructive actions need gates

---

## Project Structure

```
your-project/
├── AGENT.agent          # Core agent execution contract
├── .skills/
│   └── auth.skill       # Reusable capability
├── src/
└── package.json
```

---

## Philosophy

AgentML combines the best of:

- **README.md** — human-readable purpose and context
- **package.json** — structured metadata and commands
- **policy.yaml** — permissions and safety constraints
- **task runner** — workflows and validation gates
- **agent instruction** — everything an AI needs to work correctly

**One standard. One file. AI-native.**

---

## Dogfooding

AgentML uses AgentML to govern its own development.

This repository includes:

- `AGENT.agent` — the project-level AI execution contract
- `skills/*.skill` — reusable AgentML skills used to maintain this repo
- `agentml self-check` — validates the project against its own contract
- `.github/workflows/agentml-self-check.yml` — runs dogfood validation in CI

Run:

```bash
cargo run -- self-check
```

A passing self-check proves:

- the root contract is valid
- all local skills are valid
- safety policies are present
- forbidden paths are protected
- validation commands are defined
- docs and examples are present
