use crate::parser;
use crate::types::AgentFile;
use colored::Colorize;
use std::path::Path;

pub fn run(file: std::path::PathBuf, task_name: String) -> anyhow::Result<()> {
    let path = Path::new(&file);
    let agent: AgentFile = parser::parse_agent_file(path)?;

    let task = agent
        .tasks
        .as_ref()
        .and_then(|tasks| tasks.iter().find(|t| t.name == task_name))
        .cloned();

    match task {
        Some(t) => {
            println!("{} {}", "Running task:".cyan().bold(), t.name);
            if let Some(desc) = t.description {
                println!("{}", desc.dimmed());
            }
            if let Some(workflow) = t.workflow
                && let Some(wfs) = &agent.workflows
            {
                if let Some(wf) = wfs.iter().find(|w| w.name == workflow) {
                    println!("\n{} {}", "Workflow:".bold(), wf.name);
                    for (i, step) in wf.steps.iter().enumerate() {
                        println!("  {}. {}", i + 1, step.name);
                        if let Some(cmd) = &step.commands {
                            for c in cmd {
                                println!("     {}", c.dimmed());
                            }
                        }
                    }
                } else {
                    println!("Warning: Workflow '{}' not found", workflow.yellow());
                }
            }
            println!(
                "\n{}",
                "Task execution plan displayed (MVP does not execute commands)".dimmed()
            );
            Ok(())
        }
        None => {
            println!("{} {}", "Task not found:".red(), task_name);
            if let Some(tasks) = agent.tasks {
                println!("\n{}", "Available tasks:".cyan());
                for t in tasks {
                    println!("  - {}", t.name);
                }
            }
            std::process::exit(1);
        }
    }
}
