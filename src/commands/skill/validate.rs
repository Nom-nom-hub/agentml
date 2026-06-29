use crate::parser;
use crate::types::SkillFile;
use crate::validator;
use std::path::Path;

pub fn run(file: std::path::PathBuf) -> anyhow::Result<()> {
    let path = Path::new(&file);
    let skill: SkillFile = parser::parse_skill_file(path)?;
    let report = validator::validate_skill_file(&skill);
    report.print();
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}
