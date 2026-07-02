#[test]
fn ast_converts_stack_to_agent_context() {
    let native = r#"agent "test" {
  version "1.0.0"
  context {
    stack ["Rust", "CLI"]
  }
}
"#;
    let ast = agentml::syntax::parse_agent(native).unwrap();
    let agent = agentml::syntax::convert_ast_to_agent(&ast).unwrap();
    let ctx = agent.context.unwrap();
    assert_eq!(ctx.stack_items(), vec!["Rust", "CLI"]);
}

#[test]
fn ast_round_trips_empty_context() {
    let native = r#"agent "test" {
  version "1.0.0"
}
"#;
    let ast = agentml::syntax::parse_agent(native).unwrap();
    let agent = agentml::syntax::convert_ast_to_agent(&ast).unwrap();
    assert!(agent.context.is_none());
}
