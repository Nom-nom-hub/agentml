use crate::parser;
use crate::types::AgentFile;
use crate::validator;
use std::path::Path;

pub fn run(file: std::path::PathBuf, strict: bool) -> anyhow::Result<()> {
    let path = Path::new(&file);
    let agent: AgentFile = parser::parse_agent_file(path)?;
    let report = validator::validate_agent_file(&agent, strict);
    report.print();
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}
