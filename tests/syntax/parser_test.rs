#[test]
fn parse_agent_with_stack() {
    let native = r#"agent "test" {
  context {
    stack ["Rust", "CLI"]
  }
}
"#;
    let ast = agentml::syntax::parse_agent(native).unwrap();
    assert_eq!(ast.agent, "test");
    let ctx = ast.context.unwrap();
    assert_eq!(ctx.stack, vec!["Rust", "CLI"]);
}

#[test]
fn parse_agent_without_context() {
    let native = r#"agent "test" {
  version "1.0.0"
}
"#;
    let ast = agentml::syntax::parse_agent(native).unwrap();
    assert!(ast.context.is_none());
}

#[test]
fn parse_agent_with_colon_style_stack() {
    let native = r#"agent "test" {
  context {
    stack: ["Rust", "CLI"]
  }
}
"#;
    let ast = agentml::syntax::parse_agent(native).unwrap();
    let ctx = ast.context.unwrap();
    assert_eq!(ctx.stack, vec!["Rust", "CLI"]);
}
