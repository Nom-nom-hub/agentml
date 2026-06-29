use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf, template: Option<String>) -> Result<()> {
    let target = path;
    fs::create_dir_all(&target)?;
    let agent_path = target.join("AGENT.agent");

    if agent_path.exists() {
        println!(
            "{}",
            "AGENT.agent already exists in this directory".yellow()
        );
        return Ok(());
    }

    let template_content = match template.as_deref() {
        Some("nextjs") => {
            r#"# AgentML Execution Contract
meta:
  name: my-nextjs-app
  version: "1.0.0"

purpose: >
  AI agent for building and maintaining a Next.js application with authentication.

context:
  project_type: nextjs
  languages: [typescript, javascript]
  frameworks: [nextjs, react, tanstack-query]

permissions:
  read:
    - "**/*.ts"
    - "**/*.tsx"
    - "**/*.json"
    - "**/*.md"
  write:
    - "src/**/*.ts"
    - "src/**/*.tsx"
    - "app/**/*.tsx"
  execute:
    - "npm run"
    - "npx"

tools: [npm, node, git, bash]

safety:
  policy: >
    Never commit secrets. Use environment variables and .env.local.
    Require human review for all git push and database changes.
  forbidden_paths:
    - ".env*.local"
    - ".env.production"
    - "node_modules/**"
  forbidden_actions:
    - "git push --force"
    - "rm -rf src"
  require_confirmation:
    - "git push"
    - "npm run db:migrate"

validation:
  - name: Lint
    command: "npm run lint"
  - name: Type Check
    command: "npm run typecheck"
  - name: Test
    command: "npm test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
"#
        }
        Some("rust") => {
            r#"# AgentML Execution Contract
meta:
  name: my-rust-cli
  version: "1.0.0"

purpose: >
  AI agent for developing and maintaining a Rust CLI application.

context:
  project_type: rust-cli
  languages: [rust]
  frameworks: [clap, tokio]

permissions:
  read:
    - "**/*.rs"
    - "**/Cargo.toml"
    - "**/*.md"
  write:
    - "src/**/*.rs"
    - "Cargo.toml"
  execute:
    - "cargo"
    - "rustfmt"
    - "clippy"

tools: [cargo, rustfmt, clippy, git, bash]

safety:
  policy: >
    Never delete code without creating a commit first.
    Require human review for all releases and dependency updates.
  forbidden_paths:
    - "target/**"
    - "**/*.rs.bk"
  forbidden_actions:
    - "cargo publish"
    - "rm -rf src"
  require_confirmation:
    - "cargo publish"
    - "cargo install"

validation:
  - name: Format
    command: "cargo fmt -- --check"
  - name: Clippy
    command: "cargo clippy -- -D warnings"
  - name: Test
    command: "cargo test"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
"#
        }
        _ => {
            r#"# AgentML Execution Contract
meta:
  name: my-project
  version: "1.0.0"
  description: "Short description of what this agent should do"

purpose: >
  Describe the agent's purpose and what it is allowed to do.
  Be specific about goals and constraints.

context:
  project_type: generic
  languages: []
  frameworks: []

permissions:
  read:
    - "**/*"
  write:
    - "src/**/*"
    - "docs/**/*"
  execute:
    - "npm run"
    - "cargo"
    - "python"

tools: [git, bash, npm]

safety:
  policy: "Never commit secrets. Use environment variables."
  forbidden_paths:
    - ".env*"
    - "node_modules/**"
  forbidden_actions:
    - "rm -rf src"
    - "git push --force"
  require_confirmation:
    - "git push"

validation:
  - name: Lint
    command: "npm run lint"

output:
  format: markdown
  required_sections:
    - "summary"
    - "changes"
"#
        }
    };

    fs::write(&agent_path, template_content).with_context(|| "Failed to write AGENT.agent")?;
    println!(
        "{} {}",
        "Created".green().bold(),
        agent_path.display().to_string().cyan()
    );
    println!(
        "{}",
        "Edit AGENT.agent to define your agent's execution contract".dimmed()
    );
    Ok(())
}
