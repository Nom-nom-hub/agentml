---
layout: default
title: Templates
description: Built-in init templates for common project types.
---

# Templates

AgentML includes built-in templates for common project types. Use `agentml init --template <name>` to generate a starter `AGENT.agent`.

## Available Templates

- `generic` — default template for any project
- `rust-cli` — Rust CLI with Cargo workflows
- `nextjs-app` — Next.js App Router with TypeScript
- `python-package` — Python package with pytest and ruff

## Template Contents

Each template generates:

```
./
├── AGENT.agent          # Agent execution contract
├── skills/              # Reusable skills
│   └── .gitkeep
├── .agentml/            # Generated context
│   └── context.md
└── docs/
    └── agentml.md       # Project-specific AgentML docs
```

## Using Templates

```bash
# Current directory
agentml init --template rust-cli

# Specific directory
agentml init /path/to/project --template nextjs-app

# With existing directory (does not overwrite AGENT.agent if present)
agentml init --template python-package
```

## Custom Templates

To create a custom template, add a `.agent` file to `templates/` in the AgentML repo and reference it by name.

## Template Details

### generic

Minimal contract for any project type. Default permissions for `src/**` and `docs/**`. Basic validation with lint/test commands.

### rust-cli

Optimized for Rust CLI projects. Includes:

- Permissions for `src/**/*.rs`, `Cargo.toml`, `**/*.md`
- Validation: `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`
- Safety rules for `target/**` and build artifacts

### nextjs-app

Optimized for Next.js with TypeScript. Includes:

- Permissions for `app/**`, `src/**`, `*.ts`, `*.tsx`
- Validation: `npm run lint`, `npm run typecheck`, `npm test`
- Safety rules for `.env*.local`, `node_modules/**`

### python-package

Optimized for Python packages. Includes:

- Permissions for `src/**`, `tests/**`, `pyproject.toml`
- Validation: `ruff check`, `pytest`
- Safety rules for `.venv/`, `dist/`, `*.egg-info`
