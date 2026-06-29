use crate::parser;
use crate::types::AgentFile;
use anyhow::Result;
use colored::Colorize;
use serde_yaml;
use std::fs;
use std::path::Path;

pub fn run(file: std::path::PathBuf, output: Option<std::path::PathBuf>) -> Result<()> {
    let path = Path::new(&file);
    let agent: AgentFile = parser::parse_agent_file(path)?;

    let context_yaml = serde_yaml::to_string(&agent)?;

    let out = output.unwrap_or_else(|| std::path::PathBuf::from(".agentml/context.md"));
    fs::create_dir_all(out.parent().unwrap())?;
    std::fs::write(&out, &context_yaml)?;
    println!(
        "{} {}",
        "Context written to:".green(),
        out.display().to_string().cyan()
    );

    Ok(())
}
