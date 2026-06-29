# Contributing to AgentML

Thank you for your interest in contributing!

## How to Contribute

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test`
5. Run `cargo run -- self-check`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## Development Setup

```bash
git clone https://github.com/Nom-nom-hub/agentml.git
cd agentml && cargo build
```

## Running Tests

```bash
cargo test
```

## Code Style

- Use `cargo fmt` before committing
- Fix all clippy warnings
- Add tests for new validation rules

## Dogfooding

This project uses AgentML to govern itself. Your changes must pass:

```bash
cargo run -- validate AGENT.agent
cargo run -- self-check
```

## Questions?

Open an issue or reach out to the maintainers.
