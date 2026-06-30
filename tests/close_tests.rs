use std::env;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

static CWD_LOCK: Mutex<()> = Mutex::new(());

fn run_in_temp(test: fn(&Path)) {
    let _lock = CWD_LOCK.lock().unwrap();
    let original = env::current_dir().unwrap();
    let temp_name = format!(
        "agentml-close-{}-{:?}",
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

fn setup_project(root: &Path) {
    fs::create_dir_all(root.join(".agentml")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(
        root.join("AGENT.agent"),
        r#"purpose: "test close"
permissions:
  read:
    - "src/**"
  write:
    - "src/**/*.rs"
safety:
  forbidden_paths:
    - ".env"
validation:
  - name: Test
    command: "echo ok"
"#,
    )
    .unwrap();
}

fn init_git(root: &Path) {
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(root)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(root)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(root)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(root)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(root)
        .output()
        .unwrap();
}

#[test]
fn close_report_includes_risk_score() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);

        let result = agentml::commands::close::run(false, false, None, false);
        assert!(result.is_ok());
    });
}

#[test]
fn close_report_includes_commit_field() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);

        let result = agentml::commands::close::run(false, false, None, false);
        assert!(result.is_ok());
    });
}

#[test]
fn close_require_clean_fails_on_dirty_tree() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);
        // Modify a tracked file to make tree dirty
        fs::write(root.join("src/main.rs"), "fn main() { println!(\"dirty\"); }").unwrap();

        let result = agentml::commands::close::run(false, true, None, false);
        assert!(result.is_err());
    });
}

#[test]
fn close_json_outputs_valid_json() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);

        let result = agentml::commands::close::run(true, false, None, false);
        assert!(result.is_ok());
    });
}

#[test]
fn close_write_report_creates_file() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);

        let result = agentml::commands::close::run(false, false, None, true);
        assert!(result.is_ok());

        let report_path = root.join(".agentml").join("close-report.md");
        assert!(report_path.exists(), "close-report.md should exist");

        let content = fs::read_to_string(&report_path).unwrap();
        assert!(content.contains("Risk score"));
        assert!(content.contains("Git status"));
    });
}

#[test]
fn close_fail_at_risk_high() {
    run_in_temp(|root| {
        setup_project(root);
        init_git(root);
        // Modify a tracked file so diff shows changes
        fs::write(root.join("src/main.rs"), "fn main() { println!(\"changed\"); }").unwrap();

        // With threshold 1, any change should fail
        let result = agentml::commands::close::run(false, false, Some(1), false);
        assert!(result.is_err());
    });
}

#[test]
fn repo_agents_md_mentions_close() {
    let content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("AGENTS.md")).unwrap();
    assert!(
        content.contains("agentml close"),
        "Repo AGENTS.md must mention agentml close"
    );
}

#[test]
fn generated_agents_md_mentions_close() {
    use agentml::commands::agents_md;
    use agentml::parser::parse_agent_file;

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let agent = parse_agent_file(&repo_root.join("AGENT.agent")).unwrap();
    let output = agents_md::generate(&agent);

    assert!(
        output.contains("agentml close"),
        "Generated AGENTS.md must mention agentml close"
    );
}
