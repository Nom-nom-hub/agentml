use crate::parser;
use crate::types::SkillFile;
use crate::validator;
use std::path::Path;

pub fn run(file: std::path::PathBuf, format: &str) -> anyhow::Result<()> {
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

    let skill: SkillFile = if is_native {
        crate::syntax::parse_native_skill(path)?
    } else {
        parser::parse_skill_file(path)?
    };

    let report = validator::validate_skill_file(&skill);
    report.print();
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}
