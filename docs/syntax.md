# AgentML Native Syntax

Native AgentML syntax is an experimental feature under development on the `feat/native-agentml-syntax` branch.

## Status

**Experimental**: Native syntax is not yet stable. YAML-compatible syntax remains the default for v0.3.0.

## Purpose

Native syntax provides:

- AI-readable contracts and skills
- Human-friendly formatting
- Clear block structure
- Better error messages
- Less whitespace fragility than YAML

## Basic Structure

```agentml
agent "project-name" {
  version "0.4.0"
  contract_version 1
  description "Project description"
}

purpose {
  human_goal "For humans"
  agent_goal "For agents"
  non_goals [
    "Do not do X"
  ]
}
```

## Sections

### `agent` block

Required. Contains project identity.

### `purpose` block

Optional. Contains goals and non-goals.

### `context` block

Optional. Contains stack and important files.

### `permissions` block

Optional. Contains read, write, and forbidden paths.

### `validation` block

Optional. Contains validation commands.

## Lists

Use brackets for lists:

```agentml
stack ["Rust", "CLI"]

read [
  "**/*.rs"
  "**/*.md"
]
```

## Strings

Use double quotes:

```agentml
version "0.4.0"
description "My project"
```

## Comments

Start with `#`:

```agentml
# This is a comment
version "0.4.0"
```

## Error Messages

Parser errors include line and column:

```txt
AgentML parse error at line 12, column 5:
  Expected closing `]` for permissions.write list.

Suggestion:
Add `]` after the final write path.
```

## Conversion

Convert YAML to native:

```bash
agentml convert --to native AGENT.agent
```

## Future

Native syntax will become the default in v0.4.0 after:

1. Parser is complete
2. Validation passes
3. Migration tooling is stable
4. Documentation is comprehensive