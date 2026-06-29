use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Doctor ══".cyan().bold());
    println!();

    let is_agentml_repo = Path::new("src/main.rs").exists() && Path::new("Cargo.toml").exists();

    if is_agentml_repo {
        check_agentml_repo();
    } else {
        check_user_repo()?;
    }

    Ok(())
}

fn check_agentml_repo() {
    let checks = vec![
        ("AGENT.agent", Path::new("AGENT.agent")),
        ("skills/", Path::new("skills")),
        ("docs/spec.md", Path::new("docs/spec.md")),
        ("README.md", Path::new("README.md")),
        ("Cargo.toml", Path::new("Cargo.toml")),
        (
            ".github/workflows/agentml-self-check.yml",
            Path::new(".github/workflows/agentml-self-check.yml"),
        ),
    ];

    let mut all_ok = true;
    for (name, path) in checks {
        if path.exists() {
            println!("  {} {}", "✔".green(), name.cyan());
        } else {
            println!("  {} {}", "✘".red(), name.cyan());
            all_ok = false;
        }
    }

    println!();
    if all_ok {
        println!("{}", "All checks passed.".green().bold());
    } else {
        println!("{}", "Some checks failed.".red().bold());
    }
}

fn check_user_repo() -> Result<()> {
    let mut all_ok = true;

    // Required AgentML files
    let required = vec![
        ("AGENT.agent", Path::new("AGENT.agent")),
        ("skills/", Path::new("skills")),
        (".agentml/", Path::new(".agentml")),
        ("docs/agentml.md", Path::new("docs/agentml.md")),
        (
            ".github/workflows/agentml-check.yml",
            Path::new(".github/workflows/agentml-check.yml"),
        ),
    ];

    for (name, path) in required {
        if path.exists() {
            println!("  {} {}", "✔".green(), name.cyan());
        } else {
            println!(
                "  {} {} {}",
                "⚠".yellow(),
                name.cyan(),
                "(missing)".dimmed()
            );
            all_ok = false;
        }
    }

    // Optional project files (warnings only)
    let optional = vec![
        ("README.md", Path::new("README.md"), "project documentation"),
        (
            "Cargo.toml",
            Path::new("Cargo.toml"),
            "Rust project metadata",
        ),
        (
            "pyproject.toml",
            Path::new("pyproject.toml"),
            "Python project metadata",
        ),
        (
            "package.json",
            Path::new("package.json"),
            "Node project metadata",
        ),
    ];

    for (name, path, desc) in optional {
        if !path.exists() {
            println!(
                "  {} {} {} {}",
                "ℹ".blue(),
                name.cyan(),
                format!("({})", desc).dimmed(),
                "(optional)".dimmed()
            );
        }
    }

    println!();
    if all_ok {
        println!("{}", "AgentML structure looks good.".green().bold());
    } else {
        println!("{}", "Some AgentML files are missing.".yellow().bold());
        println!(
            "{}",
            "Run `agentml init --template generic` to create the standard structure.".dimmed()
        );
    }

    Ok(())
}
