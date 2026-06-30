use crate::detect::detect_project;
use crate::parser::parse_agent_file;
use crate::types::AgentFile;
use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize)]
pub struct BriefOutput {
    pub project: String,
    pub stack: Vec<String>,
    pub allowed_write_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub forbidden_actions: Vec<String>,
    pub validation_commands: Vec<String>,
    pub risk: RiskInfo,
    pub rules: Vec<String>,
    pub final_report_required_fields: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct RiskInfo {
    pub score: u32,
    pub status: String,
    pub reasons: Vec<String>,
}

pub fn run(format: &str, write: bool, _max_lines: usize, include_diff: bool) -> Result<()> {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_else(|_| AgentFile::default());
    let info = detect_project()?;
    let diff_risk = if include_diff { run_diff_check() } else { (0, vec![]) };

    let output = if format == "json" {
        generate_json_output(&agent, &info, diff_risk)
    } else {
        generate_md_output(&agent, &info, diff_risk)
    };

    if write {
        let brief_path = Path::new(".agentml").join("brief.md");
        fs::create_dir_all(brief_path.parent().unwrap())?;
        fs::write(&brief_path, &output)?;
        println!("{}", "Written to .agentml/brief.md".green());
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn generate_md_output(agent: &AgentFile, info: &crate::detect::ProjectInfo, diff_risk: (u32, Vec<String>)) -> String {
    let project = agent.meta.as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "unknown-project".to_string());

    let stack: Vec<String> = match info.project_type.as_str() {
        "Rust" => vec!["Rust".to_string()],
        "Next.js" => vec!["Next.js".to_string(), "TypeScript".to_string()],
        "Node" => vec!["Node.js".to_string()],
        "Vite" => vec!["Vite".to_string(), "TypeScript".to_string()],
        "Python" => vec!["Python".to_string()],
        _ => vec!["Generic".to_string()],
    };

    let allowed: Vec<String> = agent.permissions.as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    let forbidden: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(&serde_yaml::Value::String("forbidden_paths".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else { vec![] }
    } else { vec![] };

    let forbidden_actions: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(&serde_yaml::Value::String("forbidden_actions".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else { vec![] }
    } else { vec![] };

    let validation: Vec<String> = agent.validation.as_ref()
        .map(|v| v.iter().map(|vc| vc.command.clone()).collect())
        .unwrap_or_default();
    let (risk_score, risk_reasons) = diff_risk;

    let mut output = String::new();
    output.push_str(&format!("# AgentML Operating Brief\n\n"));
    output.push_str(&format!("Project:\n{}\n\n", project));
    output.push_str(&format!("Stack:\n{}\n\n", stack.join(", ")));
    output.push_str(&format!("Allowed write paths:\n{}\n\n", allowed.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n")));
    output.push_str(&format!("Forbidden:\n{}\n\n", forbidden.iter().map(|p| format!("- {}", p)).collect::<Vec<_>>().join("\n")));
    output.push_str(&format!("Current risk:\n{} — {}\n\n", risk_status(risk_score), risk_description(risk_score)));
    output.push_str(&format!("Required validation:\n{}\n\n", validation.iter().map(|c| format!("- {}", c)).collect::<Vec<_>>().join("\n")));
    output.push_str(&format!("Rules:\n1. Do not modify forbidden files.\n2. Add tests for parser, validator, or schema changes.\n3. Do not use unwrap in user-facing parsing paths.\n4. Do not report completion without running validation.\n5. Final report must include changed files, commands run, risks, and next steps.\n\n"));
    output.push_str(&format!("Final report format:\nSummary:\nFiles changed:\nCommands run:\nValidation result:\nRisk score:\nNext steps:\n"));

    output
}

fn generate_json_output(agent: &AgentFile, info: &crate::detect::ProjectInfo, diff_risk: (u32, Vec<String>)) -> String {
    let project = agent.meta.as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "unknown-project".to_string());

    let stack: Vec<String> = match info.project_type.as_str() {
        "Rust" => vec!["Rust".to_string()],
        "Next.js" => vec!["Next.js".to_string(), "TypeScript".to_string()],
        "Node" => vec!["Node.js".to_string()],
        "Vite" => vec!["Vite".to_string(), "TypeScript".to_string()],
        "Python" => vec!["Python".to_string()],
        _ => vec!["Generic".to_string()],
    };

    let allowed: Vec<String> = agent.permissions.as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    let forbidden: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(&serde_yaml::Value::String("forbidden_paths".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else { vec![] }
    } else { vec![] };

    let forbidden_actions: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(&serde_yaml::Value::String("forbidden_actions".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else { vec![] }
    } else { vec![] };

    let validation: Vec<String> = agent.validation.as_ref()
        .map(|v| v.iter().map(|vc| vc.command.clone()).collect())
        .unwrap_or_default();
    let (risk_score, risk_reasons) = diff_risk;

    let output = BriefOutput {
        project,
        stack,
        allowed_write_paths: allowed,
        forbidden_paths: forbidden,
        forbidden_actions,
        validation_commands: validation,
        risk: RiskInfo {
            score: risk_score,
            status: risk_status(risk_score).to_string(),
            reasons: risk_reasons,
        },
        rules: vec![
            "Do not modify forbidden files.".to_string(),
            "Add tests for parser, validator, or schema changes.".to_string(),
            "Do not use unwrap in user-facing parsing paths.".to_string(),
            "Do not report completion without running validation.".to_string(),
            "Final report must include changed files, commands run, risks, and next steps.".to_string(),
        ],
        final_report_required_fields: vec![
            "summary".to_string(),
            "files_changed".to_string(),
            "commands_run".to_string(),
            "validation_result".to_string(),
            "risk_score".to_string(),
            "next_steps".to_string(),
        ],
    };

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

fn run_diff_check() -> (u32, Vec<String>) {
    use std::process::Command;
    let output = Command::new("git").args(["diff", "--name-only", "HEAD"]).output();
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut score = 0u32;
            let mut reasons = Vec::new();
            for line in stdout.lines() {
                let path = line.trim();
                if path.eq_ignore_ascii_case("AGENT.agent") {
                    score += 30;
                    reasons.push("AGENT.agent changed".to_string());
                }
                if path.starts_with("skills/") && path.ends_with(".skill") {
                    score += 20;
                    reasons.push("skill changed".to_string());
                }
                if path.starts_with("src/") && path.ends_with(".rs") {
                    score += 20;
                    reasons.push("source changed without tests".to_string());
                }
            }
            return (score.min(100), reasons);
        }
    }
    (0, vec![])
}

fn risk_status(score: u32) -> &'static str {
    if score >= 100 { "blocked" }
    else if score >= 80 { "critical" }
    else if score >= 50 { "high" }
    else if score >= 20 { "medium" }
    else { "low" }
}

fn risk_description(_score: u32) -> &'static str {
    "Review the risk factors before proceeding."
}