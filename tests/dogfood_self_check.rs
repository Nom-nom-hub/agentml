use agentml::parser;
use agentml::types::AgentFile;
use agentml::validator;
use std::path::Path;
use std::path::PathBuf;

struct CwdGuard {
    original: std::path::PathBuf,
}

impl CwdGuard {
    fn new() -> Self {
        let original = std::env::current_dir().unwrap();
        Self { original }
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.original);
    }
}

fn skills_dir() -> PathBuf {
    std::env::current_dir().unwrap().join("skills")
}

#[test]
fn self_check_contract_is_valid() {
    let _guard = CwdGuard::new();
    let path = PathBuf::from("AGENT.agent");
    let agent: AgentFile = parser::parse_agent_file(&path).expect("AGENT.agent must parse");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

#[test]
fn self_check_skills_are_valid() {
    let _guard = CwdGuard::new();
    let skills_dir = skills_dir();
    if !skills_dir.exists() {
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
}

#[test]
fn self_check_docs_exist() {
    let _guard = CwdGuard::new();
    assert!(
        Path::new("docs/spec.md").exists(),
        "docs/spec.md must exist"
    );
    assert!(Path::new("README.md").exists(), "README.md must exist");
}

#[test]
fn self_check_readme_mentions_dogfooding() {
    let _guard = CwdGuard::new();
    let readme = std::fs::read_to_string("README.md").expect("README.md must exist");
    assert!(
        readme.contains("dogfood"),
        "README.md should mention dogfooding"
    );
}