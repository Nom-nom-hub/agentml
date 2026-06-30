# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2026-06-30

### Fixed
- Fixed `agentml init --detect` YAML indentation so generated `AGENT.agent` files parse and validate correctly.
- Fixed `agentml doctor` to parse and validate `AGENT.agent` content instead of only checking that the file exists.

### Added
- Added adoption proof documentation for Rust, Next.js, Node package, and Python package project shapes.

### Changed
- Updated README and website with adoption proof links.

## [0.2.1] - 2026-06-30

### Fixed
- AGENT.agent meta.version now matches Cargo.toml (0.2.0)
- AGENT.agent CLI description lists all v0.2.0 commands
- AGENT.agent workflows include audit_changes (diff + close), doctor, self-check
- AGENT.agent validation section includes doctor and self-check
- docs/release.md steps updated for current v0.2.x line

## [0.2.0] - 2025-06-29

### Added
- `agentml agents-md` command for AGENTS.md generation from AGENT.agent
- `agentml init --detect` now generates AGENTS.md, `.agentml/context.md`, `.agentml/brief.md`
- `--no-agents-md`, `--no-context`, `--no-brief` flags for `agentml init`
- AGENTS.md check in `agentml doctor`
- AGENTS.md listing in `agentml inspect`
- Real-world examples: Rust CLI, Next.js, Node package, Python package
- `examples/dangerous-change-demo/` — proof demo of AgentML catching dangerous AI agent behavior
- `docs/examples.md` — documentation for all examples
- Website Examples section on GitHub Pages

### Changed
- `--force` flag now applies to all generated files during init
- Test files now use `CARGO_MANIFEST_DIR` for path safety
- README documentation with examples section

## [0.1.3] - 2025-06-28

### Added
- `agentml doctor` command for repo health checks
- `agentml completions` command for shell tab completion (bash, zsh, fish)
- `agentml version` command for explicit version display
- `agentml mcp` command for Model Context Protocol server

### Changed
- Improved `--version` output with explicit version command
- Better README documentation for crates.io release
- Added SECURITY.md for vulnerability reporting

## [0.1.2] - crates.io release

### Added
- `agentml init` with template support (`generic`, `rust-cli`, `nextjs-app`, `python-package`)
- `agentml validate` for contract validation
- `agentml inspect` for project information
- `agentml context` for LLM context generation
- `agentml self-check` for dogfooding validation
- `agentml diff` for git diff audit with risk scoring
- `agentml brief` for agent operating brief generation
- `agentml skill` subcommands for skill management
- `agentml run` for task execution
- Skill file support with `.skill` extension
- AGENT.agent contract format
- Risk scoring system (0-100)