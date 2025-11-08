use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "CodeAgent",
    version = "0.1.0",
    about = "AI-powered code automation agent",
    long_about = "A powerful CLI agent that understands your project context and autonomously executes complex, multi-step engineering tasks."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Run a task with a natural language prompt")]
    Run {
        #[arg(help = "Natural language description of the task to perform")]
        prompt: String,

        #[arg(short, long, help = "Preview changes without applying them")]
        dry_run: bool,
    },

    #[command(about = "Initialize a new CodeAgent project")]
    Init {
        #[arg(default_value = ".", help = "Path to initialize the project")]
        path: PathBuf,
    },
}
