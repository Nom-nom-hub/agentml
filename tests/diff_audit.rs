use agentml::commands::diff::{calculate_risk, check_permissions, ChangedFile, RiskReport};
use agentml::types::AgentFile;

fn make_agent_file() -> AgentFile {
    AgentFile::default()
}

#[test]
fn test_diff_allows_normal_src_changes() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "src/main.rs".to_string(),
    }];
    let results = check_permissions(&files, &agent);
    assert!(!results.is_empty());
}

#[test]
fn test_diff_raises_risk_for_validator_changes() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "src/validator.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    assert!(report.score > 0);
}

#[test]
fn test_diff_raises_risk_when_src_changes_without_tests() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "src/parser.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    assert!(report.score >= 20);
}

#[test]
fn test_diff_recognizes_agent_file_changes() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "AGENT.agent".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    assert!(report.score >= 30);
}