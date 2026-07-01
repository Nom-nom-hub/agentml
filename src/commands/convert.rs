use std::path::Path;

pub fn run(to: &str, write: bool, backup: bool, file: std::path::PathBuf) -> anyhow::Result<()> {
    if to != "native" {
        anyhow::bail!("Only --to native is supported for now");
    }

    let path = Path::new(&file);
    let content = std::fs::read_to_string(path)?;

    if crate::syntax::is_native_syntax(&content) {
        anyhow::bail!(
            "Input file appears to be already native syntax, or format could not be detected"
        );
    }

    let warnings = check_conversion_warnings(&content);
    if !warnings.is_empty() {
        eprintln!("Warning: the following YAML fields will not be converted:");
        for w in warnings {
            eprintln!("  - {}", w);
        }
        eprintln!("See docs/migration.md for details.\n");
    }

    let output = convert_yaml_to_native(&content)?;

    if write {
        if backup && path.exists() {
            let backup_path = format!("{}.bak", path.display());
            std::fs::copy(path, &backup_path)?;
        }
        std::fs::write(path, &output)?;
        println!("Written to {}", path.display());
    } else {
        print!("{}", output);
    }

    Ok(())
}

fn check_conversion_warnings(content: &str) -> Vec<String> {
    let agent: serde_yaml::Value = match serde_yaml::from_str(content) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    let mut warnings = Vec::new();

    if agent.get("contract_version").is_some() {
        warnings.push("contract_version".to_string());
    }
    if agent.get("diff_policy").is_some() {
        warnings.push("diff_policy".to_string());
    }
    if agent.get("tasks").is_some() {
        warnings.push("tasks".to_string());
    }
    if agent.get("workflows").is_some() {
        warnings.push("workflows".to_string());
    }
    if agent.get("memory").is_some() {
        warnings.push("memory".to_string());
    }
    if agent.get("tools").is_some() {
        warnings.push("tools".to_string());
    }
    if agent.get("success_criteria").is_some() {
        warnings.push("success_criteria".to_string());
    }

    warnings
}

fn convert_yaml_to_native(content: &str) -> anyhow::Result<String> {
    let agent: crate::types::AgentFile = serde_yaml::from_str(content)
        .map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;

    let mut lines = Vec::new();
    let name = agent
        .meta
        .as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "agent".to_string());
    lines.push(format!("agent \"{}\" {{", name));

    if let Some(ref meta) = agent.meta
        && !meta.version.is_empty()
    {
        lines.push(format!("  version \"{}\"", meta.version));
    }

    if let Some(ref purpose) = agent.purpose {
        lines.push("  purpose {".to_string());
        if let Some(human) = &purpose.human_goal {
            lines.push(format!("    human_goal \"{}\"", human));
        }
        if let Some(agent_goal) = &purpose.agent_goal {
            lines.push(format!("    agent_goal \"{}\"", agent_goal));
        }
        if let Some(non_goals) = &purpose.non_goals {
            let items: Vec<String> = non_goals.iter().map(|s| format!("\"{}\"", s)).collect();
            lines.push(format!("    non_goals [{}]", items.join(", ")));
        }
        lines.push("  }".to_string());
    }

    if let Some(ref context) = agent.context {
        lines.push("  context {".to_string());
        if let Some(ref stack) = context.languages {
            let items: Vec<String> = stack.clone();
            lines.push(format!("    stack: [{}]", items.join(", ")));
        }
        lines.push("  }".to_string());
    }

    if let Some(ref perms) = agent.permissions {
        lines.push("  permissions {".to_string());
        if let Some(ref read) = perms.read {
            let items: Vec<String> = read.iter().map(|s| format!("\"{}\"", s)).collect();
            lines.push(format!("    read: [{}]", items.join(", ")));
        }
        if let Some(ref write) = perms.write {
            let items: Vec<String> = write.iter().map(|s| format!("\"{}\"", s)).collect();
            lines.push(format!("    write: [{}]", items.join(", ")));
        }
        if let Some(ref exec) = perms.execute {
            let items: Vec<String> = exec.iter().map(|s| format!("\"{}\"", s)).collect();
            lines.push(format!("    execute: [{}]", items.join(", ")));
        }
        lines.push("  }".to_string());
    }

    if let Some(ref safety) = agent.safety {
        lines.push("  safety {".to_string());
        if let Some(ref forbidden_paths) = safety.forbidden_paths {
            let items: Vec<String> = forbidden_paths
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect();
            lines.push(format!("    forbidden_paths: [{}]", items.join(", ")));
        }
        if let Some(ref forbidden_actions) = safety.forbidden_actions {
            let items: Vec<String> = forbidden_actions
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect();
            lines.push(format!("    forbidden_actions: [{}]", items.join(", ")));
        }
        if let Some(ref require_confirmation) = safety.require_confirmation {
            let items: Vec<String> = require_confirmation
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect();
            lines.push(format!("    require_approval: [{}]", items.join(", ")));
        }
        if let Some(ref policy) = safety.policy {
            lines.push(format!("    policy: \"{}\"", policy));
        }
        lines.push("  }".to_string());
    }

    if let Some(ref validation) = agent.validation {
        lines.push("  validation {".to_string());
        for cmd in validation {
            lines.push(format!("    command: \"{}\"", cmd.command));
        }
        lines.push("  }".to_string());
    }

    if let Some(ref output_spec) = agent.output {
        lines.push("  output {".to_string());
        if let Some(ref sections) = output_spec.required_sections {
            let items: Vec<String> = sections.iter().map(|s| format!("\"{}\"", s)).collect();
            lines.push(format!("    required: [{}]", items.join(", ")));
        }
        lines.push("  }".to_string());
    }

    lines.push("}".to_string());

    Ok(lines.join("\n"))
}
