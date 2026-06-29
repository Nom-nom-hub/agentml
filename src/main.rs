use agentml::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        agentml::cli::Commands::Initialize { path, template } => {
            agentml::commands::init::run(path, template)
        }
        agentml::cli::Commands::Validate { file, strict } => {
            agentml::commands::validate::run(file, strict)
        }
        agentml::cli::Commands::Inspect { file } => agentml::commands::inspect::run(file),
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
    }
}
