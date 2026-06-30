use agentml::parser;
use agentml::types::AgentFile;
use agentml::validator;
use std::path::Path;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn valid_agent_file_passes() {
    let path = project_root().join("examples/basic/AGENT.agent");
    let agent: AgentFile = parser::parse_agent_file(&path).expect("parse should succeed");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
}

#[test]
fn missing_purpose_fails() {
    let yaml = r#"
permissions:
  read: ["**/*"]
safety:
  forbidden_paths: [".env"]
  forbidden_actions: ["rm -rf"]
validation:
  - name: test
    command: "echo ok"
output:
  format: markdown
"#;
    let agent: AgentFile = serde_yaml::from_str(yaml).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(!report.valid);
    assert!(report.errors.iter().any(|e| e.code == "MISSING_PURPOSE"));
}

#[test]
fn missing_permissions_fails() {
    let yaml = r#"
purpose: "test agent"
safety:
  forbidden_paths: [".env"]
  forbidden_actions: ["rm -rf"]
validation:
  - name: test
    command: "echo ok"
output:
  format: markdown
"#;
    let agent: AgentFile = serde_yaml::from_str(yaml).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(!report.valid);
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.code == "MISSING_PERMISSIONS")
    );
}

#[test]
fn forbidden_path_cannot_be_writable() {
    let yaml = r#"
purpose: "test agent"
permissions:
  read: ["**/*"]
  write: [".env"]
safety:
  forbidden_paths: [".env"]
  forbidden_actions: []
validation:
  - name: test
    command: "echo ok"
output:
  format: markdown
"#;
    let agent: AgentFile = serde_yaml::from_str(yaml).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(!report.valid);
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.code == "FORBIDDEN_PATH_WRITE_OVERLAP")
    );
}

#[test]
fn missing_validation_commands_fails() {
    let yaml = r#"
purpose: "test agent"
permissions:
  read: ["**/*"]
safety:
  forbidden_paths: [".env"]
  forbidden_actions: []
output:
  format: markdown
"#;
    let agent: AgentFile = serde_yaml::from_str(yaml).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(!report.valid);
    assert!(report.errors.iter().any(|e| e.code == "MISSING_VALIDATION"));
}

#[test]
fn destructive_action_without_approval_warns() {
    let yaml = r#"
purpose: "test agent"
permissions:
  read: ["**/*"]
  write: ["src/**"]
safety:
  forbidden_paths: [".env"]
  forbidden_actions: ["rm -rf"]
  require_confirmation: []
validation:
  - name: test
    command: "echo ok"
output:
  format: markdown
"#;
    let agent: AgentFile = serde_yaml::from_str(yaml).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.code == "DESTRUCTIVE_NO_CONFIRMATION")
    );
}

#[test]
fn self_check_passes_on_repo() {
    let path = project_root().join("AGENT.agent");
    assert!(path.exists(), "AGENT.agent must exist for self-check test");
    let agent: AgentFile = parser::parse_agent_file(&path).expect("parse should succeed");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid, "Self-check failed: {:?}", report.errors);
}

#[test]
fn context_generation_includes_permissions_and_validation() {
    let path = project_root().join("examples/basic/AGENT.agent");
    let agent: AgentFile = parser::parse_agent_file(&path).expect("parse should succeed");
    let yaml = serde_yaml::to_string(&agent).expect("serialize should succeed");
    assert!(yaml.contains("permissions:"));
    assert!(yaml.contains("validation:"));
}
