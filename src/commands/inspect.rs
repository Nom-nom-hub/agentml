use crate::detect::{detect_project, print_inspect};
use colored::Colorize;

pub fn run() -> anyhow::Result<()> {
    let info = detect_project()?;
    print_inspect(&info);
    println!();
    println!("{}", "Recommended generated files:".bold());
    println!("  {}", "AGENT.agent".cyan());
    println!("  {}", "AGENTS.md".cyan());
    println!("  {}", ".agentml/context.md".cyan());
    println!("  {}", ".agentml/brief.md".cyan());
    Ok(())
}
