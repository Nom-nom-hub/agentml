use crate::syntax;
use crate::types::{AgentFile, SkillFile};
use anyhow::{Context, Result};
use serde_yaml;
use std::fs;
use std::path::Path;

pub fn parse_agent_file(path: &Path) -> Result<AgentFile> {
    let content = fs::read_to_string(path).with_context(|| format!("Cannot read {:?}", path))?;

    if syntax::is_native_syntax(&content) {
        syntax::parse_native_agent(path)
            .with_context(|| "Failed to parse AGENT.agent native syntax")
    } else {
        let file: AgentFile =
            serde_yaml::from_str(&content).with_context(|| "Failed to parse AGENT.agent YAML")?;
        Ok(file)
    }
}

pub fn parse_skill_file(path: &Path) -> Result<SkillFile> {
    let content = fs::read_to_string(path).with_context(|| format!("Cannot read {:?}", path))?;

    if syntax::is_native_syntax(&content) {
        syntax::parse_native_skill(path).with_context(|| "Failed to parse .skill native syntax")
    } else {
        let file: SkillFile =
            serde_yaml::from_str(&content).with_context(|| "Failed to parse .skill YAML")?;
        Ok(file)
    }
}

pub fn serialize_agent_file(file: &AgentFile) -> Result<String> {
    Ok(serde_yaml::to_string(file)?)
}
