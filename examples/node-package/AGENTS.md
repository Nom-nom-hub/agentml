# AGENTS.md

## Purpose

This project is a Node.js utility library published on npm. AgentML is used
to define a safety contract that prevents AI agents from accidentally
publishing, versioning, or modifying critical publish-related files.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. Any `.agentml/*.md` files present
3. `package.json` — to understand available scripts and dependencies

## Project context

### Stack

- Node.js (LTS)
- TypeScript
- Vitest (testing)
- ESLint (linting)
- tsc (type checking)

### Important files

- `package.json`: scripts, dependencies, version — requires care
- `tsconfig.json`: TypeScript configuration
- `src/**/*.ts`: source code
- `tests/**/*.test.ts`: test files
- `.npmrc`: never modify or expose
- `.github/workflows/*.yml`: CI/CD pipelines

## Allowed work areas

Agents may modify:

- `src/**/*.ts`
- `src/**/*.js`
- `tests/**/*.ts`
- `package.json` — with caution (see notes below)
- `tsconfig.json`
- `README.md`
- `AGENT.agent`
- `AGENTS.md`
- `.github/workflows/*.yml`

## Forbidden areas

Agents must never modify or expose:

- `.env*` — tokens and secrets
- `.npmrc` — npm authentication config
- `node_modules/**` — dependency tree
- `dist/**` — build output
- `build/**` — build output
- `coverage/**` — test coverage reports
- `**/*.pem`, `**/*.key` — cryptographic material
- `.git/**` — git internals
- `~/.npmrc`, `~/.ssh/` — user-level secrets

## Package.json safety notes

- `npm version` and `npm publish` are **forbidden actions** that require
  human confirmation.
- Agents should not bump the `version` field without explicit user request.
- Adding or removing `publishConfig` requires human approval.

## Validation commands

Before reporting completion, run:

- `npm run lint` — lint
- `npx tsc --noEmit` — type check
- `npm test` — test
- `npm run build` — build

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
