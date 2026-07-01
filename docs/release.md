# v0.4.0 Release Criteria

## Native Syntax Stability Requirements

Native syntax can ship in v0.4.0 only when:

1. **Native examples validate**
   - `cargo run -- validate examples/native/AGENT.agent --format native` passes
   - `cargo run -- skill validate examples/native/*.skill --format native` passes

2. **Native init output validates**
   - `agentml init --template generic --syntax native` produces valid contract
   - `agentml init --template rust-cli --syntax native` produces valid contract
   - `agentml init --template nextjs --syntax native` produces valid contract
   - `agentml init --template node-package --syntax native` produces valid contract
   - `agentml init --template python-package --syntax native` produces valid contract
   - `agentml init --detect --syntax native` produces valid contract

3. **Conversion output validates**
   - `agentml convert --to native AGENT.agent` produces valid native contract
   - `agentml convert --to native AGENT.agent --write` works correctly
   - `agentml convert --to native AGENT.agent --write --backup` preserves original

4. **Command compatibility works with native AGENT.agent**
   - `agentml doctor` works with native contracts
   - `agentml brief --format json` works with native contracts
   - `agentml diff` works with native contracts
   - `agentml close` works with native contracts
   - `agentml skill match` works with native contracts

5. **YAML remains default and supported**
   - v0.3.0 YAML syntax still works
   - Existing YAML contracts parse correctly
   - Backward compatibility maintained

6. **Documentation explains migration clearly**
   - docs/syntax.md updated with current syntax
   - docs/spec.md updated with native syntax spec
   - Migration guide provided

7. **CI passes**
   - `cargo fmt --check` passes
   - `cargo clippy --all-targets -- -D warnings` passes
   - `cargo test` passes
   - `cargo run -- self-check` passes

## Post-Release Tasks

After v0.4.0:

1. Update default `--syntax` to `native`
2. Deprecate `--syntax yaml` flag
3. Remove YAML parsing in v0.5.0