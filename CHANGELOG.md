# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-28

### Added
- Initial release of AgentML
- `AGENT.agent` schema with purpose, context, permissions, safety, validation, and output blocks
- `.skill` file format for reusable AI capabilities
- `agentml init` command with templates (generic, rust-cli, nextjs-app, python-package)
- `agentml validate` command with semantic validation and risk scoring
- `agentml inspect` command for human-readable contract summaries
- `agentml run <task>` command for task execution plans
- `agentml context` command for LLM-readable context export
- `agentml skill validate` command
- `agentml skill pack` command
- `agentml self-check` command for dogfood validation
- `agentml diff` command for permission diffs
- `agentml doctor` command for sanity checks
- Validation engine with 9 semantic rules
- Risk scoring model (0-100)
- JSON Schema for `AGENT.agent`
- Example contracts: `examples/basic/`, `examples/nextjs-auth/`, `examples/demo-sensitive-change/`, `examples/minimal/`, `examples/rust-cli/`, `examples/nextjs-app/`, `examples/python-package/`
- GitHub Actions self-check workflow
- Comprehensive test suite (14+ tests)
- Documentation: `docs/quickstart.md`, `docs/spec.md`, `docs/security.md`, `docs/templates.md`, `docs/ci.md`, `docs/dogfooding.md`
- Dogfooding: AgentML validates itself and its skills
- Release polish: README.md, CHANGELOG.md, LICENSE, CONTRIBUTING.md, SECURITY.md, CODE_OF_CONDUCT.md
- GitHub issue and PR templates

### Security
- Forbidden path and action enforcement
- Destructive command confirmation requirements
- Secrets policy validation
- Write permission safety checks

### Documentation
- README with quickstart, CLI reference, and dogfooding section
- Language specification
- Dogfooding guide
- Security model documentation
