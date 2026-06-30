use agentml::syntax::{is_native_syntax, parse_native_agent, parse_native_skill};
use std::path::Path;

#[test]
fn native_agent_parses_correctly() {
    let path = Path::new("examples/native/AGENT.agent");
    let result = parse_native_agent(path);
    assert!(
        result.is_ok(),
        "Failed to parse native agent: {:?}",
        result.err()
    );
}

#[test]
fn native_skill_parses_correctly() {
    let path = Path::new("examples/native/rust-cli-maintainer.skill");
    let result = parse_native_skill(path);
    assert!(
        result.is_ok(),
        "Failed to parse native skill: {:?}",
        result.err()
    );
}

#[test]
fn native_detection_works() {
    let native_content = r#"agent "test" { version "0.4.0" }"#;
    let yaml_content = r#"meta:
  name: test
version: "0.4.0""#;

    assert!(is_native_syntax(native_content));
    assert!(!is_native_syntax(yaml_content));
}
