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
        Some(serde_yaml::Value::Mapping(map))
    } else {
        None
    };

    let safety_value = if let Some(ref s) = ast.safety {
        let mut map: serde_yaml::Mapping = Default::default();
        if !s.rules.is_empty() {
            map.insert(
                serde_yaml::Value::String("forbidden_actions".to_string()),
                serde_yaml::Value::Sequence(
                    s.rules
                        .iter()
                        .map(|r| serde_yaml::Value::String(r.clone()))
                        .collect(),
                ),
            );
        }
        if !s.secrets_never_read.is_empty() {
            map.insert(
                serde_yaml::Value::String("secrets_never_read".to_string()),
                serde_yaml::Value::Sequence(
                    s.secrets_never_read
                        .iter()
                        .map(|r| serde_yaml::Value::String(r.clone()))
                        .collect(),
                ),
            );
        }
        Some(serde_yaml::Value::Mapping(map))
    } else {
        None
    };

    let permissions = ast.permissions.as_ref().map(|p| crate::types::Permissions {
        read: Some(p.read.clone()),
        write: Some(p.write.clone()),
        execute: None,
    });

    let agent = AgentFile {
        meta: Some(crate::types::AgentMeta {
            name: ast.agent.clone(),
            version: ast.version.clone(),
            description: ast.description.clone(),
        }),
        purpose: purpose_value,
        context: ast.context.as_ref().map(|c| crate::types::AgentContext {
            project_type: c.stack.first().cloned(),
            languages: None,
            frameworks: None,
        }),
        permissions,
        safety: safety_value,
        validation: Some(validation_commands),
        ..Default::default()
    };
    Ok(agent)
}

pub fn convert_ast_to_skill(ast: &SkillAst) -> anyhow::Result<SkillFile> {
    let output_string = if !ast.output.final_report.is_empty() {
        Some(ast.output.final_report.join(", "))
    } else {
        None
    };

    let skill = SkillFile {
        skill: ast.skill.clone(),
        version: ast.version.clone(),
        description: ast.description.clone(),
        actions: Some(ast.rules.clone()),
        requires_validation: Some(ast.requires_validation.clone()),
        success: ast.success.items.first().cloned(),
        output: output_string,
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
