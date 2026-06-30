use std::path::Path;

fn project_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn every_example_agent_file_exists() {
    let examples = [
        "examples/rust-cli/AGENT.agent",
        "examples/nextjs-app/AGENT.agent",
        "examples/node-package/AGENT.agent",
        "examples/python-package/AGENT.agent",
        "examples/dangerous-change-demo/AGENT.agent",
    ];
    for path in &examples {
        assert!(project_root().join(path).exists(), "{} must exist", path);
    }
}

#[test]
fn every_example_agents_md_exists() {
    let examples = [
        "examples/rust-cli/AGENTS.md",
        "examples/nextjs-app/AGENTS.md",
        "examples/node-package/AGENTS.md",
        "examples/python-package/AGENTS.md",
        "examples/dangerous-change-demo/AGENTS.md",
    ];
    for path in &examples {
        assert!(project_root().join(path).exists(), "{} must exist", path);
    }
}

#[test]
fn every_example_readme_exists() {
    let examples = [
        "examples/rust-cli/README.md",
        "examples/nextjs-app/README.md",
        "examples/node-package/README.md",
        "examples/python-package/README.md",
        "examples/dangerous-change-demo/README.md",
    ];
    for path in &examples {
        assert!(project_root().join(path).exists(), "{} must exist", path);
    }
}

#[test]
fn dangerous_change_demo_has_before_and_after() {
    assert!(
        project_root()
            .join("examples/dangerous-change-demo/before.md")
            .exists()
    );
    assert!(
        project_root()
            .join("examples/dangerous-change-demo/after.md")
            .exists()
    );
}

#[test]
fn docs_examples_md_exists() {
    assert!(project_root().join("docs/examples.md").exists());
}

#[test]
fn each_example_agent_validates() {
    use agentml::parser::parse_agent_file;
    use agentml::validator::validate_agent_file;

    let examples = [
        "examples/rust-cli/AGENT.agent",
        "examples/nextjs-app/AGENT.agent",
        "examples/node-package/AGENT.agent",
        "examples/python-package/AGENT.agent",
        "examples/dangerous-change-demo/AGENT.agent",
    ];
    for rel_path in &examples {
        let path = project_root().join(rel_path);
        let agent = parse_agent_file(&path)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", rel_path, e));
        let report = validate_agent_file(&agent, false);
        assert!(
            report.valid,
            "{} failed validation: {:?}",
            rel_path, report.errors
        );
    }
}

#[test]
fn readme_links_examples() {
    let readme =
        std::fs::read_to_string(project_root().join("README.md")).expect("README.md must exist");
    assert!(
        readme.contains("examples/rust-cli/"),
        "README.md should link to rust-cli example"
    );
    assert!(
        readme.contains("examples/nextjs-app/"),
        "README.md should link to nextjs-app example"
    );
    assert!(
        readme.contains("examples/node-package/"),
        "README.md should link to node-package example"
    );
    assert!(
        readme.contains("examples/python-package/"),
        "README.md should link to python-package example"
    );
    assert!(
        readme.contains("examples/dangerous-change-demo/"),
        "README.md should link to dangerous-change-demo example"
    );
}
