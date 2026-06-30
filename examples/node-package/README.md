# Node Utils Library — AgentML Safety Contract

This project uses [AgentML](https://agentml.dev) to define how AI coding
agents operate in this repository safely. The machine-readable source of
truth is `AGENT.agent`.

## How AgentML prevents accidental npm release

Publishing a package to npm is a sensitive operation — once published, a
version cannot be deleted. The `AGENT.agent` file defines a safety policy
that blocks agents from running:

- `npm publish`
- `npm version`

These commands are listed under `safety.forbidden_actions` and
`safety.require_confirmation`. An agent **cannot** execute them without
explicit human approval, even if the user asks it to "do a release."

Additionally, `safety.forbidden_paths` prevents agents from reading or
writing `.npmrc` (which contains npm tokens) and `dist/` (publish
artifacts), ensuring credentials and built output are never exposed.

## How agents handle package.json changes

`package.json` is writable under `permissions.write`, but agents must:

1. Never bump the `version` field unilaterally.
2. Never add or remove `publishConfig` without asking.
3. Never overwrite `scripts` that relate to publishing.

The validation step (`npm run build && npm test`) ensures structural
integrity before any `package.json` change is reported.

## Why publish commands require human confirmation

1. **Publishing is irreversible** — unlike code changes, a bad publish
   can break users and cannot be rolled back.
2. **Version numbers matter** — semver violations cause downstream breakage.
3. **Human judgement** — only a human can decide if the change warrants a
   patch, minor, or major bump.

By requiring confirmation through `require_confirmation`, AgentML ensures
that every publish or version bump is reviewed by a person.

## How to adopt this contract

1. Copy `AGENT.agent` and `AGENTS.md` to your own npm package repository.
2. Adjust `permissions.read` and `permissions.write` to match your source
   layout.
3. Customize `safety.forbidden_actions` for your release workflow (e.g.,
   add `npm run release` if you use a custom script).
4. Run `agentml validate AGENT.agent` to verify the contract is well-formed.
5. Run `agentml diff` after any agent session to review what changed.
