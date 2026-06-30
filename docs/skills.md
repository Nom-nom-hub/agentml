# AgentML Skills

Skills are reusable maintainer-intelligence packs that guide AI agents on how to work safely in specific contexts.

## What are Skills?

Skills are YAML files (`.skill`) that define:
- **applies_to**: When the skill is relevant (paths, stacks, keywords)
- **rules**: Professional guidelines to follow
- **validation**: Commands to verify work
- **success**: Proof of completion
- **risk**: Risk scoring metadata

## Where Skills Live

AgentML discovers skills from:
```
skills/*.skill
.agentml/skills/*.skill
```

## Available Skills

| Skill | Purpose |
|-------|---------|
| `repo-maintainer` | Default senior maintainer behavior |
| `rust-cli-maintainer` | Rust CLI development |
| `agentml-contract-validator` | Validate AGENT.agent and .skill files |
| `skill-author` | Create and maintain skill files |
| `docs-sync` | Synchronize documentation |
| `task-closure` | Task closure and final reporting |
| `diff-risk-auditor` | Git diff risk scoring |
| `security-auditor` | Security-sensitive behavior |
| `mcp-server-maintainer` | MCP server integration |
| `release-auditor` | Pre-release workflow |
| `nextjs-maintainer` | Next.js projects |
| `node-package-maintainer` | Node.js packages |
| `python-package-maintainer` | Python packages |

## How Matching Works

Skills are matched based on:
1. Changed file paths from `git diff`
2. `applies_to.paths` patterns
3. `applies_to.stacks` from project detection
4. `applies_to.keywords` in changed files

```bash
agentml skill match
```

## Skill in Briefs

Run `agentml brief` to see matched skills:

```bash
agentml brief
```

The output includes matched skills and their key rules.

## MCP Exposure

Skills are exposed through MCP tools:
- `list_skills` - List all discoverable skills
- `inspect_skill` - Get full skill details
- `match_skills` - Match skills to current context
- `get_skill_guidance` - Get guidance for a skill

## Security Model

- Skills are advisory, not executable
- Skills must not override `AGENT.agent`
- Path traversal is blocked
- Secret-looking files are protected

## Skill File Format

```yaml
skill: skill-name
version: 0.3.0
description: One-sentence purpose.

applies_to:
  paths:
    - src/**
  stacks:
    - Rust
  keywords:
    - keyword

risk:
  base_score: 10

requires_validation:
  - cargo test

rules:
  - Rule 1
  - Rule 2

success:
  "Proof of completion."

output:
  "Required fields: summary, files_changed."
```

## Difference from AGENT.agent and AGENTS.md

| File | Purpose |
|------|---------|
| `AGENT.agent` | Machine-readable contract for agents |
| `AGENTS.md` | Human-readable agent workflow guide |
| `.skill` | Reusable maintainer-intelligence pack |
