use agentml::parser;
use agentml::validator;
use std::path::Path;
use std::path::PathBuf;

fn skills_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("skills")
}

#[test]
fn every_skill_file_validates() {
    let skills_dir = skills_dir();
    if !skills_dir.exists() {
        return;
    }
    let entries = std::fs::read_dir(&skills_dir).expect("skills dir must exist");
    for entry in entries {
        let entry = entry.expect("entry must be readable");
        let path = entry.path();
        if path.extension().map(|e| e == "skill").unwrap_or(false) {
            let skill = parser::parse_skill_file(&path).expect("skill parse should succeed");
            let report = validator::validate_skill_file(&skill);
            assert!(
                report.valid,
                "Skill {:?} failed validation: {:?}",
                path, report.errors
            );
        }
    }
}
