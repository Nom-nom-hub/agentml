use crate::parser::parse_agent_file;
use crate::types::AgentFile;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(write: bool, force: bool) -> Result<()> {
    let agent_path = Path::new("AGENT.agent");
    if !agent_path.exists() {
        anyhow::bail!("AGENT.agent not found in current directory");
    }
    let agent = parse_agent_file(agent_path)?;
    let markdown = generate(&agent);

    if write {
        let md_path = Path::new("AGENTS.md");
        if md_path.exists() && !force {
            println!(
                "{}",
                "AGENTS.md already exists. Use --force to overwrite.".yellow()
            );
            return Ok(());
        }
        fs::write(md_path, &markdown)?;
        println!("{}", "Written to AGENTS.md".green());
    } else {
        println!("{}", markdown);
    }

    Ok(())
}

pub fn generate(agent: &AgentFile) -> String {
    let mut output = String::new();

    output.push_str("# AGENTS.md\n\n");
    output.push_str("## Purpose\n\n");
    output.push_str(
        "This project uses AgentML to define how AI coding agents should safely work in this repository.\n\n",
    );
    output.push_str("The machine-readable source of truth is `AGENT.agent`.\n\n");
    output.push_str("## Required first steps\n\n");
    output.push_str("Before editing files, agents should read:\n\n");
    output.push_str("1. `AGENT.agent`\n");
    output.push_str("2. `.agentml/brief.md`\n");
    output.push_str("3. `.agentml/context.md`\n\n");
    output.push_str("Recommended command:\n\n```bash\nagentml brief\n```\n\n");
    output.push_str("If AgentML is not installed, read AGENT.agent directly.\n\n");

    output.push_str("## Project context\n\n");
    output.push_str("### Stack\n\n");

    let mut stack = Vec::new();
    if let Some(ctx) = &agent.context {
        if let Some(pt) = &ctx.project_type {
            stack.push(pt.clone());
        }
        if let Some(langs) = &ctx.languages {
            for lang in langs {
                stack.push(lang.clone());
            }
        }
        if let Some(fw) = &ctx.frameworks {
            for f in fw {
                stack.push(f.clone());
            }
        }
    }
    if stack.is_empty() {
        stack.push("Generic".to_string());
    }
    for s in &stack {
        output.push_str(&format!("- {}\n", s));
    }
    output.push('\n');

    output.push_str("### Important files\n\n");
    if let Some(perms) = &agent.permissions
        && let Some(read) = &perms.read
    {
        for p in read {
            output.push_str(&format!("- {}\n", p));
        }
    }
    output.push('\n');

    output.push_str("## Allowed work areas\n\n");
    output.push_str("Agents may usually modify:\n\n");
    if let Some(perms) = &agent.permissions
        && let Some(write) = &perms.write
    {
        for p in write {
            output.push_str(&format!("- {}\n", p));
        }
    }
    output.push('\n');
    output.push_str("Always check AGENT.agent for the authoritative list.\n\n");

    output.push_str("## Forbidden areas\n\n");
    output.push_str("Agents must not modify or expose:\n\n");
    if let Some(safety) = &agent.safety
        && let Some(obj) = safety.as_mapping()
    {
        if let Some(paths) = obj
            .get(serde_yaml::Value::String("forbidden_paths".to_string()))
            .and_then(|v| v.as_sequence())
        {
            for p in paths {
                if let Some(s) = p.as_str() {
                    output.push_str(&format!("- {}\n", s));
                }
            }
        }
        if let Some(sp) = obj.get(serde_yaml::Value::String("secrets_policy".to_string()))
            && let Some(sp_map) = sp.as_mapping()
            && let Some(nr) = sp_map
                .get(serde_yaml::Value::String("never_read".to_string()))
                .and_then(|v| v.as_sequence())
        {
            output.push_str("\nNever read:\n");
            for p in nr {
                if let Some(s) = p.as_str() {
                    output.push_str(&format!("- {}\n", s));
                }
            }
        }
    }

    output.push('\n');
    output.push_str("## Validation commands\n\n");
    output.push_str("Before reporting completion, run:\n\n");
    if let Some(validation) = &agent.validation {
        for v in validation {
            output.push_str(&format!("- `{}` — {}\n", v.command, v.name));
        }
    }
    output.push('\n');

    output.push_str("## Diff audit\n\n");
    output.push_str("After making changes, run:\n\n```bash\nagentml diff\n```\n\n");
    output.push_str("Include the risk score in the final report.\n\n");

    output.push_str("## Final report format\n\n");
    output.push_str("Every agent task should end with:\n\n");
    output.push_str(
        "```\nSummary:\nFiles changed:\nCommands run:\nValidation result:\nRisk score:\nRisks:\nNext steps:\n```\n\n",
    );

    output.push_str("## Source of truth\n\n");
    output.push_str("If files disagree, follow this order:\n\n");
    output.push_str("1. `AGENT.agent`\n");
    output.push_str("2. `.agentml/brief.md`\n");
    output.push_str("3. `.agentml/context.md`\n");
    output.push_str("4. `AGENTS.md`\n");
    output.push_str("5. `README.md`\n");

    output
}
