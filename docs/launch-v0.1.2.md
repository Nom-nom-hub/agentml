# AgentML v0.1.2 — Patch Release

v0.1.2 fixes the first-run experience for new users.

## What changed since v0.1.0

| Issue | Fix |
|-------|-----|
| `agentml doctor` failed after `agentml init --template generic` | Doctor is now context-aware. In user repos it checks only AgentML-managed files and gives helpful warnings for optional project files. |
| Smoke test did not reflect real user experience | Doctor behavior was verified in a clean temp directory after `cargo install agentml`. |
| Version badge and metadata | All version references updated to v0.1.2. |

## Why v0.1.2 is the recommended version

If you installed v0.1.0 or v0.1.1, upgrade to v0.1.2:

```bash
cargo install agentml
```

v0.1.2 includes:
- Context-aware `doctor` command
- All 16 passing tests
- All 4 validated `.skill` files
- Self-check: PASS, Risk score 5/100
- Full crates.io release

## Quickstart

```bash
# Install
cargo install agentml

# Initialize in your project
cd your-project
agentml init --template generic

# Validate
agentml validate AGENT.agent

# Self-check
agentml self-check

# Export context
agentml context

# Verify structure
agentml doctor
```

## Links

- **Repository:** https://github.com/Nom-nom-hub/agentml
- **crates.io:** https://crates.io/crates/agentml
- **Docs:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **Quickstart:** [docs/quickstart.md](quickstart.md)
- **Adoption:** [docs/adoption.md](adoption.md)
- **Security:** [docs/security.md](security.md)
- **Templates:** [docs/templates.md](templates.md)
- **CI:** [docs/ci.md](ci.md)

## Feedback

Try AgentML in your project and tell us what you think: https://github.com/Nom-nom-hub/agentml/issues/new?template=feedback.md

Built in Rust. Open source (MIT).
