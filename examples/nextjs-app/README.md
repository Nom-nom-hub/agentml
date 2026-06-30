# AgentML Next.js Example

This directory contains a sample `AGENT.agent` contract and `AGENTS.md` guide for a Next.js blog platform with authentication, CMS, and API routes.

## How it works

AgentML uses a YAML contract (`AGENT.agent`) to tell AI coding agents:

- **What files they can read and write** — agents are restricted to `src/app/`, `src/components/`, `src/lib/`, and `tests/`. Everything else is off-limits.
- **What commands they may execute** — only `pnpm` and `npx`. Dangerous Vercel commands (`vercel secrets`, `vercel env pull`) are forbidden.
- **What actions require confirmation** — `git push`, database migrations, and production deploys all require human sign-off.
- **What validation must pass** — lint, type check, tests, and build must all succeed before the agent reports completion.

## Protecting environment variables

The contract explicitly forbids agents from reading or modifying:

- `.env.local` / `.env.production` — your secrets stay secret.
- `vercel.json` — deployment config is off-limits.
- `**/*.pem` / `**/*.key` — TLS keys are never exposed.

## Requiring validation before completion

Every agent task must run:

```bash
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```

If any step fails, the agent must fix the issue before reporting done.

## How to use this contract

1. Copy `AGENT.agent` and `AGENTS.md` into the root of your Next.js project.
2. Create `.agentml/brief.md` and `.agentml/context.md` with project-specific instructions.
3. Install AgentML:
   ```bash
   cargo install agentml
   ```
4. Validate the contract:
   ```bash
   agentml validate AGENT.agent
   ```
5. Run the diff audit after any AI change:
   ```bash
   agentml diff
   ```

## Files

| File | Purpose |
|------|---------|
| `AGENT.agent` | Machine-readable YAML contract |
| `AGENTS.md` | Human-readable agent instructions |
| `README.md` | This file |
