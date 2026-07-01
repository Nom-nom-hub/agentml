# Native AgentML Examples

This directory contains examples of AgentML's native syntax, which is currently in development on the `feat/native-agentml-syntax` branch.

## Files

- `AGENT.agent` - Rust CLI maintainer contract in native syntax
- `rust-cli-maintainer.skill` - Skill file in native syntax

## Status

Native syntax features:
- [x] Lexer tokenizing identifiers, strings, numbers, booleans, tokens
- [x] Parser for agent and skill blocks with nested fields
- [x] AST types for AgentAst and SkillAst
- [x] Auto-detection for native vs YAML syntax
- [x] Native examples validate
- [ ] Format flags (`--format native/yaml/auto`)
- [ ] Native init support
- [ ] Conversion command

## Usage

Validate a native file:

```bash
agentml validate AGENT.agent --format native
```

Initialize with native syntax:

```bash
agentml init --template rust-cli --syntax native
```

Convert from YAML:

```bash
agentml convert --to native AGENT.agent
```