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
            no_agents_md,
            no_context,
            no_brief,
            syntax,
        }) => {
            if force {
                unsafe { std::env::set_var("AGENTML_FORCE_INIT", "1") };
            }
            agentml::commands::init::run(
                path,
                template,
                detect,
                no_agents_md,
                no_context,
                no_brief,
                syntax.as_deref(),
            )
        }
        Some(agentml::cli::Commands::Validate {
            file,
            strict,
            format,
        }) => {
            let fmt = format.unwrap_or_else(|| "auto".to_string());
            agentml::commands::validate::run(file, strict, &fmt)
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
        Some(agentml::cli::Commands::AgentsMd { write, force }) => {
            agentml::commands::agents_md::run(write, force)
        }
        Some(agentml::cli::Commands::Skill { skill }) => match skill {
            agentml::cli::SkillCommands::Validate { file, format } => {
                let fmt = format.unwrap_or_else(|| "auto".to_string());
                agentml::commands::skill::validate::run(file, &fmt)
            }
            agentml::cli::SkillCommands::Pack { folder } => {
                agentml::commands::skill::pack::run(folder)
            }
            agentml::cli::SkillCommands::List => agentml::commands::skill::run_list(),
            agentml::cli::SkillCommands::Inspect { path } => {
                agentml::commands::skill::run_inspect(&path)
            }
            agentml::cli::SkillCommands::Match => agentml::commands::skill::run_match(),
        },
        Some(agentml::cli::Commands::SelfCheck {}) => agentml::commands::self_check::run(),
        Some(agentml::cli::Commands::Close {
            json,
            require_clean,
            fail_at_risk,
            write_report,
        }) => agentml::commands::close::run(json, require_clean, fail_at_risk, write_report),
        Some(agentml::cli::Commands::Diff {}) => agentml::commands::diff::run(),
        Some(agentml::cli::Commands::Doctor {}) => agentml::commands::doctor::run(),
        Some(agentml::cli::Commands::Convert {
            to,
            write,
            backup,
            file,
        }) => agentml::commands::convert::run(&to, write, backup, file),
        None => {
            eprintln!("No command provided. Use --help for usage.");
            std::process::exit(1);
        }
    }
}
