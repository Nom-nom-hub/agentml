---
layout: default
title: CI Integration
description: Enforce AgentML validation in continuous integration.
---

# CI Integration

This document explains how to enforce AgentML validation in continuous integration.

## GitHub Actions

Copy this workflow into `.github/workflows/agentml-check.yml` in your repo:

```yaml
name: AgentML Check
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  agentml:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Format check
        run: cargo fmt --check
      - name: Clippy
        run: cargo clippy --all-targets -- -D warnings
      - name: Tests
        run: cargo test
      - name: Validate AGENT.agent
        run: cargo run -- validate AGENT.agent
      - name: Validate skills
        run: cargo run -- skill validate skills/*.skill
      - name: Self-check
        run: cargo run -- self-check
      - name: Generate context
        run: cargo run -- context
```

## What It Does

1. **Format check** — ensures code style consistency
2. **Clippy** — catches Rust anti-patterns
3. **Tests** — ensures functionality works
4. **Validate AGENT.agent** — ensures the contract is valid
5. **Validate skills** — ensures all `.skill` files are valid
6. **Self-check** — ensures the dogfood contract still passes
7. **Generate context** — updates `.agentml/context.md`

## Non-Rust Projects

If your project does not use Rust, replace steps 1-3 with your language's equivalent:

```yaml
      - name: Lint
        run: npm run lint
      - name: Test
        run: pytest
```

Keep the AgentML steps (4-7) unchanged.

## Matrix Strategy

For multi-version testing:

```yaml
jobs:
  agentml:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@${{ matrix.rust }}
      - run: cargo test
      - run: cargo run -- validate AGENT.agent
```

## Caching

Speed up CI with cargo caching:

```yaml
      - uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## Allowed Failures

If you want CI to pass but still report on certain checks:

```yaml
      - name: Clippy
        continue-on-error: true
        run: cargo clippy --all-targets -- -D warnings
```

Do **not** allow failures for `validate AGENT.agent` or `self-check`. These are the contract enforcement steps.

## Automated Context Updates

If you want `.agentml/context.md` to be automatically committed when it changes:

```yaml
      - name: Generate context
        run: cargo run -- context
      - name: Commit context
        run: |
          git diff --quiet .agentml/context.md || git add .agentml/context.md && git commit -m "update context" && git push
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Other CI Systems

- **GitLab CI**: Use the same steps in `.gitlab-ci.yml`
- **CircleCI**: Use `cimg/rust` Docker image
- **Azure Pipelines**: Use `rust:latest` container

The core AgentML commands are CI-agnostic.
