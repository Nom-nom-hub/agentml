use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "agentml")]
#[command(about = "AI-native markup language and CLI for agent execution contracts", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "init")]
    Initialize {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long)]
        template: Option<String>,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        detect: bool,
        #[arg(long)]
        no_agents_md: bool,
        #[arg(long)]
        no_context: bool,
        #[arg(long)]
        no_brief: bool,
    },
    #[command(alias = "check")]
    Validate {
        file: PathBuf,
        #[arg(short, long)]
        strict: bool,
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
    Inspect {},
    Brief {
        #[arg(long)]
        format: Option<String>,
        #[arg(long)]
        write: bool,
        #[arg(long)]
        max_lines: Option<usize>,
        #[arg(long)]
        include_diff: bool,
        #[arg(long)]
        no_diff: bool,
    },
    AgentsMd {
        #[arg(long)]
        write: bool,
        #[arg(long)]
        force: bool,
    },
    Mcp {},
    Skill {
        #[command(subcommand)]
        skill: SkillCommands,
    },
    SelfCheck {},
    Diff {},
    Doctor {},
    Completions {
        shell: String,
    },
    Version {},
}

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    Validate { file: PathBuf },
    Pack { folder: PathBuf },
}
