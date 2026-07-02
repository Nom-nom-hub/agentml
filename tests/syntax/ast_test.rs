#[test]
fn context_ast_default_stack_is_empty() {
    let ctx = agentml::syntax::ast::ContextAst::default();
    assert!(ctx.stack.is_empty());
}

#[test]
fn agent_ast_default_fields() {
    let agent = agentml::syntax::ast::AgentAst::default();
    assert_eq!(agent.agent, "");
}
