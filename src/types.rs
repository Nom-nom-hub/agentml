use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn deserialize_purpose_opt<'de, D>(deserializer: D) -> Result<Option<Purpose>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<serde_yaml::Value>::deserialize(deserializer)?;
    match opt {
        Some(value) => match value {
            serde_yaml::Value::String(s) => Ok(Some(Purpose {
                human_goal: Some(s),
                agent_goal: None,
                non_goals: None,
            })),
            serde_yaml::Value::Mapping(map) => {
                let key_human = serde_yaml::Value::String("human_goal".to_string());
                let key_agent = serde_yaml::Value::String("agent_goal".to_string());
                let key_non_goals = serde_yaml::Value::String("non_goals".to_string());
                let human_goal = map
                    .get(&key_human)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let agent_goal = map
                    .get(&key_agent)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let non_goals = map
                    .get(&key_non_goals)
                    .and_then(|v| v.as_sequence())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    });
                Ok(Some(Purpose {
                    human_goal,
                    agent_goal,
                    non_goals,
                }))
            }
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AgentFile {
    pub meta: Option<AgentMeta>,
    #[serde(default, deserialize_with = "deserialize_purpose_opt")]
    pub purpose: Option<Purpose>,
    pub context: Option<AgentContext>,
    pub permissions: Option<Permissions>,
    pub tools: Option<Vec<String>>,
    pub workflows: Option<Vec<Workflow>>,
    pub tasks: Option<Vec<Task>>,
    pub memory: Option<String>,
    pub safety: Option<Safety>,
    pub validation: Option<Vec<ValidationCommand>>,
    pub success_criteria: Option<Vec<String>>,
    pub output: Option<OutputConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentMeta {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AgentContext {
    pub project_type: Option<String>,
    pub languages: Option<Vec<String>>,
    pub frameworks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Permissions {
    pub read: Option<Vec<String>>,
    pub write: Option<Vec<String>>,
    pub execute: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowStep {
    pub name: String,
    pub description: Option<String>,
    pub commands: Option<Vec<String>>,
    pub success: Option<String>,
    pub on_failure: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
    pub workflow: Option<String>,
    pub inputs: Option<HashMap<String, TaskInput>>,
    pub success: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskInput {
    pub description: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Purpose {
    pub human_goal: Option<String>,
    pub agent_goal: Option<String>,
    pub non_goals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Safety {
    pub policy: Option<String>,
    pub forbidden_paths: Option<Vec<String>>,
    pub forbidden_actions: Option<Vec<String>>,
    pub require_confirmation: Option<Vec<String>>,
    pub secrets_policy: Option<SecretsPolicy>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SecretsPolicy {
    pub never_read: Option<Vec<String>>,
    pub never_output_secret_values: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationCommand {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OutputConfig {
    pub format: Option<String>,
    pub required_sections: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SkillFile {
    pub skill: String,
    pub version: String,
    pub description: String,
    pub requirements: Option<Vec<String>>,
    pub inputs: Option<Vec<SkillInput>>,
    pub actions: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    pub success: Option<String>,
    pub output: Option<String>,
    pub applies_to: Option<SkillAppliesTo>,
    pub risk: Option<SkillRisk>,
    pub requires_validation: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SkillInput {
    pub name: String,
    pub description: String,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SkillAppliesTo {
    pub paths: Option<Vec<String>>,
    pub stacks: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SkillRisk {
    pub base_score: Option<u32>,
    pub high_risk_paths: Option<Vec<String>>,
}
