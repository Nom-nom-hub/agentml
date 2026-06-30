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
