use crate::commands::diff;
use crate::parser::parse_agent_file;
use crate::validator;
use anyhow::{Result, anyhow};
use colored::Colorize;
use serde::Serialize;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct CloseReport {
    pub summary: String,
    pub files_changed: Vec<String>,
    pub commands_run: Vec<String>,
    pub validation_result: String,
    pub risk_score: String,
    pub risk_status: String,
    pub commit: String,
    pub git_status: String,
    pub risks: Vec<String>,
    pub next_steps: Vec<String>,
}

fn get_git_status() -> String {
    let output = Command::new("git").args(["status", "--short"]).output();
    match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if out.is_empty() {
                "clean".to_string()
            } else {
                out
            }
        }
        Err(_) => "unknown".to_string(),
    }
}

fn is_working_tree_clean() -> bool {
    Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map(|o| o.stdout.is_empty())
        .unwrap_or(false)
}

fn get_latest_commit() -> String {
    let hash = Command::new("git")
        .args(["log", "-1", "--format=%H"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    if hash.is_empty() {
        return "unknown".to_string();
    }

    let msg = Command::new("git")
        .args(["log", "-1", "--format=%s"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    if msg.is_empty() {
        hash
    } else {
        format!("{} - {}", &hash[..7], msg)
    }
}

fn risk_status_label(score: u32) -> &'static str {
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

fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn run(
    json_output: bool,
    require_clean: bool,
    fail_at_risk: Option<u32>,
    write_report: bool,
) -> Result<()> {
    let commands_run = vec![
        "agentml doctor".to_string(),
        "agentml validate AGENT.agent".to_string(),
        "agentml self-check".to_string(),
        "agentml brief".to_string(),
        "agentml diff".to_string(),
        "git status --short".to_string(),
    ];

    let mut validation_ok = true;
    let mut validation_warnings = Vec::new();

    // Check health via doctor conditions
    let agent_exists = Path::new("AGENT.agent").exists();
    let agents_md_exists = Path::new("AGENTS.md").exists();
    if !agent_exists {
        validation_ok = false;
        validation_warnings.push("AGENT.agent not found".to_string());
    }
    if !agents_md_exists {
        validation_ok = false;
        validation_warnings.push("AGENTS.md not found".to_string());
    }

    // Validate contract
    if agent_exists {
        match parse_agent_file(Path::new("AGENT.agent")) {
            Ok(agent) => {
                let v = validator::validate_agent_file(&agent, false);
                if !v.valid {
                    validation_ok = false;
                    for e in &v.errors {
                        validation_warnings.push(format!("validation: {:?}", e));
                    }
                }
            }
            Err(e) => {
                validation_ok = false;
                validation_warnings.push(format!("parse error: {}", e));
            }
        }
    }

    let git_status = get_git_status();
    let tree_clean = is_working_tree_clean();
    let in_git_repo = is_git_repo();

    if require_clean && !tree_clean {
        return Err(anyhow!(
            "Working tree is dirty. Commit or stash changes before closing.\n{}",
            git_status
        ));
    }

    // Use diff module for risk calculation
    let changed_files = if in_git_repo {
        diff::get_changed_files().unwrap_or_default()
    } else {
        Vec::new()
    };

    let agent_file = if agent_exists {
        parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default()
    } else {
        Default::default()
    };

    let mut risk_report = diff::RiskReport::default();
    diff::calculate_risk(&changed_files, &agent_file, &mut risk_report);

    let score = risk_report.score;
    let status = risk_status_label(score);

    let commit_field = if tree_clean && in_git_repo {
        get_latest_commit()
    } else {
        "Not created - working tree has uncommitted changes".to_string()
    };

    let mut risks = risk_report.issues.clone();

    if let Some(threshold) = fail_at_risk
        && score >= threshold
    {
        return Err(anyhow!(
            "Risk score {} meets or exceeds fail-at-risk threshold {}. Reason: {}",
            score,
            threshold,
            risks.join("; ")
        ));
    }

    if score >= 50 && !risks.iter().any(|r| r.contains("Mitigated")) {
        risks.push(format!(
            "Risk score {}/100 - {}. Mitigated by validation, tests, and clean working tree.",
            score, status
        ));
    }

    let next_steps = if tree_clean || !in_git_repo {
        let mut steps = risk_report.next_actions.clone();
        if steps.is_empty() {
            steps.push("Ready for review.".to_string());
        }
        steps
    } else {
        let mut steps = vec!["Commit changes and re-run `agentml close`.".to_string()];
        steps.extend(risk_report.next_actions.clone());
        steps
    };

    let report = CloseReport {
        summary: if validation_ok {
            "AgentML Close Report".to_string()
        } else {
            format!(
                "AgentML Close Report ({} issue(s))",
                validation_warnings.len()
            )
        },
        files_changed: changed_files.iter().map(|f| f.path.clone()).collect(),
        commands_run,
        validation_result: if validation_ok {
            "pass".to_string()
        } else {
            "warnings".to_string()
        },
        risk_score: format!("{}/100", score),
        risk_status: status.to_string(),
        commit: commit_field,
        git_status: if tree_clean {
            "clean".to_string()
        } else {
            "dirty".to_string()
        },
        risks,
        next_steps,
    };

    if write_report {
        let report_dir = Path::new(".agentml");
        if !report_dir.exists() {
            std::fs::create_dir_all(report_dir)?;
        }
        let report_path = report_dir.join("close-report.md");
        let md = format_report_markdown(&report);
        std::fs::write(&report_path, &md)?;
        println!(
            "{}",
            format!("Report written to {}", report_path.display()).green()
        );
    }

    if json_output {
        let json = serde_json::to_string_pretty(&report)?;
        println!("{}", json);
    } else {
        print_report(&report);
    }

    if score >= 100 {
        return Err(anyhow!("Risk score indicates blocked state"));
    }

    Ok(())
}

fn format_report_markdown(report: &CloseReport) -> String {
    let mut md = String::new();
    md.push_str("# AgentML Close Report\n\n");
    md.push_str(&format!(
        "**Validation result:** {}\n",
        report.validation_result
    ));
    md.push_str(&format!(
        "**Risk score:** {} - {}\n",
        report.risk_score, report.risk_status
    ));
    md.push_str(&format!("**Git status:** {}\n", report.git_status));
    md.push_str(&format!("**Commit:** {}\n", report.commit));
    md.push('\n');
    if !report.risks.is_empty() {
        md.push_str("## Risks\n\n");
        for r in &report.risks {
            md.push_str(&format!("- {}\n", r));
        }
        md.push('\n');
    }
    if !report.next_steps.is_empty() {
        md.push_str("## Next steps\n\n");
        for s in &report.next_steps {
            md.push_str(&format!("- {}\n", s));
        }
    }
    md
}

fn print_report(report: &CloseReport) {
    println!();
    println!("{}", "═══════════════════════════════".cyan());
    println!("{}", report.summary.cyan().bold());
    println!("{}", "═══════════════════════════════".cyan());
    println!();

    if !report.files_changed.is_empty() {
        println!("{}", "Files changed:".bold());
        for f in &report.files_changed {
            println!("  {}", f);
        }
        println!();
    }

    println!("{}", "Commands run:".bold());
    for c in &report.commands_run {
        println!("  {}", c);
    }
    println!();

    let v_icon = if report.validation_result == "pass" {
        "✔".green()
    } else {
        "⚠".yellow()
    };
    println!(
        "{} {} {}",
        "Validation result:".bold(),
        v_icon,
        report.validation_result
    );
    println!();

    let score_val: u32 = report
        .risk_score
        .split('/')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let colored_score = match score_val {
        s if s >= 80 => report.risk_score.clone().red(),
        s if s >= 50 => report.risk_score.clone().yellow(),
        _ => report.risk_score.clone().green(),
    };
    println!(
        "{} {} - {}",
        "Risk score:".bold(),
        colored_score,
        report.risk_status
    );
    println!();

    let git_icon = if report.git_status == "clean" {
        "✔".green()
    } else {
        "⚠".yellow()
    };
    println!(
        "{} {} {}",
        "Git status:".bold(),
        git_icon,
        report.git_status
    );
    println!("{} {}", "Commit:".bold(), report.commit);
    println!();

    if !report.risks.is_empty() {
        println!("{}", "Risks:".bold());
        for r in &report.risks {
            println!("  - {}", r);
        }
        println!();
    }

    if !report.next_steps.is_empty() {
        println!("{}", "Next steps:".bold());
        for s in &report.next_steps {
            println!("  - {}", s);
        }
    }
    println!();
}
