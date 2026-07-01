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

fn convert_yaml_to_native(content: &str) -> anyhow::Result<String> {
    let agent: crate::types::AgentFile = serde_yaml::from_str(content)
        .map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;

    let mut output = String::new();
    let name = agent
        .meta
        .as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_default();
    output.push_str(&format!("agent \"{}\" {{\n", name));

    if let Some(ref meta) = agent.meta
        && !meta.version.is_empty()
    {
        output.push_str(&format!("  version \"{}\"\n", meta.version));
    }

    if let Some(ref purpose) = agent.purpose {
        output.push_str("  purpose {\n");
        if let Some(human) = purpose.get("human_goal").and_then(|v| v.as_str()) {
            output.push_str(&format!("    human_goal \"{}\"\n", human));
        }
        if let Some(agent_goal) = purpose.get("agent_goal").and_then(|v| v.as_str()) {
            output.push_str(&format!("    agent_goal \"{}\"\n", agent_goal));
        }
        if let Some(non_goals) = purpose.get("non_goals").and_then(|v| v.as_sequence()) {
            output.push_str("    non_goals [");
            let items: Vec<String> = non_goals
                .iter()
                .filter_map(|s| s.as_str())
                .map(|s| format!("\"{}\"", s))
                .collect();
            output.push_str(&items.join(", "));
            output.push_str("]\n");
        }
        output.push_str("  }\n");
    }

    if let Some(ref perms) = agent.permissions {
        output.push_str("  permissions {\n");
        if let Some(ref read) = perms.read {
            output.push_str(&format!(
                "    read: [{}]\n",
                read.iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(ref write) = perms.write {
            output.push_str(&format!(
                "    write: [{}]\n",
                write
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(ref exec) = perms.execute {
            output.push_str(&format!(
                "    execute: [{}]\n",
                exec.iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        output.push_str("  }\n");
    }

    if let Some(ref validation) = agent.validation {
        output.push_str("  validation {\n");
        for cmd in validation {
            output.push_str(&format!("    command: \"{}\"\n", cmd.command));
        }
        output.push_str("  }\n");
    }

    if let Some(ref safety) = agent.safety
        && let Some(obj) = safety.as_mapping()
    {
        output.push_str("  safety {\n");
        let key = serde_yaml::Value::String("forbidden_actions".to_string());
        if let Some(rules) = obj.get(&key)
            && let Some(r) = rules.as_sequence()
        {
            output.push_str(&format!(
                "    forbidden_actions: [{}]\n",
                r.iter()
                    .filter_map(|x| x.as_str())
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        output.push_str("  }\n");
    }

    if let Some(ref output_spec) = agent.output {
        output.push_str("  output {\n");
        if let Some(ref sections) = output_spec.required_sections {
            output.push_str(&format!(
                "    required: [{}]\n",
                sections
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        output.push_str("  }\n");
    }

    output.push_str("}\n");

    Ok(output)
}
