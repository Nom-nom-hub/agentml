use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "agentml")]
#[command(about = "AI-native markup language and CLI for agent execution contracts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "init")]
    Initialize {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        template: Option<String>,
    },
    #[command(alias = "check")]
    Validate {
        file: PathBuf,
        #[arg(short, long)]
        strict: bool,
    },
    Inspect {
        file: PathBuf,
    },
    Run {
        task: String,
        file: PathBuf,
    },
    Context {
        #[arg(default_value = "AGENT.agent")]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Skill {
        #[command(subcommand)]
        skill: SkillCommands,
    },
    SelfCheck {},
}

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    Validate { file: PathBuf },
    Pack { folder: PathBuf },
}
