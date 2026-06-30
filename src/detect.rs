use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct ProjectInfo {
    pub project_type: String,
    pub package_manager: Option<String>,
    pub detected_scripts: Vec<(String, String)>,
    pub important_files: Vec<String>,
    pub validation_commands: Vec<String>,
}

pub fn detect_project() -> Result<ProjectInfo> {
    let mut info = ProjectInfo::default();

    if Path::new("Cargo.toml").exists() {
        info.project_type = "Rust".to_string();
        info.validation_commands = vec![
            "cargo fmt --check".to_string(),
            "cargo clippy --all-targets -- -D warnings".to_string(),
            "cargo test".to_string(),
        ];
        info.important_files = vec![
            "Cargo.toml".to_string(),
            "src/**/*.rs".to_string(),
            "tests/**/*.rs".to_string(),
        ];
        return Ok(info);
    }

    if Path::new("package.json").exists() {
        let package_json = fs::read_to_string("package.json")?;
        let pkg: serde_json::Value = serde_json::from_str(&package_json)?;

        if pkg
            .get("dependencies")
            .and_then(|d| d.get("next"))
            .is_some()
        {
            info.project_type = "Next.js".to_string();
            info.important_files = vec![
                "app/**/*.tsx".to_string(),
                "src/app/**/*.tsx".to_string(),
                "components/**/*.tsx".to_string(),
                "next.config.*".to_string(),
                "package.json".to_string(),
            ];
        } else if pkg
            .get("dependencies")
            .and_then(|d| d.get("vite"))
            .is_some()
        {
            info.project_type = "Vite".to_string();
            info.important_files = vec![
                "src/**/*.ts".to_string(),
                "src/**/*.tsx".to_string(),
                "index.html".to_string(),
                "vite.config.*".to_string(),
            ];
        } else {
            info.project_type = "Node".to_string();
            info.important_files = vec![
                "src/**/*.ts".to_string(),
                "src/**/*.js".to_string(),
                "package.json".to_string(),
            ];
        }

        if Path::new("pnpm-lock.yaml").exists() {
            info.package_manager = Some("pnpm".to_string());
        } else if Path::new("yarn.lock").exists() {
            info.package_manager = Some("yarn".to_string());
        } else {
            info.package_manager = Some("npm".to_string());
        }

        if let Some(scripts) = pkg.get("scripts").and_then(|s| s.as_object()) {
            for (name, _) in scripts {
                info.detected_scripts.push((
                    name.clone(),
                    format!(
                        "{} run {}",
                        info.package_manager.as_deref().unwrap_or("npm"),
                        name
                    ),
                ));
            }
            if scripts.contains_key("lint") {
                info.validation_commands.push(format!(
                    "{} run lint",
                    info.package_manager.as_deref().unwrap_or("npm")
                ));
            }
            if scripts.contains_key("test") {
                info.validation_commands.push(format!(
                    "{} run test",
                    info.package_manager.as_deref().unwrap_or("npm")
                ));
            }
            if scripts.contains_key("typecheck") || scripts.contains_key("type-check") {
                info.validation_commands.push(format!(
                    "{} run typecheck",
                    info.package_manager.as_deref().unwrap_or("npm")
                ));
            }
            if scripts.contains_key("build") {
                info.validation_commands.push(format!(
                    "{} run build",
                    info.package_manager.as_deref().unwrap_or("npm")
                ));
            }
        }

        return Ok(info);
    }

    if Path::new("pyproject.toml").exists()
        || Path::new("requirements.txt").exists()
        || Path::new("setup.py").exists()
    {
        info.project_type = "Python".to_string();
        info.package_manager = None;
        info.important_files = vec![
            "src/**/*.py".to_string(),
            "tests/**/*.py".to_string(),
            "pyproject.toml".to_string(),
        ];

        let pyproject = fs::read_to_string("pyproject.toml").unwrap_or_default();
        if pyproject.contains("pytest") {
            info.validation_commands.push("pytest".to_string());
        }
        if pyproject.contains("ruff") || Path::new("ruff.toml").exists() {
            info.validation_commands.push("ruff check .".to_string());
        }
        if pyproject.contains("mypy") {
            info.validation_commands.push("mypy .".to_string());
        }

        return Ok(info);
    }

    info.project_type = "Generic".to_string();
    info.important_files = vec![
        "src/**".to_string(),
        "docs/**".to_string(),
        "README.md".to_string(),
    ];
    println!(
        "{}: No known stack detected. Generated conservative generic AGENT.agent.",
        "Warning".yellow()
    );

    Ok(info)
}

pub fn print_inspect(info: &ProjectInfo) {
    println!("{}", "══ AgentML Project Inspect ══".cyan().bold());
    println!();
    println!("{}", "Project type:".bold());
    println!("  {}", info.project_type);
    println!();
    if let Some(pm) = &info.package_manager {
        println!("{}", "Package manager:".bold());
        println!("  {}", pm);
        println!();
    }
    if !info.detected_scripts.is_empty() {
        println!("{}", "Detected scripts:".bold());
        for (name, cmd) in &info.detected_scripts {
            println!("  {}: {}", name, cmd);
        }
        println!();
    }
    println!("{}", "Important files:".bold());
    for f in &info.important_files {
        println!("  {}", f);
    }
    println!();
    println!("{}", "Recommended validation:".bold());
    for cmd in &info.validation_commands {
        println!("  {}", cmd);
    }
    println!();
    println!("{}", "Status:".bold());
    if info.project_type == "Generic" {
        println!(
            "  {}",
            "Unknown stack - manual configuration needed".yellow()
        );
    } else {
        println!("  {}", "Ready to generate AGENT.agent".green());
    }
}
