use agentml::validator;

fn validate_native(content: &str) -> bool {
    let ast = agentml::syntax::parse_agent(content).unwrap();
    let agent = agentml::syntax::convert_ast_to_agent(&ast).unwrap();
    validator::validate_agent_file(&agent, false).valid
}

const GENERIC_NATIVE: &str = r#"agent "my-project" {
  version "1.0.0"
  context {
    stack ["Generic"]
  }
  permissions {
    read ["**/*.md"]
    write ["src/**"]
    execute ["npm run"]
  }
  safety {
    policy "Never commit secrets."
    forbidden_paths [".env"]
    forbidden_actions ["rm -rf"]
    require_approval ["git push"]
  }
  validation {
    command "npm run lint"
  }
  output {
    required ["changes", "tests"]
  }
}
"#;

#[test]
fn generic_native_template_uses_canonical_stack_syntax() {
    assert!(GENERIC_NATIVE.contains(r#"stack ["Generic"]"#));
    assert!(!GENERIC_NATIVE.contains("stack:"));
}

#[test]
fn generic_native_template_validates() {
    assert!(validate_native(GENERIC_NATIVE));
}
