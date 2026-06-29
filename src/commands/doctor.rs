use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Doctor ══".cyan().bold());
    println!();

    let checks = vec![
        ("AGENT.agent", Path::new("AGENT.agent")),
        ("docs/spec.md", Path::new("docs/spec.md")),
        ("README.md", Path::new("README.md")),
        ("Cargo.toml", Path::new("Cargo.toml")),
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

    Ok(())
}
