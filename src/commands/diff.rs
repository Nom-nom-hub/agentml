use crate::parser::parse_agent_file;
use crate::types::AgentFile;
use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use glob::glob;
use serde_yaml::Value;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: String,
}

#[derive(Debug, Default)]
pub struct RiskReport {
    pub score: u32,
    pub issues: Vec<String>,
    pub required_validations: Vec<String>,
    pub next_actions: Vec<String>,
}

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Diff Audit ══".cyan().bold());
    println!();

    let agent_file = parse_agent_file(std::path::Path::new("AGENT.agent")).unwrap_or_else(|_| {
        AgentFile::default()
    });

    let changed_files = get_changed_files()?;
    if changed_files.is_empty() {
        println!("{}", "No changes detected.".dimmed());
        return Ok(());
    }

    let mut report = RiskReport::default();

    println!("{}", "Changed files:".bold());
    for f in &changed_files {
        println!("  {}", f.path);
    }
    println!();

    let permission_check = check_permissions(&changed_files, &agent_file);
    println!("{}", "Permission check:".bold());
    for (path, status) in &permission_check {
        let colored_status = match status.as_str() {
            "allowed" => "allowed".green(),
            "forbidden" => "forbidden".red(),
            _ => "denied".yellow(),
        };
        println!("  {}: {}", path, colored_status);
    }
    println!();

    calculate_risk(&changed_files, &agent_file, &mut report);
    check_required_validations(&agent_file, &mut report);

    println!("{}", "Risk:".bold());
    for issue in &report.issues {
        println!("  {}", issue);
    }
    println!();

    println!("{}", "Required validation:".bold());
    for cmd in &report.required_validations {
        println!("  {}", cmd);
    }
    println!();

    println!("{}", "Result:".bold());
    let score_str = report.score.to_string();
    let colored_score = match report.score {
        s if s >= 80 => score_str.clone().red(),
        s if s >= 50 => score_str.clone().yellow(),
        _ => score_str.green(),
    };
    println!("  Risk score: {}/100", colored_score);

    let status_text = if report.score >= 100 {
        "blocked".to_string().red()
    } else if report.score >= 80 {
        "critical".to_string().red()
    } else if report.score >= 50 {
        "high".to_string().yellow()
    } else if report.score >= 20 {
        "medium".to_string().yellow()
    } else {
        "low".to_string().green()
    };
    println!("  Status: {}", status_text);
    println!();

    if !report.next_actions.is_empty() {
        println!("{}", "Required next actions:".bold());
        for action in &report.next_actions {
            println!("  - {}", action);
        }
    }

    if report.score >= 100 {
        return Err(anyhow!("Risk score indicates blocked state - forbidden file modified"));
    }

    Ok(())
}

pub fn get_changed_files() -> Result<Vec<ChangedFile>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .output()
        .context("Failed to run git diff")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for line in stdout.lines() {
        let path = line.trim();
        if !path.is_empty() {
            files.push(ChangedFile {
                path: path.to_string(),
            });
        }
    }

    Ok(files)
}

pub fn check_permissions(files: &[ChangedFile], agent: &AgentFile) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let write_patterns: Vec<String> = agent.permissions.as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();
    let forbidden_patterns: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            if let Some(paths) = obj.get(&Value::String("forbidden_paths".to_string())) {
                if let Some(arr) = paths.as_sequence() {
                    arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    for file in files {
        let path = &file.path;
        let mut status = "allowed".to_string();

        for pattern in &forbidden_patterns {
            if let Ok(mut paths) = glob(pattern) {
                if paths.any(|p| p.map(|pp| pp.to_string_lossy() == *path).unwrap_or(false)) {
                    status = "forbidden".to_string();
                    break;
                }
            }
        }

        if status != "forbidden" && !write_patterns.is_empty() {
            let mut allowed = false;
            for pattern in &write_patterns {
                if let Ok(mut paths) = glob(pattern) {
                    if paths.any(|p| p.map(|pp| pp.to_string_lossy() == *path).unwrap_or(false)) {
                        allowed = true;
                        break;
                    }
                }
            }
            if !allowed {
                status = "denied".to_string();
            }
        }

        results.push((path.clone(), status));
    }

    results
}

pub fn calculate_risk(files: &[ChangedFile], _agent: &AgentFile, report: &mut RiskReport) {
    for file in files {
        if file.path.eq_ignore_ascii_case("AGENT.agent") {
            report.score += 30;
            report.issues.push(format!("{}: AGENT.agent changed: +30", file.path));
            report.next_actions.push("Review AGENT.agent changes carefully".to_string());
        }
        if file.path.starts_with("skills/") && file.path.ends_with(".skill") {
            report.score += 20;
            report.issues.push(format!("{}: skill changed: +20", file.path));
        }
        if file.path.starts_with("src/") && file.path.ends_with(".rs") {
            let test_path = file.path.replace("src/", "tests/").replace(".rs", "_test.rs");
            if !std::path::Path::new(&test_path).exists() {
                report.score += 20;
                report.issues.push(format!("{}: source changed without tests: +20", file.path));
                report.next_actions.push(format!("Add or update tests for {}", file.path));
            }
        }
        if file.path.eq_ignore_ascii_case("README.md") {
            report.score += 5;
            report.issues.push(format!("{}: README changed: +5", file.path));
        }
        if file.path.starts_with("docs/") || file.path.ends_with(".md") {
            report.score += 10;
            report.issues.push(format!("{}: docs changed: +10", file.path));
        }
    }

    if report.score > 100 {
        report.score = 100;
    }
}

fn check_required_validations(agent: &AgentFile, report: &mut RiskReport) {
    if let Some(validations) = &agent.validation {
        for v in validations {
            report.required_validations.push(v.command.clone());
        }
    } else if report.score > 0 {
        report.required_validations = vec![
            "cargo fmt --check".to_string(),
            "cargo clippy --all-targets -- -D warnings".to_string(),
            "cargo test".to_string(),
        ];
    }
}