pub mod pack;
pub mod validate;

use crate::parser::parse_skill_file;
use crate::types::SkillFile;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

fn discover_skills() -> Vec<(PathBuf, SkillFile)> {
    let mut skills = Vec::new();
    let search_paths = [PathBuf::from("skills"), PathBuf::from(".agentml/skills")];

    for base in &search_paths {
        if !base.exists() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "skill")
                    && let Ok(skill) = parse_skill_file(&path)
                {
                    skills.push((path, skill));
                }
            }
        }
    }
    skills
}

pub fn run_list() -> Result<()> {
    let skills = discover_skills();

    if skills.is_empty() {
        println!("{}", "No skills found.".dimmed());
        return Ok(());
    }

    println!("{}", "AgentML Skills".bold());
    println!();

    for (path, skill) in &skills {
        println!("{}", skill.skill.cyan());
        println!("  path: {}", path.display());
        println!("  {}", skill.description.dimmed());
        println!();
    }

    Ok(())
}

pub fn run_inspect(path_arg: &str) -> Result<()> {
    let path = PathBuf::from(path_arg);
    let skill = if path.exists() {
        parse_skill_file(&path)?
    } else {
        let found_path = find_skill_by_name(path_arg);
        parse_skill_file(&found_path.with_extension("skill"))?
    };

    println!("{}", format!("Skill: {}", skill.skill).bold().cyan());
    println!();
    println!("{} {}", "Version:".bold(), skill.version);
    println!("{} {}", "Description:".bold(), skill.description);

    if let Some(reqs) = &skill.requirements {
        println!("\n{}:", "Requirements".bold());
        for r in reqs {
            println!("  - {}", r);
        }
    }

    if let Some(actions) = &skill.actions {
        println!("\n{}:", "Actions".bold());
        for a in actions {
            println!("  - {}", a);
        }
    }

    if let Some(rules) = &skill.rules {
        println!("\n{}:", "Rules".bold());
        for r in rules {
            println!("  - {}", r);
        }
    }

    if let Some(success) = &skill.success {
        println!("\n{}:", "Success".bold());
        println!("  {}", success);
    }

    if let Some(output) = &skill.output {
        println!("\n{}:", "Output".bold());
        println!("  {}", output);
    }

    if let Some(applies) = &skill.applies_to {
        println!("\n{}:", "Applies To".bold());
        if let Some(paths) = &applies.paths {
            println!("  Paths:");
            for p in paths {
                println!("    - {}", p);
            }
        }
        if let Some(stacks) = &applies.stacks {
            println!("  Stacks:");
            for s in stacks {
                println!("    - {}", s);
            }
        }
        if let Some(keywords) = &applies.keywords {
            println!("  Keywords:");
            for k in keywords {
                println!("    - {}", k);
            }
        }
    }

    if let Some(risk) = &skill.risk {
        println!("\n{}:", "Risk".bold());
        if let Some(score) = risk.base_score {
            println!("  Base score: {}", score);
        }
        if let Some(paths) = &risk.high_risk_paths {
            println!("  High risk paths:");
            for p in paths {
                println!("    - {}", p);
            }
        }
    }

    if let Some(validation) = &skill.requires_validation {
        println!("\n{}:", "Validation".bold());
        for v in validation {
            println!("  - {}", v);
        }
    }

    Ok(())
}

fn find_skill_by_name(name: &str) -> PathBuf {
    let search_paths = [PathBuf::from("skills"), PathBuf::from(".agentml/skills")];

    for base in &search_paths {
        let candidate = base.join(name);
        if candidate.with_extension("skill").exists() {
            return candidate.with_extension("skill");
        }
    }
    PathBuf::from(name)
}

pub fn run_match() -> Result<()> {
    use std::process::Command;

    let skills = discover_skills();
    let mut matched_skills: Vec<(&PathBuf, &SkillFile)> = Vec::new();

    let mut changed_files: Vec<String> = Vec::new();
    if let Ok(output) = Command::new("git").args(["diff", "--name-only"]).output()
        && let Ok(stdout) = String::from_utf8(output.stdout)
    {
        changed_files = stdout.lines().map(|s| s.to_string()).collect();
    }

    let mut project_stack: Option<String> = None;
    if std::path::Path::new("Cargo.toml").exists() {
        project_stack = Some("Rust".to_string());
    } else if std::path::Path::new("package.json").exists() {
        project_stack = Some("Node".to_string());
    } else if std::path::Path::new("pyproject.toml").exists() {
        project_stack = Some("Python".to_string());
    }

    for (path, skill) in &skills {
        let mut is_match = false;

        if let Some(applies) = &skill.applies_to {
            if let Some(stacks) = &applies.stacks
                && let Some(ref stack) = project_stack
                && stacks
                    .iter()
                    .any(|s| s.to_lowercase().contains(&stack.to_lowercase()))
            {
                is_match = true;
            }

            if let Some(paths) = &applies.paths {
                for pattern in paths {
                    for changed in &changed_files {
                        if changed.starts_with(pattern.trim_end_matches("/*"))
                            || changed.contains(pattern.trim_start_matches("*/"))
                        {
                            is_match = true;
                            break;
                        }
                    }
                    if is_match {
                        break;
                    }
                }
            }

            if let Some(keywords) = &applies.keywords {
                for keyword in keywords {
                    for changed in &changed_files {
                        if changed.contains(keyword) {
                            is_match = true;
                            break;
                        }
                    }
                    if is_match {
                        break;
                    }
                }
            }
        }

        if is_match {
            matched_skills.push((path, skill));
        }
    }

    println!("{}", "AgentML Skill Match".bold().cyan());
    println!();

    if matched_skills.is_empty() {
        println!("{}", "No skills matched the current context.".dimmed());
        return Ok(());
    }

    println!("{}:", "Matched skills".bold());
    for (path, skill) in &matched_skills {
        println!();
        println!("  {} ({})", skill.skill.cyan(), path.display());
        println!("    {}", skill.description);
        if let Some(rules) = &skill.rules {
            println!("    {}:", "Rules".bold());
            for r in rules.iter().take(3) {
                println!("      - {}", r);
            }
        }
        if let Some(validation) = &skill.requires_validation {
            println!("    {}:", "Validation".bold());
            for v in validation.iter().take(3) {
                println!("      - {}", v);
            }
        }
    }

    println!();
    println!(
        "{}",
        "Run `agentml skill inspect <name>` for full details.".dimmed()
    );

    Ok(())
}
