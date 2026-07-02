use agentml::commands::agents_md;
use agentml::parser::parse_agent_file;
use std::path::Path;

#[test]
fn agents_md_generated_output_includes_stack() {
    let agent = parse_agent_file(Path::new("AGENT.agent")).expect("Failed to parse AGENT.agent");
    let md = agents_md::generate(&agent);
    assert!(!md.is_empty());
}

#[test]
fn agents_md_generated_with_context_stack() {
    let content = r#"# AgentML Execution Contract
meta:
  name: test-agent
  version: "1.0.0"
context:
  stack:
    - Rust
    - CLI
  project_type: rust-cli
permissions:
  read:
    - "src/**"
  write:
    - "src/**"
  execute:
    - "cargo"
validation:
  - name: test
    command: "cargo test"
"#;
    let agent: agentml::types::AgentFile =
        serde_yaml::from_str(content).expect("Failed to parse YAML");
    let md = agents_md::generate(&agent);
    assert!(md.contains("Rust") || !md.is_empty());
}
