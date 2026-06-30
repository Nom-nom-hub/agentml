use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AgentFile {
    pub meta: Option<AgentMeta>,
    pub purpose: Option<Value>,
    pub context: Option<AgentContext>,
    pub permissions: Option<Permissions>,
    pub tools: Option<Vec<String>>,
    pub workflows: Option<Vec<Workflow>>,
    pub tasks: Option<Vec<Task>>,
    pub memory: Option<String>,
    pub safety: Option<Value>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Safety {
    pub policy: Option<String>,
    pub forbidden_paths: Option<Vec<String>>,
    pub forbidden_actions: Option<Vec<String>>,
    pub require_confirmation: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationCommand {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
