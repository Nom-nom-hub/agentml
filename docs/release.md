# Release Checklist

This document describes the maintenance release process for AgentML.

## Pre-release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Ensure all tests pass:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

## Release

1. Run verification commands:
   ```bash
   cargo run -- doctor
   cargo run -- self-check
   cargo run -- validate AGENT.agent
   cargo run -- brief --format json
   cargo run -- diff
   ```

2. Dry-run publish:
   ```bash
   cargo publish --dry-run
   ```

3. Publish:
   ```bash
   cargo publish
   ```

4. Tag the release:
   ```bash
   git tag -a v0.1.3 -m "Release v0.1.3"
   git push origin v0.1.3
   ```

## Post-release

1. Update GitHub release notes from CHANGELOG.md
2. Verify crates.io has the new version
3. Update version in docs if needed