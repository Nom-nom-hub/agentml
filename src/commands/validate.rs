use crate::parser;
use crate::types::AgentFile;
use crate::validator;
use std::path::Path;

pub fn run(file: std::path::PathBuf, strict: bool, format: &str) -> anyhow::Result<()> {
    let path = Path::new(&file);
    let content = std::fs::read_to_string(path)?;

    let is_native = crate::syntax::is_native_syntax(&content);
    let parsed_format = if is_native { "native" } else { "yaml" };

    if format != "auto" && format != parsed_format {
        anyhow::bail!(
            "{} was parsed as {}, but `--format {}` was requested.\nUse `--format {}` or `--format auto`.",
            path.display(),
            parsed_format,
            format,
            parsed_format
        );
    }

    let agent: AgentFile = if is_native {
        crate::syntax::parse_native_agent(path)?
    } else {
        parser::parse_agent_file(path)?
    };

    let report = validator::validate_agent_file(&agent, strict);
    report.print();
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}
