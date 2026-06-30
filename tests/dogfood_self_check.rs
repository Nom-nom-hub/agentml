use agentml::parser;
use agentml::types::AgentFile;
use agentml::validator;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn skills_dir() -> PathBuf {
    env::current_dir().unwrap().join("skills")
}

#[test]
fn self_check_contract_is_valid() {
    let original = env::current_dir().unwrap();
    let path = PathBuf::from("AGENT.agent");
    let agent: AgentFile = parser::parse_agent_file(&path).expect("AGENT.agent must parse");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
    assert!(report.errors.is_empty());
    let _ = env::set_current_dir(&original);
}

#[test]
fn self_check_skills_are_valid() {
    let original = env::current_dir().unwrap();
    let skills_dir = skills_dir();
    if !skills_dir.exists() {
        let _ = env::set_current_dir(&original);
        return;
    }
    for entry in std::fs::read_dir(&skills_dir).expect("skills dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().map(|e| e == "skill").unwrap_or(false) {
            let skill = parser::parse_skill_file(&path).expect("skill parse");
            let report = validator::validate_skill_file(&skill);
            assert!(
                report.valid,
                "Skill {:?} invalid: {:?}",
                path, report.errors
            );
        }
    }
    let _ = env::set_current_dir(&original);
}

#[test]
fn self_check_docs_exist() {
    let original = env::current_dir().unwrap();
    assert!(
        Path::new("docs/spec.md").exists(),
        "docs/spec.md must exist"
    );
    assert!(Path::new("README.md").exists(), "README.md must exist");
    let _ = env::set_current_dir(&original);
}

#[test]
fn self_check_readme_mentions_dogfooding() {
    let original = env::current_dir().unwrap();
    let readme = std::fs::read_to_string("README.md").expect("README.md must exist");
    assert!(
        readme.contains("dogfood"),
        "README.md should mention dogfooding"
    );
    let _ = env::set_current_dir(&original);
}