# AGENTS.md

## Purpose

This project uses AgentML to define how AI coding agents should safely work in a Python package repository published on PyPI.

The machine-readable source of truth is `AGENT.agent`.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. `.agentml/brief.md` (if present)
3. `.agentml/context.md` (if present)

Recommended command:

```bash
agentml brief
```

If AgentML is not installed, read `AGENT.agent` directly.

## Project context

### Stack

- Python
- pytest
- ruff
- mypy
- build (PyPA)

### Important files

- `pyproject.toml`
- `src/**/*.py`
- `tests/**/*.py`
- `AGENT.agent`
- `setup.py`
- `README.md`

## Allowed work areas

Agents may usually modify:

- `src/**`
- `tests/**`
- `docs/**`
- `examples/**`
- `pyproject.toml`
- `setup.py`
- `README.md`
- `AGENT.agent`
- `.github/workflows/*.yml`

Always check `AGENT.agent` for the authoritative list.

## Forbidden areas

Agents must not modify or expose:

- `.env*`
- `.venv/`
- `venv/`
- `__pycache__/`
- `dist/`
- `build/`
- `*.egg-info`
- `.pypirc`
- `**/*.pem`
- `.git/`

## Validation commands

Before reporting completion, run:

```bash
ruff check .       # Lint
mypy .             # Type check
pytest             # Tests
python -m build --sdist  # Build smoke test
```

## Diff audit

After making changes, run:

```bash
agentml diff
```

Include the risk score in the final report.

## Final report format

Every agent task should end with:

```
Summary:
Files changed:
Commands run:
Validation result:
Risk score:
Risks:
Next steps:
```

## Source of truth

If files disagree, follow this order:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`
4. `AGENTS.md`
5. `README.md`
