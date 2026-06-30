# Python ML Utils — Safe Agent Workflows

This directory is an **AgentML example** showing how to constrain AI agents
when working on a Python package published to PyPI.

## Safe agent workflows for Python repos

Python repositories have unique safety concerns — virtual environments,
build artifacts, cached bytecode, and PyPI credentials can all be accidentally
exposed or destroyed by an overly eager agent. AgentML solves this by
embedding a machine-readable safety contract (`AGENT.agent`) that agents
read before acting.

Key workflow protections:

- **Scoped reads/writes.** Agents can only read `src/`, `tests/`, and docs —
  never `.env`, `.venv`, or `.pypirc`.
- **Controlled execution.** Agents may run `pytest` and `ruff` freely, but
  `twine upload` and `python -m build` require explicit human confirmation.
- **Forbidden actions.** Destructive operations like `rm -rf src` or
  `git push --force` are blocked entirely.

## Package/build artifact protection

Build outputs (`dist/`, `build/`, `*.egg-info`) and cached bytecode
(`__pycache__/`, `*.pyc`) are in `forbidden_paths`. Agents will refuse
to read or write these files, preventing accidental corruption or
leakage of build artifacts.

## How AgentML prevents accidental PyPI releases

The `forbidden_actions` and `require_confirmation` fields work together:

```yaml
forbidden_actions:
  - "twine upload"
  - "python -m build && twine upload"
require_confirmation:
  - "twine upload"
  - "python -m build"
```

- A bare `twine upload` is **forbidden** — agents will refuse outright.
- `python -m build` is **allowed but only with confirmation**, so a human
  always reviews the build before distribution.
- The `validation` block includes `python -m build --sdist` as a **read-only
  build smoke test** that produces no `dist/` output (the `--sdist` flag
  writes to a temp directory), giving agents a safe way to verify the
  package builds without creating artifacts.

## How to adopt

1. Copy `AGENT.agent` and `AGENTS.md` to the root of your Python project.
2. Adjust the `permissions` and `safety` sections to match your project
   structure.
3. Run `agentml validate AGENT.agent` to check the contract is well-formed.
4. Commit both files — they become the contract that governs every AI agent
   interaction with your repository.
