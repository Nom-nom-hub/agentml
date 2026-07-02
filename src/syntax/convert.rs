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

    let success_criteria = ast
        .validation
        .as_ref()
        .map(|v| v.success.clone())
        .unwrap_or_default();

    let purpose = ast.purpose.as_ref().map(|p| crate::types::Purpose {
        human_goal: Some(p.human_goal.clone()),
        agent_goal: Some(p.agent_goal.clone()),
        non_goals: Some(p.non_goals.clone()),
    });

    let safety = ast.safety.as_ref().map(|s| crate::types::Safety {
        policy: s.rules.first().cloned(),
        forbidden_paths: if s.forbidden_paths.is_empty() {
            None
        } else {
            Some(s.forbidden_paths.clone())
        },
        forbidden_actions: if s.forbidden_actions.is_empty() {
            None
        } else {
            Some(s.forbidden_actions.clone())
        },
        require_confirmation: if s.require_approval.is_empty() {
            None
        } else {
            Some(s.require_approval.clone())
        },
        secrets_policy: None,
    });

    let permissions = ast.permissions.as_ref().map(|p| crate::types::Permissions {
        read: Some(p.read.clone()),
        write: Some(p.write.clone()),
        execute: None,
    });

    let diff_policy = ast.diff_policy.as_ref().map(|dp| crate::types::DiffPolicy {
        strict_ci: dp.strict_ci,
        fail_at_risk_score: dp.fail_at_risk_score,
        require_tests_for_src_changes: dp.require_tests_for_src_changes,
        watched_paths: dp
            .watched_paths
            .iter()
            .map(|wp| crate::types::WatchedPath {
                name: None,
                path: Some(wp.path.clone()),
                paths: None,
                risk: wp.risk,
                requires: wp.requires.clone(),
            })
            .collect(),
    });

    let output = ast.output.as_ref().and_then(|o| {
        if o.final_report.is_empty() {
            None
        } else {
            Some(crate::types::OutputConfig {
                format: None,
                required_sections: Some(o.final_report.clone()),
            })
        }
    });

    let agent = AgentFile {
        meta: Some(crate::types::AgentMeta {
            name: ast.agent.clone(),
            version: ast.version.clone(),
            contract_version: ast.contract_version,
            description: ast.description.clone(),
        }),
        purpose,
        context: ast.context.as_ref().map(|c| crate::types::AgentContext {
            stack: if c.stack.is_empty() {
                None
            } else {
                Some(c.stack.clone())
            },
            project_type: c.stack.first().cloned(),
            languages: None,
            frameworks: None,
        }),
        permissions,
        safety,
        validation: Some(validation_commands),
        success_criteria: Some(success_criteria),
        diff_policy,
        output,
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
