const GENERIC_NATIVE: &str = r#"agent "my-project" {
  version "1.0.0"

  purpose {
    human_goal "Test the generic native template"
    agent_goal "Validate generic native template"
    non_goals ["Do no harm"]
  }

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
    require_approval ["rm -rf"]
  }

  validation {
    command "npm run lint"
    success "Tests pass"
  }

  output {
    format markdown
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
    let ast = agentml::syntax::parse_agent(GENERIC_NATIVE).unwrap();
    let agent = agentml::syntax::convert_ast_to_agent(&ast).unwrap();
    let report = agentml::validator::validate_agent_file(&agent, false);
    assert!(
        report.valid,
        "Generic native template should validate. Errors: {:?}, Warnings: {:?}",
        report.errors, report.warnings
    );
}
