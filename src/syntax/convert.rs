use crate::syntax::ast::{AgentAst, SkillAst};
use crate::types::{AgentFile, SkillFile, ValidationCommand};

pub fn convert_ast_to_agent(ast: &AgentAst) -> anyhow::Result<AgentFile> {
    let validation_commands = ast
        .validation
        .as_ref()
        .map(|v| {
            v.commands
                .iter()
                .enumerate()
                .map(|(i, c)| ValidationCommand {
                    name: format!("cmd_{}", i),
                    command: c.clone(),
                    description: None,
                })
                .collect()
        })
        .unwrap_or_default();

    let purpose_value = if let Some(ref p) = ast.purpose {
        let mut map: serde_yaml::Mapping = Default::default();
        map.insert(
            serde_yaml::Value::String("human_goal".to_string()),
            serde_yaml::Value::String(p.human_goal.clone()),
        );
        map.insert(
            serde_yaml::Value::String("agent_goal".to_string()),
            serde_yaml::Value::String(p.agent_goal.clone()),
        );
        map.insert(
            serde_yaml::Value::String("non_goals".to_string()),
            serde_yaml::Value::Sequence(
                p.non_goals
                    .iter()
                    .map(|s| serde_yaml::Value::String(s.clone()))
                    .collect(),
            ),
        );
        serde_yaml::Value::Mapping(map)
    } else {
        serde_yaml::Value::Null
    };

    let agent = AgentFile {
        meta: Some(crate::types::AgentMeta {
            name: ast.agent.clone(),
            version: ast.version.clone(),
            description: ast.description.clone(),
        }),
        purpose: Some(purpose_value),
        context: None,
        permissions: ast.permissions.as_ref().map(|p| crate::types::Permissions {
            read: Some(p.read.clone()),
            write: Some(p.write.clone()),
            execute: None,
        }),
        safety: None,
        validation: Some(validation_commands),
        ..Default::default()
    };
    Ok(agent)
}

pub fn convert_ast_to_skill(ast: &SkillAst) -> anyhow::Result<SkillFile> {
    let skill = SkillFile {
        skill: ast.skill.clone(),
        version: ast.version.clone(),
        description: ast.description.clone(),
        actions: Some(ast.rules.clone()),
        requires_validation: Some(ast.requires_validation.clone()),
        applies_to: ast
            .applies_to
            .as_ref()
            .map(|a| crate::types::SkillAppliesTo {
                paths: Some(a.paths.clone()),
                stacks: Some(a.stacks.clone()),
                keywords: Some(a.keywords.clone()),
            }),
        ..Default::default()
    };
    Ok(skill)
}
