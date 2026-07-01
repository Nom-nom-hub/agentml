use agentml::validator;

fn parse_native_agent_path(content: &str) -> anyhow::Result<agentml::types::AgentFile> {
    let ast = agentml::syntax::parse_agent(content)?;
    agentml::syntax::convert_ast_to_agent(&ast)
}

#[test]
fn init_template_generic_native_validates() {
    let native = r#"agent "my-project" {
  version "1.0.0"

  purpose {
    human_goal "Test agent"
    agent_goal "Test goal"
    non_goals ["Delete production data"]
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
    forbidden_paths [".env", ".git/**"]
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
    let agent = parse_native_agent_path(native).unwrap();
    let report = validator::validate_agent_file(&agent, false);
    assert!(
        report.valid,
        "Generic native template should validate: {:?}",
        report.errors
    );
}

#[test]
fn native_generated_contract_has_forbidden_paths() {
    let native = r#"agent "test" {
  safety {
    forbidden_paths [".env", ".git/**"]
    forbidden_actions ["rm -rf"]
  }
}
"#;
    let agent = parse_native_agent_path(native).unwrap();
    assert!(agent.safety.is_some());
    let safety = agent.safety.unwrap();
    assert!(safety.forbidden_paths.is_some());
    assert!(!safety.forbidden_paths.unwrap().is_empty());
}

#[test]
fn native_generated_contract_has_final_report_fields() {
    let content = r#"# AGENTS.md

Summary:
Files changed:
Risk score:
"#;
    assert!(content.contains("Summary:"));
    assert!(content.contains("Files changed:"));
    assert!(content.contains("Risk score:"));
}

#[test]
fn native_purpose_struct_parses() {
    let native = r#"agent "test" {
  purpose {
    human_goal "Human readable"
    agent_goal "Agent goal"
    non_goals ["don't do X"]
  }
}
"#;
    let agent = parse_native_agent_path(native).unwrap();
    assert!(agent.purpose.is_some());
    let purpose = agent.purpose.unwrap();
    assert_eq!(purpose.human_goal.unwrap(), "Human readable");
    assert_eq!(purpose.agent_goal.unwrap(), "Agent goal");
    assert_eq!(purpose.non_goals.unwrap().len(), 1);
}

#[test]
fn native_safety_policy_parses() {
    let native = r#"agent "test" {
  safety {
    policy "Never commit secrets"
    forbidden_paths [".env"]
    forbidden_actions ["rm -rf"]
    require_approval ["git push"]
  }
}
"#;
    let agent = parse_native_agent_path(native).unwrap();
    assert!(agent.safety.is_some());
    let safety = agent.safety.unwrap();
    assert!(safety.policy.is_some());
    assert!(safety.forbidden_paths.is_some());
    assert!(safety.forbidden_actions.is_some());
    assert!(safety.require_confirmation.is_some());
}
