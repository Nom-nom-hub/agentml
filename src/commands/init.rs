use crate::detect::{detect_project, print_inspect};
use crate::parser::parse_agent_file;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn run(
    path: PathBuf,
    template: Option<String>,
    detect: bool,
    no_agents_md: bool,
    no_context: bool,
    no_brief: bool,
) -> Result<()> {
    let target = path;
    fs::create_dir_all(&target)?;
    let agent_path = target.join("AGENT.agent");

    if agent_path.exists() && std::env::var("AGENTML_FORCE_INIT").is_err() {
        println!(
            "{}",
            "AGENT.agent already exists in this directory".yellow()
        );
        println!("Use --force or set AGENTML_FORCE_INIT=1 to overwrite.");
        return Ok(());
    }

    let (template_content, _template_name) = if detect {
        let info = detect_project()?;
        print_inspect(&info);
        (generate_detected_template(&info), "detected".to_string())
    } else {
        match template.as_deref() {
            Some("rust-cli") => (rust_cli_template(), "rust-cli".to_string()),
            Some("nextjs-app") => (nextjs_app_template(), "nextjs-app".to_string()),
            Some("python-package") => (python_package_template(), "python-package".to_string()),
            _ => (generic_template(), "generic".to_string()),
        }
    };

    fs::write(&agent_path, template_content).with_context(|| "Failed to write AGENT.agent")?;
    println!(
        "{} {}",
        "Created".green().bold(),
        agent_path.display().to_string().cyan()
    );

    let skills_dir = target.join("skills");
    fs::create_dir_all(&skills_dir)?;
    fs::write(skills_dir.join(".gitkeep"), "")?;

    let agentml_dir = target.join(".agentml");
    fs::create_dir_all(&agentml_dir)?;
    let force = std::env::var("AGENTML_FORCE_INIT").is_ok();

    if !no_context {
        let context_path = agentml_dir.join("context.md");
        if context_path.exists() && !force {
            println!(
                "{}",
                ".agentml/context.md already exists. Use --force to overwrite.".yellow()
            );
        } else {
            fs::write(&context_path, generate_context_md())?;
            println!(
                "{} {}",
                "Created".green().bold(),
                context_path.display().to_string().cyan()
            );
        }
    }

    if !no_brief {
        let brief_path = agentml_dir.join("brief.md");
        if brief_path.exists() && !force {
            println!(
                "{}",
                ".agentml/brief.md already exists. Use --force to overwrite.".yellow()
            );
        } else {
            let cwd = env::current_dir()?;
            env::set_current_dir(&target)?;
            let brief_result = crate::commands::brief::run("md", true, 80, false);
            env::set_current_dir(&cwd)?;
            if let Err(e) = brief_result {
                println!(
                    "{} {}",
                    "Warning:".yellow(),
                    format_args!("Could not generate .agentml/brief.md: {}", e)
                );
            } else {
                println!(
                    "{} {}",
                    "Created".green().bold(),
                    brief_path.display().to_string().cyan()
                );
            }
        }
    }

    if !no_agents_md {
        let agents_md_path = target.join("AGENTS.md");
        if agents_md_path.exists() && !force {
            println!(
                "{}",
                "AGENTS.md already exists. Use --force to overwrite.".yellow()
            );
        } else {
            match parse_agent_file(&agent_path) {
                Ok(agent) => {
                    let md = crate::commands::agents_md::generate(&agent);
                    fs::write(&agents_md_path, &md)?;
                    println!(
                        "{} {}",
                        "Created".green().bold(),
                        agents_md_path.display().to_string().cyan()
                    );
                }
                Err(e) => {
                    println!(
                        "{} {}",
                        "Warning:".yellow(),
                        format_args!("Could not generate AGENTS.md: {}", e)
                    );
                }
            }
        }
    }

    let docs_dir = target.join("docs");
    fs::create_dir_all(&docs_dir)?;
    fs::write(docs_dir.join("agentml.md"), generate_docs_md())?;

    let workflow_dir = target.join(".github").join("workflows");
    fs::create_dir_all(&workflow_dir)?;
    fs::write(workflow_dir.join("agentml-check.yml"), workflow_template())?;

    println!("{}", "Template structure created.".dimmed());
    println!();
    println!("Next steps:");
    println!("  1. Edit AGENT.agent for your project");
    println!("  2. Run: agentml validate AGENT.agent");
    println!("  3. Run: agentml self-check");
    Ok(())
}

fn generate_context_md() -> String {
    r#"# AgentML Context

Project: detected
Template: detected
Generated by: agentml init --detect

## Permissions

See AGENT.agent for full permissions.

## Validation Commands

See AGENT.agent for validation commands.

## Safety Policy

See AGENT.agent for safety rules.
"#
    .to_string()
}

fn generate_docs_md() -> String {
    r#"# AgentML in detected

This project uses AgentML to govern AI agent behavior.

- Contract: [AGENT.agent](../AGENT.agent)
- Context: [.agentml/context.md](../.agentml/context.md)

Run validation:

```bash
agentml validate AGENT.agent
agentml self-check
```
"#
    .to_string()
}

fn generate_detected_template(info: &crate::detect::ProjectInfo) -> String {
    let read_patterns: Vec<String> = info
        .important_files
        .iter()
        .map(|f| format!("    - {}", f))
        .collect();
    let validation: String = info
        .validation_commands
        .iter()
        .map(|c| format!("  - name: validate\n    command: \"{}\"", c))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"# AgentML Execution Contract
meta:
  name: detected-project
  version: "1.0.0"
  description: "Auto-detected project contract"

purpose: >
  AI agent for working with this detected project.
  Project type: {}
  
context:
  project_type: {}
  languages: []
  frameworks: []

permissions:
  read:
{}
  write:
    - "src/**/*"
    - "docs/**/*"
  execute:
    - "npm run"
    - "cargo"
    - "python"

tools: [git, bash, npm, cargo, python]

safety:
  policy: "Never commit secrets. Use environment variables."
  forbidden_paths:
    - ".env*"
    - "node_modules/**"
    - "target/**"
  forbidden_actions:
    - "rm -rf src"
    - "git push --force"
  require_confirmation:
    - "git push"

validation:
{}

output:
  format: markdown
  required_sections:
    - "summary"
    - "changes"
    - "tests"
"#,
        info.project_type,
        info.project_type.to_lowercase().replace(" ", "-"),
        read_patterns.join("\n"),
        validation
    )
}

fn generic_template() -> String {
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
    - "**/*.md"
    - "**/*.json"
    - "**/*.toml"
    - "**/*.yaml"
    - "**/*.yml"
    - "src/**"
    - "app/**"
    - "pages/**"
    - "components/**"
    - "tests/**"
  write:
    - "src/**"
    - "app/**"
    - "pages/**"
    - "components/**"
    - "tests/**"
    - "docs/**"
    - "README.md"
  execute:
    - "npm run"
    - "cargo"
    - "python"

tools: [git, bash, npm, cargo, python]

safety:
  policy: "Never commit secrets. Use environment variables."
  forbidden_paths:
    - ".env*"
    - ".env.*"
    - ".git/**"
    - "node_modules/**"
    - "target/**"
    - "dist/**"
    - "build/**"
    - "**/*secret*"
    - "**/*credential*"
    - "**/*.pem"
    - "**/*.key"
  destructive_actions:
    require_approval:
      - "rm -rf"
      - "git push --force"
      - "npm publish"
      - "cargo publish"
  secrets_policy:
    never_read:
      - ".env"
      - "*.pem"
      - "*.key"
    never_output_secret_values: true

validation:
  - name: Lint
    command: "npm run lint"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
"#
    .to_string()
}

fn rust_cli_template() -> String {
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
    .to_string()
}

fn nextjs_app_template() -> String {
    r#"# AgentML Execution Contract
meta:
  name: my-nextjs-app
  version: "1.0.0"

purpose: >
  AI agent for building and maintaining a Next.js application.

context:
  project_type: nextjs
  languages: [typescript, javascript]
  frameworks: [nextjs, react]

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
    .to_string()
}

fn python_package_template() -> String {
    r#"# AgentML Execution Contract
meta:
  name: my-python-package
  version: "1.0.0"

purpose: >
  AI agent for developing and maintaining a Python package.

context:
  project_type: python-package
  languages: [python]
  frameworks: [pytest, ruff]

permissions:
  read:
    - "**/*.py"
    - "**/pyproject.toml"
    - "**/*.md"
  write:
    - "src/**/*.py"
    - "tests/**/*.py"
    - "pyproject.toml"
  execute:
    - "python"
    - "pytest"
    - "ruff"

tools: [python, pip, git, bash]

safety:
  policy: "Never commit secrets. Use environment variables."
  forbidden_paths:
    - ".venv/**"
    - "dist/**"
    - "*.egg-info"
  forbidden_actions:
    - "rm -rf src"
    - "git push --force"
  require_confirmation:
    - "git push"
    - "twine upload"

validation:
  - name: Lint
    command: "ruff check ."
  - name: Test
    command: "pytest"

output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
    - "risks"
"#
    .to_string()
}

fn workflow_template() -> &'static str {
    r#"name: AgentML Check
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  agentml:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test
      - run: cargo run -- validate AGENT.agent
      - run: cargo run -- skill validate skills/*.skill
      - run: cargo run -- self-check
"#
}
