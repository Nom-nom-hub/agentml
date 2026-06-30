# Release Checklist

This document describes the maintenance release process for AgentML.

## Pre-release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Ensure working tree is clean:
   ```bash
   git status --short
   ```
4. Ensure all tests pass:
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
   cargo run -- close
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
   VERSION=$(cargo metadata --format-version=1 --no-deps | cargo install cargo-json >/dev/null 2>&1 && echo "v$(cargo metadata --format-version=1 --no-deps | python3 -c 'import sys,json;print(json.load(sys.stdin)["packages"][0]["version"])')" || echo "v0.2.1")
   git tag -a "$VERSION" -m "Release $VERSION"
   git push origin "$VERSION"
   ```

5. Create GitHub release:
   ```bash
   VERSION=$(cargo metadata --format-version=1 --no-deps | python3 -c 'import sys,json;print("v"+json.load(sys.stdin)["packages"][0]["version"])')
   gh release create "$VERSION" --title "$VERSION" --notes "$(cat CHANGELOG.md)" --target main
   ```

## GitHub Pages (optional)

If using GitHub Pages for docs:

1. Enable Pages in repo settings:
   - Go to Settings → Pages
   - Source: `Deploy from a GitHub Actions workflow`
   - Workflow: `GitHub Pages`

2. The workflow will deploy automatically on push to main.

## Post-release

1. Update GitHub release notes from CHANGELOG.md
2. Verify crates.io has the new version
3. Update version in docs if needed

## Release checklist

- Working tree is clean.
- Version bump committed.
- CHANGELOG.md committed.
- Release tag created.