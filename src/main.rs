#![allow(dead_code, unused_variables)]

use clap::Parser;
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

use cli::{Cli, InteractiveSession};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();
    let mut session = InteractiveSession::new(cli.path);
    session.run();

    Ok(())
}
