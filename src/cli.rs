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
    #[command(alias = "init", about = "Initialize AgentML in a project")]
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
        #[arg(long)]
        syntax: Option<String>,
    },
    #[command(alias = "check", about = "Validate an AGENT.agent contract file")]
    Validate {
        file: PathBuf,
        #[arg(short, long)]
        strict: bool,
        #[arg(long)]
        format: Option<String>,
    },
    #[command(about = "Run a task defined in the contract")]
    Run { task: String, file: PathBuf },
    #[command(about = "Generate project context from the contract")]
    Context {
        #[arg(default_value = "AGENT.agent")]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    #[command(about = "Inspect project structure and detect project type")]
    Inspect {},
    #[command(about = "Generate an operating brief for AI agents")]
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
    #[command(about = "Generate AGENTS.md from the contract")]
    AgentsMd {
        #[arg(long)]
        write: bool,
        #[arg(long)]
        force: bool,
    },
    #[command(about = "Start the MCP server for agent integration")]
    Mcp {},
    #[command(about = "Manage AgentML skills")]
    Skill {
        #[command(subcommand)]
        skill: SkillCommands,
    },
    #[command(about = "Close a task with validation and risk report")]
    Close {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        require_clean: bool,
        #[arg(long)]
        fail_at_risk: Option<u32>,
        #[arg(long)]
        write_report: bool,
    },
    #[command(about = "Run AgentML self-check on this project")]
    SelfCheck {},
    #[command(about = "Audit uncommitted changes with risk scoring")]
    Diff {},
    #[command(about = "Check project health and configuration")]
    Doctor {},
    #[command(about = "Generate shell completions")]
    Completions { shell: String },
    #[command(about = "Show version information")]
    Version {},
    #[command(about = "Convert between YAML and native syntax")]
    Convert {
        #[arg(long)]
        to: String,
        #[arg(long)]
        write: bool,
        #[arg(long)]
        backup: bool,
        file: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    #[command(about = "Validate a skill file")]
    Validate {
        file: PathBuf,
        #[arg(long)]
        format: Option<String>,
    },
    #[command(about = "Pack a skill folder into a .skill file")]
    Pack { folder: PathBuf },
    #[command(about = "List available skills")]
    List,
    #[command(about = "Inspect a skill file")]
    Inspect { path: String },
    #[command(about = "Match skills against the current project")]
    Match,
}
