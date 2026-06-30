# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - Unreleased

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