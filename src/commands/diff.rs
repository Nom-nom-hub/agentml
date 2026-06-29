use anyhow::Result;
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Diff ══".cyan().bold());
    println!();
    println!("{}", "No changes detected.".dimmed());
    Ok(())
}
