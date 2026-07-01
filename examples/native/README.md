# Native AgentML Examples

This directory contains examples of AgentML's native syntax, which is currently in development on the `feat/native-agentml-syntax` branch.

## Files

- `AGENT.agent` - Rust CLI maintainer contract in native syntax
- `rust-cli-maintainer.skill` - Skill file in native syntax

## Status

Native syntax is experimental on main and planned for the v0.4.0 release after stabilization. YAML-compatible syntax remains the current public default in v0.3.0.

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