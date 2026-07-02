#[test]
fn convert_yaml_to_native_uses_canonical_stack_syntax() {
    let yaml = r#"
meta:
  name: test
  version: "1.0.0"
context:
  stack:
    - Rust
    - CLI
"#;
    let output = agentml::commands::convert::convert_yaml_to_native(yaml).unwrap();
    assert!(output.contains(r#"stack ["Rust", "CLI"]"#));
}

#[test]
fn convert_yaml_to_native_omits_colon_for_arrays() {
    let yaml = r#"
meta:
  name: test
  version: "1.0.0"
permissions:
  read:
    - src/**
    - tests/**
"#;
    let output = agentml::commands::convert::convert_yaml_to_native(yaml).unwrap();
    assert!(output.contains(r#"read ["src/**", "tests/**"]"#));
    assert!(!output.contains(r#"read: ["#));
}

#[test]
fn root_agent_has_output_final_report() {
    use agentml::parser::parse_agent_file;
    use std::path::Path;
    let agent = parse_agent_file(Path::new("AGENT.agent")).expect("Failed to parse AGENT.agent");
    assert!(
        agent.output.is_some(),
        "AGENT.agent must have an output block"
    );
    let output = agent.output.unwrap();
    assert!(
        output.required_sections.is_some(),
        "AGENT.agent output must have required_sections"
    );
    let sections = output.required_sections.unwrap();
    assert!(!sections.is_empty(), "required_sections must not be empty");
    assert!(
        sections.iter().any(|s| s.contains("Summary")),
        "Should include Summary field"
    );
    assert!(
        sections.iter().any(|s| s.contains("Files changed")),
        "Should include Files changed field"
    );
    assert!(
        sections.iter().any(|s| s.contains("Risk score")),
        "Should include Risk score field"
    );
    assert!(
        sections.iter().any(|s| s.contains("Commit")),
        "Should include Commit field"
    );
}

#[test]
fn convert_preserves_output_final_report() {
    use std::path::Path;
    let content =
        std::fs::read_to_string(Path::new("AGENT.agent")).expect("Failed to read AGENT.agent");
    let output = agentml::commands::convert::convert_yaml_to_native(&content)
        .expect("Failed to convert AGENT.agent");
    assert!(
        output.contains("output {"),
        "Converted output should have output block"
    );
    assert!(
        output.contains("final_report ["),
        "Converted output should use final_report keyword"
    );
}

#[test]
fn converted_root_agent_validates_without_output_warning() {
    use std::path::Path;
    let content =
        std::fs::read_to_string(Path::new("AGENT.agent")).expect("Failed to read AGENT.agent");
    let native = agentml::commands::convert::convert_yaml_to_native(&content)
        .expect("Failed to convert AGENT.agent");
    let ast = agentml::syntax::parse_agent(&native).expect("Failed to parse converted native");
    let agent = agentml::syntax::convert_ast_to_agent(&ast).expect("Failed to convert AST");
    assert!(
        agent.output.is_some(),
        "Converted native must preserve output block"
    );
    let output = agent.output.unwrap();
    assert!(
        output.required_sections.is_some(),
        "Converted native output must have required_sections"
    );
}

#[test]
fn native_output_final_report_maps_to_internal_model() {
    let native = r#"agent "test" {
  output {
    final_report ["Summary", "Files changed"]
  }
}"#;
    let ast = agentml::syntax::parse_agent(native).expect("Failed to parse native");
    let agent = agentml::syntax::convert_ast_to_agent(&ast).expect("Failed to convert AST");
    assert!(
        agent.output.is_some(),
        "Output must be present after conversion"
    );
    let output = agent.output.unwrap();
    let sections = output
        .required_sections
        .expect("required_sections must be present");
    assert_eq!(sections, vec!["Summary", "Files changed"]);
}
