use agentml::commands::doctor;
use std::env;
use std::fs;
use std::path::Path;

fn setup_generic_init(root: &Path) {
    fs::create_dir_all(root.join("skills")).unwrap();
    fs::create_dir_all(root.join(".agentml")).unwrap();
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::create_dir_all(root.join(".github").join("workflows")).unwrap();
    fs::write(root.join("AGENT.agent"), "purpose: test\n").unwrap();
    fs::write(root.join("skills").join(".gitkeep"), "").unwrap();
    fs::write(root.join(".agentml").join("context.md"), "# context\n").unwrap();
    fs::write(root.join("docs").join("agentml.md"), "# agentml docs\n").unwrap();
    fs::write(
        root.join(".github")
            .join("workflows")
            .join("agentml-check.yml"),
        "name: AgentML Check\n",
    )
    .unwrap();
}

#[test]
fn doctor_passes_in_agentml_repo() {
    let result = doctor::run();
    assert!(result.is_ok());
}

#[test]
fn doctor_passes_after_generic_init() {
    let temp_dir = env::temp_dir().join("agentml-doctor-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    setup_generic_init(&temp_dir);

    let original = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();
    let result = doctor::run();
    env::set_current_dir(original).unwrap();

    let _ = fs::remove_dir_all(&temp_dir);
    assert!(result.is_ok(), "doctor should pass after generic init");
}

#[test]
fn doctor_warns_for_missing_user_repo_files() {
    let temp_dir = env::temp_dir().join("agentml-doctor-test-missing");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    fs::write(temp_dir.join("AGENT.agent"), "purpose: test\n").unwrap();

    let original = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();
    let result = doctor::run();
    env::set_current_dir(original).unwrap();

    let _ = fs::remove_dir_all(&temp_dir);
    assert!(
        result.is_ok(),
        "doctor should not hard-fail for missing optional files"
    );
}
