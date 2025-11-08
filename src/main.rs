#![allow(dead_code, unused_variables)]

use clap::Parser;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::prelude::*;
use std::io;

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
mod tui;

use cli::Cli;
use tui::{App, EventHandler, draw, handle_input};
use crossterm::event::KeyCode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();
    let project_path = cli.path.unwrap_or_else(|| ".".to_string());

    let ollama_client = integrations::ollama::OllamaClient::new(
        "http://localhost:11434".to_string(),
        "mistral".to_string(),
    );

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(project_path);
    let events = EventHandler::new();

    let result = run_app(&mut terminal, &mut app, &events, ollama_client).await;

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    events: &EventHandler,
    ollama: integrations::ollama::OllamaClient,
) -> io::Result<()> {
    while app.is_running {
        terminal.draw(|f| draw(f, app))?;

        if let Some(event) = events.next() {
            match event {
                tui::events::AppEvent::Input(key_event) => {
                    if let KeyCode::Char(c) = key_event.code {
                        handle_input(app, KeyCode::Char(c), key_event.modifiers);
                    } else {
                        handle_input(app, key_event.code, key_event.modifiers);
                    }

                    if app.input_mode == tui::app::InputMode::Insert && key_event.code == KeyCode::Enter {
                        let prompt = app.input.clone();
                        app.input.clear();
                        
                        app.add_message("CodeAgent".to_string(), "⏳ Thinking...".to_string());
                        
                        match ollama.generate(&prompt).await {
                            Ok(response) => {
                                app.chat_messages.pop();
                                app.add_message("CodeAgent".to_string(), response);
                            }
                            Err(e) => {
                                app.chat_messages.pop();
                                app.add_message(
                                    "Error".to_string(),
                                    format!("Failed to generate response: {}", e),
                                );
                            }
                        }
                    }
                }
                tui::events::AppEvent::Tick => {
                    app.tick();
                }
            }
        }
    }

    Ok(())
}

