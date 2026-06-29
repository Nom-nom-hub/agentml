use agentml::parser;
use agentml::types::AgentFile;
use agentml::validator;
use std::path::Path;

#[test]
fn self_check_contract_is_valid() {
    let path = Path::new("AGENT.agent");
    let agent: AgentFile = parser::parse_agent_file(path).expect("AGENT.agent must parse");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

#[test]
fn self_check_skills_are_valid() {
    let skills_dir = Path::new("skills");
    if !skills_dir.exists() {
        return;
    }
    for entry in std::fs::read_dir(skills_dir).expect("skills dir") {
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
    assert!(
        Path::new("docs/spec.md").exists(),
        "docs/spec.md must exist"
    );
    assert!(Path::new("README.md").exists(), "README.md must exist");
}

#[test]
fn self_check_readme_mentions_dogfooding() {
    let readme = std::fs::read_to_string("README.md").expect("README.md must exist");
    assert!(
        readme.contains("dogfood"),
        "README.md should mention dogfooding"
    );
}
