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
