# AGENTS.md

## Purpose

This project uses AgentML to define how AI coding agents should safely work in this Next.js repository.

The machine-readable source of truth is `AGENT.agent`.

## Required first steps

Before editing files, agents should read:

1. `AGENT.agent`
2. `.agentml/brief.md`
3. `.agentml/context.md`

Recommended command:

```bash
agentml brief
```

If AgentML is not installed, read `AGENT.agent` directly.

## Project context

### Stack

- Next.js (App Router)
- React
- TypeScript
- Tailwind CSS
- Prisma ORM
- pnpm

### Features

- SSR / SSG / ISR
- API routes
- Authentication
- Content management system
- Public API routes

### Important files

- `src/app/**/*.tsx`
- `src/app/**/*.ts`
- `src/components/**/*.tsx`
- `src/lib/**/*.ts`
- `src/styles/**/*.css`
- `tests/**/*.ts`
- `*config.*`
- `package.json`
- `README.md`

## Allowed work areas

Agents may modify:

- `src/app/**/*.tsx`
- `src/app/**/*.ts`
- `src/components/**/*.tsx`
- `src/lib/**/*.ts`
- `tests/**/*.ts`
- `README.md`

Always check `AGENT.agent` for the authoritative list.

## Forbidden areas

Agents must not modify or expose:

- `.env.local`
- `.env.production`
- `.next/**`
- `node_modules/**`
- `vercel.json`
- `**/*.pem`
- `**/*.key`
- Any `.env` files containing secrets
- Vercel secrets or environment pull artifacts

## Validation commands

Before reporting completion, run:

- `pnpm lint` — Lint
- `pnpm typecheck` — Type Check
- `pnpm test` — Test
- `pnpm build` — Build

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
