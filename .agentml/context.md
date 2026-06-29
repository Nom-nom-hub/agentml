meta:
  name: agentml
  version: 0.1.1
  description: null
purpose:
  human_goal: |
    Build an AI-native markup language and CLI for agent execution contracts.
  agent_goal: |
    Modify this repo safely, validate all changes, and report proof of completion.
  non_goals:
  - Do not become a general programming language.
  - Do not execute untrusted skills without validation.
  - Do not store secrets or credentials.
context:
  project_type: null
  languages: null
  frameworks: null
permissions:
  read:
  - '**/*.rs'
  - '**/*.agent'
  - '**/*.skill'
  - '**/*.md'
  - '**/*.json'
  - Cargo.toml
  write:
  - src/**
  - tests/**
  - docs/**
  - examples/**
  - AGENT.agent
  - agentml.schema.json
  - README.md
  execute: null
tools: null
workflows:
- name: default_change
  description: |
    Default change workflow: inspect contract, plan minimal change, edit files, run validation, report result.
  steps:
  - name: inspect_contract
    description: null
    commands: null
    success: null
    on_failure: null
  - name: plan_minimal_change
    description: null
    commands: null
    success: null
    on_failure: null
  - name: edit_files
    description: null
    commands: null
    success: null
    on_failure: null
  - name: run_validation
    description: null
    commands: null
    success: null
    on_failure: null
  - name: report_result
    description: null
    commands: null
    success: null
    on_failure: null
- name: release_check
  description: |
    Pre-release workflow: run fmt, clippy, tests, validate AGENT.agent, validate all skills.
  steps:
  - name: fmt
    description: null
    commands:
    - cargo fmt -- --check
    success: null
    on_failure: null
  - name: clippy
    description: null
    commands:
    - cargo clippy --all-targets -- -D warnings
    success: null
    on_failure: null
  - name: test
    description: null
    commands:
    - cargo test
    success: null
    on_failure: null
  - name: validate_contract
    description: null
    commands:
    - cargo run -- validate AGENT.agent
    success: null
    on_failure: null
  - name: validate_skills
    description: null
    commands:
    - cargo run -- skill validate skills/*.skill
    success: null
    on_failure: null
tasks: null
memory: null
safety:
  forbidden_paths:
  - .env
  - .git/**
  - target/**
  - '**/*secret*'
  - '**/*credential*'
  - ~/.ssh/**
  forbidden_actions:
  - rm -rf
  - git reset --hard
  - git clean -fd
  - cargo publish
  destructive_actions:
    require_explicit_user_approval: true
    commands:
    - rm -rf
    - git reset --hard
    - git clean -fd
    - cargo publish
  secrets_policy:
    never_read:
    - .env
    - '*.pem'
    - '*.key'
    never_output_secret_values: true
validation:
- name: fmt
  command: cargo fmt -- --check
  description: null
- name: clippy
  command: cargo clippy --all-targets -- -D warnings
  description: null
- name: test
  command: cargo test
  description: null
- name: self_validate
  command: cargo run -- validate AGENT.agent
  description: null
success_criteria:
- Rust tests pass.
- AGENT.agent validates successfully.
- Example .skill files validate successfully.
- No forbidden files are modified.
- Final report includes changed files, tests run, and risks.
output:
  format: markdown
  required_sections: null
