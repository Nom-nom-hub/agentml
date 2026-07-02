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

fn deserialize_diff_policy<'de, D>(deserializer: D) -> Result<Option<DiffPolicy>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_yaml::Value::deserialize(deserializer)?;
    match value {
        serde_yaml::Value::Mapping(map) => {
            let strict_ci = map
                .get(serde_yaml::Value::String("strict_ci".to_string()))
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let fail_at_risk_score = map
                .get(serde_yaml::Value::String("fail_at_risk_score".to_string()))
                .and_then(|v| v.as_u64())
                .unwrap_or(80) as u32;
            let require_tests_for_src_changes = map
                .get(serde_yaml::Value::String(
                    "require_tests_for_src_changes".to_string(),
                ))
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let watched_paths: Vec<WatchedPath> = match map
                .get(serde_yaml::Value::String("watched_paths".to_string()))
            {
                Some(serde_yaml::Value::Sequence(arr)) => arr
                    .iter()
                    .filter_map(|v| {
                        if let serde_yaml::Value::Mapping(m) = v {
                            Some(WatchedPath {
                                path: m
                                    .get(serde_yaml::Value::String("path".to_string()))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string()),
                                risk: m
                                    .get(serde_yaml::Value::String("risk".to_string()))
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0) as u32,
                                requires: m
                                    .get(serde_yaml::Value::String("requires".to_string()))
                                    .and_then(|v| v.as_sequence())
                                    .map(|arr| {
                                        arr.iter()
                                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default(),
                                ..Default::default()
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
                Some(serde_yaml::Value::Mapping(m)) => m
                    .iter()
                    .filter_map(|(k, v)| {
                        if let serde_yaml::Value::String(name) = k {
                            let path_val: Option<Vec<String>> = v
                                .get(serde_yaml::Value::String("paths".to_string()))
                                .and_then(|pv| pv.as_sequence())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect()
                                });
                            let paths_first: Option<String> = path_val
                                .as_ref()
                                .and_then(|p: &Vec<String>| p.first().cloned());
                            Some(WatchedPath {
                                name: Some(name.clone()),
                                path: paths_first,
                                paths: path_val,
                                risk: v
                                    .get(serde_yaml::Value::String("risk".to_string()))
                                    .and_then(|rv| rv.as_u64())
                                    .unwrap_or(0) as u32,
                                requires: v
                                    .get(serde_yaml::Value::String("requires".to_string()))
                                    .and_then(|rv| rv.as_sequence())
                                    .map(|arr| {
                                        arr.iter()
                                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => Vec::new(),
            };

            Ok(Some(DiffPolicy {
                strict_ci,
                fail_at_risk_score,
                require_tests_for_src_changes,
                watched_paths,
            }))
        }
        _ => Ok(None),
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
    #[serde(default, deserialize_with = "deserialize_diff_policy")]
    pub diff_policy: Option<DiffPolicy>,
    pub output: Option<OutputConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentMeta {
    pub name: String,
    pub version: String,
    pub contract_version: Option<u32>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AgentContext {
    pub project_type: Option<String>,
    pub languages: Option<Vec<String>>,
    pub frameworks: Option<Vec<String>>,
    pub stack: Option<Vec<String>>,
}

impl AgentContext {
    pub fn stack_items(&self) -> Vec<String> {
        if let Some(ref stack) = self.stack {
            return stack.clone();
        }
        let mut result = Vec::new();
        if let Some(pt) = &self.project_type {
            result.push(pt.clone());
        }
        if let Some(langs) = &self.languages {
            for lang in langs {
                result.push(lang.clone());
            }
        }
        if let Some(fw) = &self.frameworks {
            for f in fw {
                result.push(f.clone());
            }
        }
        result
    }
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
pub struct DiffPolicy {
    pub strict_ci: bool,
    pub fail_at_risk_score: u32,
    pub require_tests_for_src_changes: bool,
    pub watched_paths: Vec<WatchedPath>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WatchedPath {
    pub name: Option<String>,
    pub path: Option<String>,
    pub paths: Option<Vec<String>>,
    pub risk: u32,
    pub requires: Vec<String>,
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
