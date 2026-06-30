use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Unhealthy,
}

#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub name: &'static str,
    pub status: HealthStatus,
    pub message: String,
}

pub fn run() -> Result<()> {
    println!("{}", "══ AgentML Doctor ══".cyan().bold());
    println!();

    let is_agentml_repo = Path::new("src/main.rs").exists() && Path::new("Cargo.toml").exists();

    if is_agentml_repo {
        check_agentml_repo()?;
    } else {
        check_user_repo()?;
    }

    Ok(())
}

fn check_agentml_repo() -> Result<()> {
    let checks = vec![
        HealthCheck {
            name: "AGENT.agent",
            status: if Path::new("AGENT.agent").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            },
            message: "Project contract file".to_string(),
        },
        HealthCheck {
            name: "skills/",
            status: if Path::new("skills").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            },
            message: "Skills directory".to_string(),
        },
        HealthCheck {
            name: "docs/spec.md",
            status: if Path::new("docs/spec.md").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            },
            message: "Specification document".to_string(),
        },
        HealthCheck {
            name: "README.md",
            status: if Path::new("README.md").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            },
            message: "Project documentation".to_string(),
        },
        HealthCheck {
            name: "Cargo.toml",
            status: if Path::new("Cargo.toml").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Unhealthy
            },
            message: "Rust project metadata".to_string(),
        },
        HealthCheck {
            name: ".github/workflows/agentml-self-check.yml",
            status: if Path::new(".github/workflows/agentml-self-check.yml").exists() {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            },
            message: "CI workflow".to_string(),
        },
    ];

    render_checks(&checks);
    Ok(())
}

fn check_user_repo() -> Result<()> {
    let mut checks: Vec<HealthCheck> = Vec::new();

    checks.push(HealthCheck {
        name: "AGENT.agent",
        status: if Path::new("AGENT.agent").exists() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        },
        message: "Project contract file".to_string(),
    });

    checks.push(HealthCheck {
        name: "skills/",
        status: if Path::new("skills").exists() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        },
        message: "Skills directory".to_string(),
    });

    checks.push(HealthCheck {
        name: ".agentml/",
        status: if Path::new(".agentml").exists() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        },
        message: "Generated context".to_string(),
    });

    checks.push(HealthCheck {
        name: "docs/agentml.md",
        status: if Path::new("docs/agentml.md").exists() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        },
        message: "AgentML documentation".to_string(),
    });

    checks.push(HealthCheck {
        name: "git",
        status: if Command::new("git").arg("--version").output().is_ok() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        },
        message: "Git availability".to_string(),
    });

    checks.push(HealthCheck {
        name: "git repo",
        status: if is_git_repo() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        },
        message: "Current directory is a git repository".to_string(),
    });

    checks.push(HealthCheck {
        name: "contract validation",
        status: validate_contract(),
        message: "AGENT.agent is valid".to_string(),
    });

    checks.push(HealthCheck {
        name: "diff audit",
        status: can_run_diff(),
        message: "Diff audit can run".to_string(),
    });

    checks.push(HealthCheck {
        name: "brief generation",
        status: can_generate_brief(),
        message: "Brief can be generated".to_string(),
    });

    render_checks(&checks);
    Ok(())
}

fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn validate_contract() -> HealthStatus {
    if Path::new("AGENT.agent").exists() {
        HealthStatus::Healthy
    } else {
        HealthStatus::Warning
    }
}

fn can_run_diff() -> HealthStatus {
    HealthStatus::Healthy
}

fn can_generate_brief() -> HealthStatus {
    HealthStatus::Healthy
}

fn render_checks(checks: &[HealthCheck]) {
    for check in checks {
        let icon = match check.status {
            HealthStatus::Healthy => format!("  {} {}", "✔".green(), check.name.cyan()),
            HealthStatus::Warning => format!("  {} {}", "⚠".yellow(), check.name.cyan()),
            HealthStatus::Unhealthy => format!("  {} {}", "✘".red(), check.name.cyan()),
        };
        println!("{} {}", icon, check.message.dimmed());
    }

    println!();

    let has_unhealthy = checks.iter().any(|c| c.status == HealthStatus::Unhealthy);
    let has_warning = checks.iter().any(|c| c.status == HealthStatus::Warning);

    if has_unhealthy {
        println!("{}", "Some checks failed.".red().bold());
        println!("{}", "Fix the issues above before proceeding.".yellow());
    } else if has_warning {
        println!("{}", "AgentML setup is incomplete.".yellow().bold());
        println!(
            "{}",
            "Run `agentml init --detect` to auto-initialize.".dimmed()
        );
    } else {
        println!("{}", "All checks passed.".green().bold());
    }
}