use agentml::parser;
use agentml::types::AgentFile;
use agentml::validator;
use std::path::Path;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn skills_dir() -> PathBuf {
    project_root().join("skills")
}

#[test]
fn self_check_contract_is_valid() {
    let path = project_root().join("AGENT.agent");
    if !path.exists() {
        panic!("AGENT.agent must exist - run tests from project root");
    }
    let agent: AgentFile = parser::parse_agent_file(&path).expect("AGENT.agent must parse");
    let report = validator::validate_agent_file(&agent, false);
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

#[test]
fn self_check_skills_are_valid() {
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
    assert!(
        project_root().join("docs/spec.md").exists(),
        "docs/spec.md must exist"
    );
    assert!(
        project_root().join("README.md").exists(),
        "README.md must exist"
    );
}

#[test]
fn self_check_readme_mentions_dogfooding() {
    let readme =
        std::fs::read_to_string(project_root().join("README.md")).expect("README.md must exist");
    assert!(
        readme.contains("dogfood"),
        "README.md should mention dogfooding"
    );
}
