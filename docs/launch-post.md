# AgentML v0.1.0 is live.

AgentML is a contract language for AI coding agents.

Instead of giving agents loose Markdown instructions like `AGENTS.md` or `CLAUDE.md`, AgentML gives them a **structured project contract** with safe commands, sensitive paths, reusable skills, validation rules, risk scoring, generated context, self-checks, and CI enforcement.

## Why AgentML?

AI coding agents are powerful but need guardrails. Without contracts:

- Agents edit files they shouldn't
- Destructive commands run without confirmation
- Sensitive files are modified blindly
- Validation steps are skipped
- Reports are inconsistent

AgentML makes the contract **explicit, machine-readable, and enforced by the CLI**.

## Quickstart

```bash
# Install
cargo install --path .

# Initialize in your project
cd your-project
agentml init --template generic

# Verify
agentml validate AGENT.agent
agentml self-check
agentml context
```

That is the entire workflow. You now have an `AGENT.agent` file governing your project.

## Key Commands

```bash
agentml init --template <generic|rust-cli|nextjs-app|python-package>
agentml validate AGENT.agent
agentml inspect AGENT.agent
agentml run <task>
agentml context
agentml skill validate <file>
agentml self-check
```

## Dogfooding

AgentML uses AgentML to protect and validate its own repo.

- `AGENT.agent` — the project-level contract
- `skills/*.skill` — 4 validated reusable skills
- `agentml self-check` — validates the project against its own contract
- `.github/workflows/agentml-self-check.yml` — enforces dogfooding in CI

```bash
cargo run -- self-check
```

Result: **PASS**, Risk score 5/100.

## Demo

See `examples/demo-sensitive-change/` for a 2-minute walkthrough of how AgentML protects sensitive files from AI agents.

## Links

- **Repo:** https://github.com/Nom-nom-hub/agentml
- **Docs:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **Quickstart:** [docs/quickstart.md](docs/quickstart.md)
- **Spec:** [docs/spec.md](docs/spec.md)
- **Security:** [docs/security.md](docs/security.md)

Built in Rust. Open source (MIT).
