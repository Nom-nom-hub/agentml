# AgentML Native Syntax

Native AgentML syntax is an experimental feature under development on the `feat/native-agentml-syntax` branch.

## Status

**Experimental**: Native syntax is on `main` and planned for v0.4.0 after stabilization. YAML-compatible syntax remains the default for v0.3.0.

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
  description "Project description"

  purpose {
    human_goal "For humans"
    agent_goal "For agents"
    non_goals ["Do not do X"]
  }
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

Optional. Contains read, write, and execute paths.

### `safety` block

Optional. Contains forbidden_actions, require_approval, and other safety rules.

### `validation` block

Optional. Contains validation commands.

### `output` block

Optional. Contains required output sections.

## Lists

Use brackets for lists:

```agentml
stack: ["Rust", "CLI"]

read: [
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

## Using Native Syntax

### Validate a native file

```bash
agentml validate AGENT.agent --format native
```

### Initialize with native syntax

```bash
agentml init --template rust-cli --syntax native
```

Available templates:
- `generic` - Generic project
- `rust-cli` - Rust CLI application
- `nextjs-app` - Next.js application
- `python-package` - Python package
- `node-package` - Node.js/NPM package

### Convert from YAML to native

```bash
agentml convert --to native AGENT.agent
```

Options:
- `--write` - Write to file instead of stdout
- `--backup` - Create `.bak` file before overwriting

## Error Messages

Parser errors include line and column:

```txt
AgentML parse error at line 12, column 5:
  Expected closing `]` for permissions.write list.

Suggestion:
Add `]` after the final write path.
```

## Future

Native syntax will become the default in v0.4.0 after:

1. Parser is complete
2. Validation passes
3. Migration tooling is stable
4. Documentation is comprehensive