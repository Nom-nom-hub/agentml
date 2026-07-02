# Native AgentML Examples

This directory contains examples of AgentML's native syntax, an experimental public feature introduced in v0.4.0.

## Files

- `AGENT.agent` - Rust CLI maintainer contract in native syntax
- `rust-cli-maintainer.skill` - Skill file in native syntax

## Status

Native syntax is an experimental public feature in v0.4.0. YAML-compatible syntax remains fully supported as the default format.

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