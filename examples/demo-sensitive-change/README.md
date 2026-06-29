# AgentML Sensitive Change Demo

**See how AgentML protects sensitive files from AI agents in under 2 minutes.**

---

## The Problem

Without a contract, AI agents treat every file the same. They can:

- Edit authentication logic without knowing it is sensitive
- Delete database migrations without backup
- Skip validation because no one told them to run it
- Output reports in whatever format they choose

This is not hypothetical. It happens every day in AI-assisted development.

---

## The AgentML Solution

AgentML makes the contract **explicit, validateable, and enforceable**.

### Before AgentML

```
Agent prompt: "Fix the login bug"

Agent actions (uncontrolled):
  ✏️  edits config/auth.rs          ← sensitive auth logic, no review
  ✏️  edits migrations/001_init.sql  ← DB schema, no backup check
  ✖️  skips tests                   ← no one required them
  📄 outputs: "done"               ← inconsistent format
```

### After AgentML

```
Agent reads AGENT.agent:
  ✓ Write permission: src/main.rs only
  ✗ Forbidden: config/**
  ✗ Forbidden: migrations/**
  ✓ Validation required: cargo test
  ✓ Output format: markdown with sections

Agent actions (controlled):
  ✏️  edits src/main.rs             ← allowed
  🚫 config/auth.rs                ← BLOCKED by contract
  🚫 migrations/                   ← BLOCKED by contract
  ✅ runs cargo test               ← required by contract
  📄 outputs structured report     ← enforced by contract
```

---

## Step-by-Step Demo

### 1. Inspect the contract

```bash
cd examples/demo-sensitive-change
cargo run -- inspect AGENT.agent
```

**Expected output:**
```
══ AgentML Inspection Report ══

Purpose: Demo showing how AgentML protects sensitive files from AI agents.

Permissions:
  Read (3 paths):
    - **/*.rs
    - **/*.md
    - AGENT.agent
  Write (1 paths):
    - src/main.rs

Safety:
  Forbidden Paths: 2
  Forbidden Actions: rm -rf, git push --force
```

**What this tells you:** The agent can only write to `src/main.rs`. It cannot touch `config/` or `migrations/`.

### 2. Validate the contract

```bash
cargo run -- validate AGENT.agent
```

**Expected output:**
```
══ AgentML Validation Report ══

✔ VALID

Warnings:
  [DESTRUCTIVE_NO_CONFIRMATION] Destructive action 'rm -rf' is forbidden but no confirmation requirement is enforced
    Add matching pattern to require_confirmation to prevent accidental execution

Risk Score: 15/100
```

**What this tells you:**
- The contract is valid
- Risk is **low** (15/100) because permissions are narrow and safety policies are present
- One warning: `rm -rf` is forbidden but lacks a confirmation gate

### 3. Run a task

```bash
cargo run -- run deploy AGENT.agent
```

**Expected output:**
```
Running task: deploy

Workflow: default_change
  1. inspect_contract
  2. plan_minimal_change
  3. edit_files
  4. run_validation
  5. report_result

Task execution plan displayed (MVP does not execute commands)
```

**What this tells you:** The agent has a clear execution plan. It knows what steps to take and what success looks like.

### 4. Run self-check

```bash
cargo run -- self-check
```

**Expected output:**
```
══ AgentML Self-Check ══

Contract: AGENT.agent
Status: valid

Safety:
  ✔ forbidden_paths
  ✔ destructive_actions_policy
  ✔ secrets_policy

Result:
  Dogfood status: PASS
  Risk score: 15/100
```

**What this tells you:** The contract passes its own validation. Sensitive paths are protected.

---

## Understanding Risk Score

Risk is calculated from four factors:

| Factor | Points | This Demo |
|--------|--------|-----------|
| Broad unsafe write permission | +25 | 0 — writes are narrow |
| Forbidden path write overlap | +15 | 0 — no overlap detected |
| Missing secrets policy | +10 | 0 — secrets policy present |
| Destructive action without confirmation | +5 | 5 — `rm -rf` lacks confirmation |
| **Total** | | **5** |
| **Risk level** | | **Low** |

### Risk Levels

| Score | Level | Action |
|-------|-------|--------|
| 0-20 | Low | Proceed with normal review |
| 21-50 | Medium | Require extra validation |
| 51-80 | High | Block automated changes; require human sign-off |
| 81-100 | Critical | Do not run; contract must be fixed |

---

## Sensitive Path Detection

AgentML detects sensitive paths through two mechanisms:

### 1. Explicit forbidden paths

In `AGENT.agent`:

```yaml
safety:
  forbidden_paths:
    - "config/**"       # Auth logic
    - "migrations/**"   # Database schema
```

The validator checks that no `write` permission overlaps with these paths.

### 2. Pattern-based detection

The validator warns on common sensitive patterns:

- `**/*secret*` — files with "secret" in the name
- `**/*credential*` — files with "credential" in the name
- `.env*` — environment files
- `*.pem`, `*.key` — cryptographic keys

---

## Key Contracts in This Demo

Look at `AGENT.agent`:

```yaml
permissions:
  read:
    - "**/*.rs"
    - "**/*.md"
    - "AGENT.agent"
  write:
    - "src/main.rs"       # ONLY normal code is writable

safety:
  forbidden_paths:
    - "config/**"         # Auth logic is PROTECTED
    - "migrations/**"     # DB changes need human review
  forbidden_actions:
    - "rm -rf"
    - "git push --force"
  require_confirmation:
    - "cargo publish"
  secrets_policy:
    never_read:
      - ".env"
      - "*.pem"
      - "*.key"
    never_output_secret_values: true

validation:
  - name: cargo check
    command: "cargo check"
  - name: cargo test
    command: "cargo test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "risks"
```

---

## Try It Yourself

```bash
# 1. Validate
cargo run -- validate AGENT.agent

# 2. Inspect
cargo run -- inspect AGENT.agent

# 3. Run task
cargo run -- run deploy AGENT.agent

# 4. Self-check
cargo run -- self-check
```

---

## Why Not Just AGENTS.md?

`AGENTS.md` gives agents loose instructions in Markdown. It is human-readable but:

- **No validation** — agents can ignore it
- **No enforcement** — nothing stops them from editing forbidden files
- **No risk scoring** — you don't know how safe your instructions are
- **No CI integration** — instructions drift over time
- **No skills** — reusable capabilities must be copy-pasted

AgentML turns those same instructions into a **structured contract** that is:

- **Validateable** — the CLI checks every rule
- **Enforceable** — CI runs `self-check` on every PR
- **Quantified** — risk score tells you how safe the contract is
- **Composable** — `.skill` files make capabilities reusable

AgentML is not anti-documentation. It is documentation that agents cannot ignore.

---

## Source Files

- `src/main.rs` — normal application code (writable)
- `config/auth.rs` — sensitive authentication logic (protected)
- `migrations/` — database migrations (protected)
