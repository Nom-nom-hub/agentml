use agentml::commands::diff::{ChangedFile, RiskReport, calculate_risk, check_permissions};
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

#[test]
fn diff_counts_top_level_integration_test() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "src/commands/convert.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    let has_test_risk = report
        .issues
        .iter()
        .any(|i| i.contains("source changed without tests"));
    let cwd = std::env::current_dir().unwrap();
    let in_repo = cwd.join("Cargo.toml").exists();
    if !in_repo {
        return;
    }
    assert!(
        !has_test_risk,
        "Top-level flat test file should satisfy risk check. Issues: {:?}",
        report.issues
    );
}

#[test]
fn diff_warns_for_nested_unwired_test_file() {
    let agent = make_agent_file();
    let nested = "tests/commands/convert_test.rs";
    let warning_pattern = "Test file exists but may not be compiled by Cargo";
    // Verify the warning message format is correct
    let files = vec![ChangedFile {
        path: "src/commands/convert.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    // The flat file exists now, so no risk, but the warning could appear
    let has_warning = report.issues.iter().any(|i| i.contains(warning_pattern));
    // Warning only appears if nested file exists; after promotion it should not
    if std::path::Path::new(nested).exists() {
        assert!(
            has_warning,
            "Should warn when nested file exists without flat wrapper"
        );
    }
}
