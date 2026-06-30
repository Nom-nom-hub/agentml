use agentml::commands::agents_md;
use agentml::parser::parse_agent_file;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

static CWD_LOCK: Mutex<()> = Mutex::new(());

fn run_in_temp(test: fn(&Path)) {
    let _lock = CWD_LOCK.lock().unwrap();
    let original = env::current_dir().unwrap();
    let temp_name = format!(
        "agentml-agents-md-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    );
    let temp_dir = env::temp_dir().join(temp_name);
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    test(&temp_dir);

    env::set_current_dir(&original).unwrap();
    let _ = fs::remove_dir_all(&temp_dir);
}

fn setup_test_project(root: &Path) {
    fs::create_dir_all(root.join(".agentml")).unwrap();
    fs::write(
        root.join("AGENT.agent"),
        r#"purpose: "test project"
permissions:
  read:
    - "src/**"
    - "docs/**"
  write:
    - "src/**/*.rs"
safety:
  forbidden_paths:
    - ".env"
    - "target/**"
  secrets_policy:
    never_read:
      - ".env"
validation:
  - name: Lint
    command: "cargo fmt --check"
  - name: Test
    command: "cargo test"
output:
  format: markdown
  required_sections:
    - "changes"
    - "tests"
"#,
    )
    .unwrap();
}

#[test]
fn agents_md_prints_valid_markdown() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("# AGENTS.md"));
        assert!(output.contains("## Purpose"));
        assert!(output.contains("## Required first steps"));
        assert!(output.contains("## Project context"));
        assert!(output.contains("## Allowed work areas"));
        assert!(output.contains("## Forbidden areas"));
        assert!(output.contains("## Validation commands"));
        assert!(output.contains("## Diff audit"));
        assert!(output.contains("## Final report format"));
        assert!(output.contains("## Source of truth"));
    });
}

#[test]
fn agents_md_write_creates_file() {
    run_in_temp(|_root| {
        setup_test_project(Path::new("."));
        assert!(!Path::new("AGENTS.md").exists());
        let result = agents_md::run(true, false);
        assert!(result.is_ok());
        assert!(Path::new("AGENTS.md").exists());
    });
}

#[test]
fn agents_md_does_not_overwrite_without_force() {
    run_in_temp(|_root| {
        setup_test_project(Path::new("."));
        fs::write("AGENTS.md", "existing content").unwrap();
        let result = agents_md::run(true, false);
        assert!(result.is_ok());
        let content = fs::read_to_string("AGENTS.md").unwrap();
        assert_eq!(content, "existing content");
    });
}

#[test]
fn agents_md_overwrites_with_force() {
    run_in_temp(|_root| {
        setup_test_project(Path::new("."));
        fs::write("AGENTS.md", "existing content").unwrap();
        let result = agents_md::run(true, true);
        assert!(result.is_ok());
        let content = fs::read_to_string("AGENTS.md").unwrap();
        assert_ne!(content, "existing content");
        assert!(content.contains("# AGENTS.md"));
    });
}

#[test]
fn generated_agents_md_includes_validation_commands() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("cargo fmt --check"));
        assert!(output.contains("cargo test"));
    });
}

#[test]
fn generated_agents_md_includes_forbidden_paths() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains(".env"));
        assert!(output.contains("target/**"));
    });
}

#[test]
fn generated_agents_md_includes_final_report_format() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("Summary:"));
        assert!(output.contains("Files changed:"));
        assert!(output.contains("Risk score:"));
        assert!(output.contains("Next steps:"));
    });
}

#[test]
fn agents_md_fails_without_agent_file() {
    run_in_temp(|_root| {
        let result = agents_md::run(false, false);
        assert!(result.is_err());
    });
}

#[test]
fn generated_agents_md_includes_maintenance_intelligence() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("## Maintenance Intelligence"));
        assert!(output.contains("Did CLI behavior change?"));
        assert!(output.contains("Did the website need a matching update?"));
        assert!(output.contains("Did CHANGELOG.md need an entry?"));
    });
}

#[test]
fn generated_agents_md_includes_documentation_sync_rule() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("## Documentation Sync Rule"));
        assert!(
            output.contains("Code changes are incomplete if user-facing documentation is stale")
        );
    });
}

#[test]
fn generated_agents_md_includes_agent_self_update_rule() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("## Agent Self-Update Rule"));
        assert!(output.contains("Agents may update `AGENTS.md`"));
    });
}

#[test]
fn generated_agents_md_includes_pre_final_checklist() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("## Pre-Final Checklist"));
        assert!(output.contains("Risk score is included in the final report"));
        assert!(output.contains("`README.md` is updated"));
        assert!(output.contains("`CHANGELOG.md` is updated"));
        assert!(output.contains("`AGENT.agent` is updated"));
    });
}

#[test]
fn doctor_warns_when_agents_md_lacks_required_sections() {
    run_in_temp(|root| {
        setup_test_project(root);
        // Write a minimal AGENTS.md that's missing required sections
        fs::write(root.join("AGENTS.md"), "# Minimal\n\nNo sections here.\n").unwrap();
        let missing = agentml::commands::doctor::check_agents_md_quality();
        assert!(!missing.is_empty());
        assert!(missing.contains(&"## Purpose"));
        assert!(missing.contains(&"## Maintenance Intelligence"));
        assert!(missing.contains(&"## Pre-Final Checklist"));
    });
}

#[test]
fn doctor_passes_when_agents_md_includes_required_sections() {
    run_in_temp(|root| {
        setup_test_project(root);
        // Generate a proper AGENTS.md
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);
        fs::write(root.join("AGENTS.md"), &output).unwrap();
        let missing = agentml::commands::doctor::check_agents_md_quality();
        assert!(missing.is_empty());
    });
}

#[test]
fn repo_agents_md_includes_update_rules() {
    // Test the actual repo AGENTS.md
    let content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTS.md")).unwrap();

    assert!(
        content.contains("README.md"),
        "Must mention updating README.md"
    );
    assert!(content.contains("docs/"), "Must mention updating docs/");
    assert!(content.contains("website"), "Must mention updating website");
    assert!(
        content.contains("CHANGELOG.md"),
        "Must mention updating CHANGELOG.md"
    );
    assert!(
        content.contains("examples"),
        "Must mention updating examples"
    );
    assert!(
        content.contains("AGENT.agent"),
        "Must mention updating AGENT.agent"
    );
}

#[test]
fn repo_agents_md_includes_git_workflow() {
    let content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTS.md")).unwrap();
    assert!(
        content.contains("## Git Workflow"),
        "Repo AGENTS.md must include Git Workflow section"
    );
    assert!(
        content.contains("git status --short"),
        "Git Workflow must mention git status"
    );
    assert!(
        content.contains("agentml diff"),
        "Git Workflow must mention agentml diff"
    );
}

#[test]
fn repo_agents_md_includes_task_closure_rule() {
    let content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTS.md")).unwrap();
    assert!(
        content.contains("## Task Closure Rule"),
        "Repo AGENTS.md must include Task Closure Rule section"
    );
    assert!(
        content.contains("Changes are committed"),
        "Task Closure Rule must mention committing"
    );
}

#[test]
fn repo_agents_md_final_report_includes_commit() {
    let content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTS.md")).unwrap();
    assert!(
        content.contains("Commit:"),
        "Final report format must include Commit: field"
    );
}

#[test]
fn generated_agents_md_includes_git_workflow() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(output.contains("## Git Workflow"));
        assert!(output.contains("git status --short"));
        assert!(output.contains("agentml diff"));
    });
}

#[test]
fn generated_agents_md_final_report_includes_commit() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);

        assert!(
            output.contains("Commit:"),
            "Generated report format must include Commit:"
        );
        assert!(
            output.contains("Summary:"),
            "Generated report format must include Summary:"
        );
        assert!(
            output.contains("Files changed:"),
            "Generated report format must include Files changed:"
        );
    });
}

#[test]
fn doctor_warns_when_agents_md_lacks_git_workflow() {
    run_in_temp(|root| {
        setup_test_project(root);
        fs::write(
            root.join("AGENTS.md"),
            "## Purpose\n\nSome purpose.\n## Final report format\n\n```\nSummary:\nFiles changed:\nCommands run:\nValidation result:\nRisk score:\nCommit:\nRisks:\nNext steps:\n```\n",
        )
        .unwrap();
        let missing = agentml::commands::doctor::check_agents_md_quality();
        assert!(missing.contains(&"## Git Workflow"));
    });
}

#[test]
fn doctor_warns_when_final_report_lacks_commit() {
    run_in_temp(|root| {
        setup_test_project(root);
        fs::write(
            root.join("AGENTS.md"),
            "## Purpose\n\nSome purpose.\n## Final report format\n\n```\nSummary:\nFiles changed:\n```\n## Git Workflow\n\nContent\n## Task Closure Rule\n\nContent\n## Pre-Final Checklist\n\nContent\n## Source of truth\n\nContent\n## Validation commands\n\nContent\n## Required first steps\n\nContent\n## Maintenance Intelligence\n\nContent\n",
        )
        .unwrap();
        let missing = agentml::commands::doctor::check_agents_md_quality();
        assert!(missing.contains(&"Commit: in final report format"));
    });
}

#[test]
fn doctor_passes_with_all_required_sections() {
    run_in_temp(|root| {
        setup_test_project(root);
        let agent = parse_agent_file(&root.join("AGENT.agent")).unwrap();
        let output = agents_md::generate(&agent);
        fs::write(root.join("AGENTS.md"), &output).unwrap();
        let missing = agentml::commands::doctor::check_agents_md_quality();
        assert!(missing.is_empty());
    });
}
