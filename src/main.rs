use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing_subscriber;

mod cli;
mod cache;
mod config;
mod core;
mod error_correction;
mod integrations;
mod interactive;
mod llm;
mod parsers;
mod pr_generator;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Run { prompt, dry_run } => {
            core::executor::run_task(&prompt, dry_run).await?;
        }
        cli::Command::Init { path } => {
            core::config::init_project(&path)?;
        }
    }

    Ok(())
}
