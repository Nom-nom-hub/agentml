use agentml::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        agentml::cli::Commands::Initialize {
            path,
            template,
            force,
            detect,
        } => {
            if force {
                unsafe { std::env::set_var("AGENTML_FORCE_INIT", "1") };
            }
            agentml::commands::init::run(path, template, detect)
        }
        agentml::cli::Commands::Validate { file, strict } => {
            agentml::commands::validate::run(file, strict)
        }
        agentml::cli::Commands::Inspect {} => agentml::commands::inspect::run(),
        agentml::cli::Commands::Run { task, file } => agentml::commands::run::run(file, task),
        agentml::cli::Commands::Context { file, output } => {
            agentml::commands::context::run(file, output)
        }
        agentml::cli::Commands::Skill { skill } => match skill {
            agentml::cli::SkillCommands::Validate { file } => {
                agentml::commands::skill::validate::run(file)
            }
            agentml::cli::SkillCommands::Pack { folder } => {
                agentml::commands::skill::pack::run(folder)
            }
        },
        agentml::cli::Commands::SelfCheck {} => agentml::commands::self_check::run(),
        agentml::cli::Commands::Diff {} => agentml::commands::diff::run(),
        agentml::cli::Commands::Doctor {} => agentml::commands::doctor::run(),
    }
}
