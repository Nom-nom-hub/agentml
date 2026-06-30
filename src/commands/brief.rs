use crate::detect::detect_project;
use crate::parser::parse_agent_file;
use crate::types::AgentFile;
use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

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
    pub skills: Vec<BriefSkill>,
}

#[derive(Debug, Default, Serialize)]
pub struct BriefSkill {
    pub name: String,
    pub reason: String,
    pub rules: Vec<String>,
    pub validation: Vec<String>,
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
    let diff_risk = if include_diff {
        run_diff_check()
    } else {
        (0, vec![])
    };

    let matched_skills = match_skills(&agent, &info);

    let output = if format == "json" {
        generate_json_output(&agent, &info, diff_risk, &matched_skills)
    } else {
        generate_md_output(&agent, &info, diff_risk, &matched_skills)
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

fn generate_md_output(
    agent: &AgentFile,
    info: &crate::detect::ProjectInfo,
    diff_risk: (u32, Vec<String>),
    matched_skills: &[BriefSkill],
) -> String {
    let project = agent
        .meta
        .as_ref()
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

    let allowed: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    let forbidden: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(serde_yaml::Value::String("forbidden_paths".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let validation: Vec<String> = agent
        .validation
        .as_ref()
        .map(|v| v.iter().map(|vc| vc.command.clone()).collect())
        .unwrap_or_default();
    let (risk_score, _risk_reasons) = diff_risk;

    let mut output = String::new();
    output.push_str("# AgentML Operating Brief\n\n");
    output.push_str(&format!("Project:\n{}\n\n", project));
    output.push_str(&format!("Stack:\n{}\n\n", stack.join(", ")));

    output.push_str("Allowed write paths:\n");
    if allowed.is_empty() {
        output.push_str("- None configured\n\n");
    } else {
        for p in &allowed {
            output.push_str(&format!("- `{}`\n", p));
        }
        output.push('\n');
    }

    output.push_str("Forbidden paths:\n");
    if forbidden.is_empty() {
        output.push_str(
            "Warning: No forbidden paths are configured. Add forbidden paths to AGENT.agent.\n\n",
        );
    } else {
        for p in &forbidden {
            output.push_str(&format!("- `{}`\n", p));
        }
        output.push('\n');
    }

    output.push_str(&format!(
        "Risk score: {}/100 - {}\n\n",
        risk_score,
        risk_status(risk_score)
    ));

    output.push_str("Required validation:\n");
    for c in &validation {
        output.push_str(&format!("- `{}`\n", c));
    }
    output.push('\n');

    output.push_str("Relevant Skills:\n");
    if matched_skills.is_empty() {
        output.push_str("- None matched\n\n");
    } else {
        for skill in matched_skills {
            output.push_str(&format!("- `{}`\n", skill.name));
            output.push_str(&format!("  Reason: {}\n", skill.reason));
            if !skill.rules.is_empty() {
                output.push_str("  Key rules:\n");
                for r in skill.rules.iter().take(3) {
                    output.push_str(&format!("  - {}\n", r));
                }
            }
        }
        output.push('\n');
    }

    output.push_str("Source of truth:\n");
    output.push_str("1. AGENT.agent\n");
    output.push_str("2. .agentml/brief.md\n");
    output.push_str("3. .agentml/context.md\n");
    output.push_str("4. AGENTS.md\n");
    output.push_str("5. README.md\n\n");

    output.push_str("Final report format:\n");
    output.push_str("Summary:\n");
    output.push_str("Files changed:\n");
    output.push_str("Commands run:\n");
    output.push_str("Skills used:\n");
    output.push_str("Validation result:\n");
    output.push_str("Risk score:\n");
    output.push_str("Commit:\n");
    output.push_str("Git status:\n");
    output.push_str("Risks:\n");
    output.push_str("Next steps:\n\n");

    output
}

fn generate_json_output(
    agent: &AgentFile,
    info: &crate::detect::ProjectInfo,
    diff_risk: (u32, Vec<String>),
    matched_skills: &[BriefSkill],
) -> String {
    let project = agent
        .meta
        .as_ref()
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

    let allowed: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    let forbidden: Vec<String> = if let Some(safety) = &agent.safety {
        if let Some(obj) = safety.as_mapping() {
            obj.get(serde_yaml::Value::String("forbidden_paths".to_string()))
                .and_then(|v| v.as_sequence())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let validation: Vec<String> = agent
        .validation
        .as_ref()
        .map(|v| v.iter().map(|vc| vc.command.clone()).collect())
        .unwrap_or_default();
    let (risk_score, risk_reasons) = diff_risk;

    let output = BriefOutput {
        project,
        stack,
        allowed_write_paths: allowed,
        forbidden_paths: forbidden,
        forbidden_actions: vec![],
        validation_commands: validation,
        risk: RiskInfo {
            score: risk_score,
            status: format!("{}/100 - {}", risk_score, risk_status(risk_score)),
            reasons: risk_reasons,
        },
        rules: vec![
            "Do not modify forbidden files.".to_string(),
            "Add tests for parser, validator, or schema changes.".to_string(),
            "Do not use unwrap in user-facing parsing paths.".to_string(),
            "Do not report completion without running validation.".to_string(),
            "Final report must include changed files, commands run, risks, and next steps."
                .to_string(),
        ],
        final_report_required_fields: vec![
            "summary".to_string(),
            "files_changed".to_string(),
            "commands_run".to_string(),
            "validation_result".to_string(),
            "risk_score".to_string(),
            "next_steps".to_string(),
        ],
        skills: matched_skills
            .iter()
            .map(|s| BriefSkill {
                name: s.name.clone(),
                reason: s.reason.clone(),
                rules: s.rules.clone(),
                validation: s.validation.clone(),
            })
            .collect(),
    };

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

fn run_diff_check() -> (u32, Vec<String>) {
    use std::process::Command;
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .output();
    if let Ok(out) = output
        && out.status.success()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let mut score = 0u32;
        let mut reasons = Vec::new();
        for line in stdout.lines() {
            let path = line.trim();
            let is_agent = path == "AGENT.agent";
            let is_skill = path.starts_with("skills/") && path.ends_with(".skill");
            let is_src = path.starts_with("src/") && path.ends_with(".rs");

            if is_agent {
                score += 30;
                reasons.push(path.to_string());
            } else if is_skill || is_src {
                score += 20;
                reasons.push(path.to_string());
            }
        }
        return (score.min(100), reasons);
    }
    (0, vec![])
}

fn match_skills(agent: &AgentFile, info: &crate::detect::ProjectInfo) -> Vec<BriefSkill> {
    use crate::parser::parse_skill_file;
    use std::fs;

    let mut matched = Vec::new();
    let mut skills = Vec::new();
    let search_paths = [PathBuf::from("skills"), PathBuf::from(".agentml/skills")];

    for base in &search_paths {
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

    let project_stack: Option<String> = match info.project_type.as_str() {
        "Rust" => Some("Rust".to_string()),
        "Next.js" => Some("TypeScript".to_string()),
        "Node" => Some("Node".to_string()),
        "Vite" => Some("TypeScript".to_string()),
        "Python" => Some("Python".to_string()),
        _ => None,
    };

    for (_path, skill) in &skills {
        let mut is_match = false;
        let mut reason = String::new();

        if let Some(applies) = &skill.applies_to {
            if let Some(stacks) = &applies.stacks
                && let Some(ref stack) = project_stack
                && stacks
                    .iter()
                    .any(|s| s.to_lowercase().contains(&stack.to_lowercase()))
            {
                is_match = true;
                reason = format!("Project stack: {}", stack);
            }

            if let Some(paths) = &applies.paths {
                for pattern in paths {
                    for allowed in &agent
                        .permissions
                        .as_ref()
                        .and_then(|p| p.write.clone())
                        .unwrap_or_default()
                    {
                        if allowed.contains(pattern.trim_end_matches("/*"))
                            || pattern.contains("**")
                        {
                            is_match = true;
                            reason = format!("Path pattern: {}", pattern);
                            break;
                        }
                    }
                    if is_match {
                        break;
                    }
                }
            }

            if let Some(keywords) = &applies.keywords
                && keywords
                    .iter()
                    .any(|k| k == "rust" || k == "cli" || k == "skill")
            {
                is_match = true;
                reason = "Keyword match".to_string();
            }
        }

        if is_match {
            matched.push(BriefSkill {
                name: skill.skill.clone(),
                reason: if reason.is_empty() {
                    skill.description.clone()
                } else {
                    reason
                },
                rules: skill.rules.clone().unwrap_or_default(),
                validation: skill.requires_validation.clone().unwrap_or_default(),
            });
        }
    }

    matched
}

fn risk_status(score: u32) -> &'static str {
    if score >= 100 {
        "blocked"
    } else if score >= 80 {
        "critical"
    } else if score >= 50 {
        "high"
    } else if score >= 20 {
        "medium"
    } else {
        "low"
    }
}
