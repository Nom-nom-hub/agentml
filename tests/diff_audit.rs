use agentml::commands::diff::{ChangedFile, RiskReport, calculate_risk, check_permissions};
use agentml::types::{AgentFile, DiffPolicy, WatchedPath};

fn make_agent_file() -> AgentFile {
    AgentFile::default()
}

fn agent_with_watched_path(path: &str, requires: Vec<&str>) -> AgentFile {
    AgentFile {
        diff_policy: Some(DiffPolicy {
            watched_paths: vec![WatchedPath {
                path: Some(path.to_string()),
                requires: requires.into_iter().map(String::from).collect(),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
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

#[test]
fn diff_does_not_count_uncompiled_nested_test_file() {
    let agent = make_agent_file();
    let files = vec![ChangedFile {
        path: "src/commands/diff.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    // After promotion, nested test files may still exist on disk. Risk system
    // must NOT accept unwired nested tests as real coverage - only flat top-level
    // tests (compiled by Cargo) count as coverage.
    let has_test_risk = report
        .issues
        .iter()
        .any(|i| i.contains("source changed without tests"));
    let cwd = std::env::current_dir().unwrap();
    let in_repo = cwd.join("Cargo.toml").exists();
    if !in_repo {
        return;
    }
    // src/commands/diff.rs has no flat test file (tests/commands_diff_test.rs)
    // so it must get +20 risk regardless of any nested files
    assert!(
        has_test_risk,
        "Uncompiled nested test files must not count as coverage. Issues: {:?}",
        report.issues
    );
}

#[test]
fn diff_uses_watched_path_required_tests() {
    let agent = agent_with_watched_path("src/commands/diff.rs", vec!["tests/diff_audit.rs"]);
    let files = vec![ChangedFile {
        path: "src/commands/diff.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    let cwd = std::env::current_dir().unwrap();
    let in_repo = cwd.join("Cargo.toml").exists();
    if !in_repo {
        return;
    }
    // tests/diff_audit.rs exists, so no +20 risk from required test
    let has_test_risk = report
        .issues
        .iter()
        .any(|i| i.contains("source changed without tests"));
    assert!(
        !has_test_risk,
        "Explicit watched-path requires should satisfy risk check. Issues: {:?}",
        report.issues
    );
}

#[test]
fn diff_maps_diff_command_to_diff_audit_test() {
    let agent = agent_with_watched_path("src/commands/diff.rs", vec!["tests/diff_audit.rs"]);
    let files = vec![ChangedFile {
        path: "src/commands/diff.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    let cwd = std::env::current_dir().unwrap();
    let in_repo = cwd.join("Cargo.toml").exists();
    if !in_repo {
        return;
    }
    // Should NOT fall back to conventional tests/commands_diff_test.rs
    let has_conventional_issue = report
        .issues
        .iter()
        .any(|i| i.contains("tests/commands_diff_test.rs"));
    assert!(
        !has_conventional_issue,
        "Should not fall back to conventional name when explicit mapping exists. Issues: {:?}",
        report.issues
    );
}

#[test]
fn diff_does_not_require_wrong_conventional_name_when_explicit_mapping_exists() {
    let agent = agent_with_watched_path("src/commands/diff.rs", vec!["tests/diff_audit.rs"]);
    let files = vec![ChangedFile {
        path: "src/commands/diff.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    let cwd = std::env::current_dir().unwrap();
    let in_repo = cwd.join("Cargo.toml").exists();
    if !in_repo {
        return;
    }
    // Should NOT mention tests/commands_diff_test.rs at all
    let mentions_conventional = report
        .issues
        .iter()
        .any(|i| i.contains("commands_diff_test"));
    assert!(
        !mentions_conventional,
        "Explicit mapping should suppress conventional name guess. Issues: {:?}",
        report.issues
    );
}

#[test]
fn diff_warns_when_required_test_file_missing() {
    let agent = agent_with_watched_path("src/commands/diff.rs", vec!["tests/nonexistent_test.rs"]);
    let files = vec![ChangedFile {
        path: "src/commands/diff.rs".to_string(),
    }];
    let mut report = RiskReport::default();
    calculate_risk(&files, &agent, &mut report);
    // Required test file doesn't exist, so +20 should be added
    let has_missing_issue = report
        .issues
        .iter()
        .any(|i| i.contains("required test(s) missing"));
    assert!(
        has_missing_issue,
        "Should warn when required test file is missing. Issues: {:?}",
        report.issues
    );
    assert!(
        report.score >= 20,
        "Risk score should be at least 20 when required test is missing"
    );
}
