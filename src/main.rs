use agentml::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(agentml::cli::Commands::Mcp {}) => agentml::mcp::run(),
        Some(agentml::cli::Commands::Version {}) => {
            println!("agentml {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some(agentml::cli::Commands::Completions { shell }) => {
            agentml::commands::completions::run(&shell)
        }
        Some(agentml::cli::Commands::Initialize {
            path,
            template,
            force,
            detect,
        }) => {
            if force {
                unsafe { std::env::set_var("AGENTML_FORCE_INIT", "1") };
            }
            agentml::commands::init::run(path, template, detect)
        }
        Some(agentml::cli::Commands::Validate { file, strict }) => {
            agentml::commands::validate::run(file, strict)
        }
        Some(agentml::cli::Commands::Inspect {}) => agentml::commands::inspect::run(),
        Some(agentml::cli::Commands::Run { task, file }) => agentml::commands::run::run(file, task),
        Some(agentml::cli::Commands::Context { file, output }) => {
            agentml::commands::context::run(file, output)
        }
        Some(agentml::cli::Commands::Brief {
            format,
            write,
            max_lines,
            include_diff,
            no_diff,
        }) => {
            let fmt = format.unwrap_or_else(|| "md".to_string());
            let max = max_lines.unwrap_or(80);
            agentml::commands::brief::run(&fmt, write, max, include_diff && !no_diff)
        }
        Some(agentml::cli::Commands::Skill { skill }) => match skill {
            agentml::cli::SkillCommands::Validate { file } => {
                agentml::commands::skill::validate::run(file)
            }
            agentml::cli::SkillCommands::Pack { folder } => {
                agentml::commands::skill::pack::run(folder)
            }
        },
        Some(agentml::cli::Commands::SelfCheck {}) => agentml::commands::self_check::run(),
        Some(agentml::cli::Commands::Diff {}) => agentml::commands::diff::run(),
        Some(agentml::cli::Commands::Doctor {}) => agentml::commands::doctor::run(),
        None => {
            eprintln!("No command provided. Use --help for usage.");
            std::process::exit(1);
        }
    }
}
