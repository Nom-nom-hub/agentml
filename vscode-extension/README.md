# AgentML VS Code Extension

Brings AgentML contracts into VS Code — validate, audit, and enforce AI agent contracts without leaving your editor.

## Features

- **Status bar indicator** — green shield when your `AGENT.agent` is valid, red when it's not
- **Validate on save** — automatically validates `AGENT.agent` every time you save it
- **Inline diagnostics** — validation errors show up as squiggles in the editor
- **Commands** (via `Ctrl+Shift+P` / `Cmd+Shift+P`):
  - `AgentML: Initialize Contract` — pick a template and scaffold
  - `AgentML: Validate Contract` — check your contract right now
  - `AgentML: Generate Brief` — operating brief for AI agents
  - `AgentML: Audit Changes` — risk-scored diff of uncommitted changes
  - `AgentML: Check Health` — project health check
- **Syntax highlighting** for `.agent` files

## Requirements

The `agentml` CLI must be installed and on your PATH:

```bash
cargo install agentml
# or
npx agentml --version
```

Configure a custom binary path in settings: `agentml.binaryPath`.

## Settings

- `agentml.binaryPath` — path to the agentml binary (default: `agentml`)
- `agentml.validateOnSave` — validate on save (default: `true`)

## Development

```bash
npm install
npm run compile
```

Press `F5` in VS Code to launch the extension in debug mode.

## Publishing

```bash
npm run package
vsce publish
```
