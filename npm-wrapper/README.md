# agentml npm wrapper

This directory contains the npm distribution wrapper for agentml.

## Usage

```bash
npx agentml init
# or
npm install -g agentml
agentml --help
```

## How it works

1. On `npm install`, `bin/install.js` downloads the pre-built binary
   for the current platform from GitHub Releases.
2. `bin/agentml.js` proxies all commands to the downloaded binary.
3. If no pre-built binary exists, falls back to `cargo install agentml`.

## Publishing

```bash
cd npm-wrapper
npm version <version>  # must match Cargo.toml version
npm publish --access public
```

## Release binaries

GitHub Releases must include these assets for the installer to work:
- `agentml-x86_64-linux.tar.gz`
- `agentml-aarch64-linux.tar.gz`
- `agentml-x86_64-macos.tar.gz`
- `agentml-aarch64-macos.tar.gz`
- `agentml-x86_64-windows.tar.gz`

Each archive must contain the `agentml` binary (or `agentml.exe` on Windows).
