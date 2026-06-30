use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentAst {
    pub agent: String,
    pub version: String,
    pub contract_version: Option<u32>,
    pub description: Option<String>,
    pub purpose: Option<PurposeAst>,
    pub context: Option<ContextAst>,
    pub permissions: Option<PermissionsAst>,
    pub safety: Option<SafetyAst>,
    pub validation: Option<ValidationAst>,
    pub diff_policy: Option<DiffPolicyAst>,
    pub output: Option<OutputAst>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PurposeAst {
    pub human_goal: String,
    pub agent_goal: String,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextAst {
    pub stack: Vec<String>,
    pub important_files: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsAst {
    pub read: Vec<String>,
    pub write: Vec<String>,
    pub forbidden: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SafetyAst {
    pub secrets_never_read: Vec<String>,
    pub destructive_actions: Vec<DestructiveAction>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructiveAction {
    pub action: String,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationAst {
    pub commands: Vec<String>,
    pub success: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffPolicyAst {
    pub strict_ci: bool,
    pub fail_at_risk_score: u32,
    pub require_tests_for_src_changes: bool,
    pub watched_paths: Vec<WatchedPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedPath {
    pub path: String,
    pub risk: u32,
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutputAst {
    pub final_report: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillAst {
    pub skill: String,
    pub version: String,
    pub description: String,
    pub applies_to: Option<AppliesToAst>,
    pub risk: Option<RiskAst>,
    pub requires_validation: Vec<String>,
    pub rules: Vec<String>,
    pub success: SuccessAst,
    pub output: OutputAst,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppliesToAst {
    pub paths: Vec<String>,
    pub stacks: Vec<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RiskAst {
    pub base_score: u32,
    pub high_risk_paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuccessAst {
    pub items: Vec<String>,
}
