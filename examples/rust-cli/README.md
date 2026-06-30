# Rust CLI Example — AgentML

This example shows how AgentML protects a Rust CLI project. It includes a
pre-configured `AGENT.agent` that:

- Prevents cargo publish without confirmation
- Requires tests when `src/` files change
- Blocks dangerous commands (`rm -rf src`)
- Forbids writing to `target/` or `.env*`

## How AgentML protects parser/validator changes

When an agent edits a parser, validator, or critical logic in `src/`, the
`AGENT.agent` safety rules trigger:

- **Validator change (+25)**: Editing `validation` blocks in `AGENT.agent`
  raises the risk score.
- **Src without tests (+20)**: Modifying `src/**/*.rs` without touching
  `tests/**/*.rs` adds risk.
- **Forbidden path (+100)**: Touching `target/`, `.env*`, or `*.rs.bk` is an
  immediate high-risk flag.

These add up during `agentml diff` to give a clear risk score.

## Why tests are required when src changes

Any `src/` change has a risk of breaking behavior. The `AGENT.agent` write
permissions and diff auditing enforce that test files accompany source changes.
If the score is too high, a human must review before the agent proceeds.

## How diff risk scoring works

| Trigger | Points |
|---|---|
| Forbidden path access | +100 |
| Validator block change | +25 |
| `src/` edit without `tests/` edit | +20 |
| Cargo.toml change without Cargo.lock | +10 |

## How to use this example

1. Copy `AGENT.agent` to your project root.
2. Edit the `meta.name`, `purpose`, and `context` fields for your project.
3. Adjust `permissions.read` and `permissions.write` globs to match your repo.
4. Run `agentml validate AGENT.agent` to check the file.
5. Run `agentml brief` to see how the agent interprets your project.
6. Make changes and finish with `agentml diff` to see the risk score.
