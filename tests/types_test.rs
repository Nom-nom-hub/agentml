#[test]
fn agent_context_stack_items_returns_explicit_stack() {
    let ctx = agentml::types::AgentContext {
        stack: Some(vec!["Rust".to_string(), "CLI".to_string()]),
        project_type: None,
        languages: None,
        frameworks: None,
    };
    assert_eq!(ctx.stack_items(), vec!["Rust", "CLI"]);
}

#[test]
fn agent_context_stack_items_falls_back_to_legacy_fields() {
    let ctx = agentml::types::AgentContext {
        stack: None,
        project_type: Some("rust-cli".to_string()),
        languages: Some(vec!["rust".to_string(), "shell".to_string()]),
        frameworks: Some(vec!["clap".to_string()]),
    };
    assert_eq!(ctx.stack_items(), vec!["rust-cli", "rust", "shell", "clap"]);
}
