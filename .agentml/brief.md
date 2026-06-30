# AgentML Operating Brief

Project: agentml

Stack: Rust

Allowed write paths:
- ["src/**", "tests/**", "docs/**", "examples/**", "AGENT.agent", "agentml.schema.json", "README.md", ".github/workflows/*.yml", "Gemfile", "Cargo.toml", "Cargo.lock", "tests/**/*.rs"]

Forbidden:
- []

Required validation:
["cargo fmt -- --check", "cargo clippy --all-targets -- -D warnings", "cargo test", "cargo run -- validate AGENT.agent"]

Risk: 0 (low)
