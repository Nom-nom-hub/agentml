use crate::parser;
use crate::types::AgentFile;
use crate::validator;
use crate::validator::{get_forbidden_actions, get_forbidden_paths, has_secrets_policy};
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Self-Check ══".cyan().bold());
    println!();

    let agent_path = Path::new("AGENT.agent");
    if !agent_path.exists() {
        println!("{}", "✘ AGENT.agent not found".red());
        std::process::exit(1);
    }

    println!("{} {}", "Contract:".bold(), "AGENT.agent".cyan());
    let agent: AgentFile = parser::parse_agent_file(agent_path)?;
    let report = validator::validate_agent_file(&agent, false);

    if report.valid {
        println!("{}", "Status: valid".green().bold());
    } else {
        println!("{}", "Status: invalid".red().bold());
    }
    println!();

    println!("{}", "Skills:".bold());
    let skills_dir = Path::new("skills");
    if skills_dir.exists() {
        let entries = fs::read_dir(skills_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "skill").unwrap_or(false) {
                let name = path.file_name().unwrap().to_str().unwrap();
                match parser::parse_skill_file(&path) {
                    Ok(skill) => {
                        let skill_report = validator::validate_skill_file(&skill);
                        let status = if skill_report.valid {
                            format!("{}", "valid".green())
                        } else {
                            format!("{}", "invalid".red())
                        };
                        println!("  {}: {}", name, status);
                    }
                    Err(e) => {
                        println!("  {}: {} ({})", name, "invalid".red(), e);
                    }
                }
            }
        }
        println!();
    }

    println!("{}", "Safety:".bold());
    let forbidden_paths = get_forbidden_paths(&agent.safety);
    let forbidden_actions = get_forbidden_actions(&agent.safety);

    if let Some(forbidden) = &forbidden_paths {
        let empty_writes: Vec<String> = vec![];
        let writes = agent
            .permissions
            .as_ref()
            .and_then(|p| p.write.as_ref())
            .unwrap_or(&empty_writes);
        let mut safety_pass = true;
        for f in forbidden {
            if writes.iter().any(|w| f.contains(w) || w.contains(f)) {
                println!(
                    "  {} forbidden_paths overlap with write permissions: {}",
                    "✘".red(),
                    f
                );
                safety_pass = false;
            }
        }
        if safety_pass {
            println!("  {} forbidden_paths", "✔".green());
        }
    } else if agent.safety.is_some() {
        println!("  {} forbidden_paths", "✘".red());
    }

    if let Some(actions) = &forbidden_actions {
        if !actions.is_empty() {
            println!("  {} destructive_actions_policy", "✔".green());
        }
    } else if agent.safety.is_some() {
        println!("  {} destructive_actions_policy", "✘".red());
    }

    if has_secrets_policy(&agent.safety) {
        println!("  {} secrets_policy", "✔".green());
    } else if agent.safety.is_some() {
        println!("  {} secrets_policy: weak or missing", "✘".red());
    }
    println!();

    println!("{}", "Validation:".bold());
    if let Some(validation) = &agent.validation {
        for v in validation {
            println!("  {}: {}", v.name, v.command.dimmed());
        }
    } else {
        println!("  {} No validation commands defined", "✘".red());
    }
    println!();

    println!(
        "{}",
        format!("Risk Score: {}/100", report.risk_score)
            .magenta()
            .bold()
    );

    let dogfood_pass = report.valid;
    println!();
    println!("{}", "Result:".bold());
    if dogfood_pass {
        println!("  Dogfood status: {}", "PASS".green().bold());
    } else {
        println!("  Dogfood status: {}", "FAIL".red().bold());
    }
    println!("  Risk score: {}/100", report.risk_score);

    if !report.errors.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
