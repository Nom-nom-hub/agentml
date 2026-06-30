use crate::syntax::ast::{AgentAst, SkillAst};
use crate::types::{AgentFile, SkillFile};

#[allow(unused_variables)]
pub fn convert_ast_to_agent(ast: &AgentAst) -> anyhow::Result<AgentFile> {
    let agent = AgentFile::default();
    Ok(agent)
}

pub fn convert_ast_to_skill(ast: &SkillAst) -> anyhow::Result<SkillFile> {
    let skill = SkillFile {
        skill: ast.skill.clone(),
        version: ast.version.clone(),
        description: ast.description.clone(),
        actions: Some(ast.rules.clone()),
        ..Default::default()
    };
    Ok(skill)
}
