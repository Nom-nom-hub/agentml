use crate::parser;
use crate::types::AgentFile;
use colored::Colorize;
use std::path::Path;

pub fn run(file: std::path::PathBuf) -> anyhow::Result<()> {
    let path = Path::new(&file);
    let agent: AgentFile = parser::parse_agent_file(path)?;

    println!("{}", "══ AgentML Inspection Report ══".cyan().bold());
    println!();

    let purpose_display = agent
        .purpose
        .as_ref()
        .map(|v| serde_yaml::to_string(v).unwrap_or_default())
        .unwrap_or_default();
    println!("{} {}", "Purpose:".bold(), purpose_display.trim());
    println!();

    if let Some(meta) = agent.meta {
        println!("{}", "Meta".bold());
        println!("  Name: {}", meta.name);
        println!("  Version: {}", meta.version);
        if let Some(desc) = meta.description {
            println!("  Description: {}", desc);
        }
        println!();
    }

    if let Some(ctx) = agent.context {
        println!("{}", "Context".bold());
        if let Some(pt) = ctx.project_type {
            println!("  Project Type: {}", pt);
        }
        if let Some(langs) = ctx.languages {
            println!("  Languages: {}", langs.join(", "));
        }
        if let Some(fws) = ctx.frameworks {
            println!("  Frameworks: {}", fws.join(", "));
        }
        println!();
    }

    if let Some(perms) = agent.permissions {
        println!("{}", "Permissions".bold());
        if let Some(read) = perms.read {
            println!("  Read ({} paths):", read.len());
            for p in read {
                println!("    - {}", p);
            }
        }
        if let Some(write) = perms.write {
            println!("  Write ({} paths):", write.len());
            for p in write {
                println!("    - {}", p);
            }
        }
        if let Some(exec) = perms.execute {
            println!("  Execute ({} paths):", exec.len());
            for p in exec {
                println!("    - {}", p);
            }
        }
        println!();
    }

    if let Some(tools) = agent.tools {
        println!("{} {}", "Tools:".bold(), tools.join(", "));
        println!();
    }

    if let Some(workflows) = agent.workflows {
        println!("{}", "Workflows".bold());
        for wf in workflows {
            println!("  {}: {}", "Workflow".cyan(), wf.name);
            if let Some(desc) = wf.description {
                println!("    {}", desc.dimmed());
            }
            for step in &wf.steps {
                println!("    - {}", step.name);
            }
        }
        println!();
    }

    if let Some(tasks) = agent.tasks {
        println!("{}", "Tasks".bold());
        for task in tasks {
            println!("  {}: {}", "Task".cyan(), task.name);
            if let Some(desc) = task.description {
                println!("    {}", desc.dimmed());
            }
            if let Some(success) = task.success {
                println!("    Success: {}", success);
            }
        }
        println!();
    }

    if let Some(safety) = agent.safety {
        println!("{}", "Safety".bold());
        println!("  YAML: {:?}", safety);
        println!();
    }

    if let Some(validation) = agent.validation {
        println!("{}", "Validation Commands".bold());
        for v in validation {
            println!("  - {}", v.name);
            println!("    {}", v.command.dimmed());
        }
        println!();
    }

    if let Some(output) = agent.output {
        println!("{}", "Output Config".bold());
        if let Some(fmt) = output.format {
            println!("  Format: {}", fmt);
        }
        if let Some(sections) = output.required_sections {
            println!("  Required Sections: {}", sections.join(", "));
        }
    }

    Ok(())
}
