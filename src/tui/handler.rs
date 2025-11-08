use crossterm::event::{KeyCode, KeyModifiers};
use crate::tui::app::{App, InputMode, Tab};

pub fn handle_input(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) {
    match app.input_mode {
        InputMode::Normal => match key_code {
            KeyCode::Char('i') => {
                app.input_mode = InputMode::Insert;
                app.status_bar.mode = "INSERT".to_string();
            }
            KeyCode::Char(':') => {
                app.input_mode = InputMode::Command;
                app.status_bar.mode = "COMMAND".to_string();
                app.input.push(':');
            }
            KeyCode::Char('/') => {
                app.input_mode = InputMode::Search;
                app.status_bar.mode = "SEARCH".to_string();
            }
            KeyCode::Char('q') => {
                app.is_running = false;
            }
            KeyCode::Char('?') => {
                app.show_help = !app.show_help;
            }
            KeyCode::Tab => {
                app.cycle_tab();
            }
            KeyCode::BackTab => {
                for _ in 0..3 {
                    app.cycle_tab();
                }
            }
            KeyCode::Char('j') => {
                if app.selected_file < app.file_tree.len() - 1 {
                    app.selected_file += 1;
                }
            }
            KeyCode::Char('k') => {
                if app.selected_file > 0 {
                    app.selected_file -= 1;
                }
            }
            _ => {}
        },
        InputMode::Insert => match key_code {
            KeyCode::Enter => {
                app.submit_input();
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => {
                app.input.clear();
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Char(c) => {
                if app.input.len() < 1000 {
                    app.input.push(c);
                }
            }
            KeyCode::Tab if modifiers.contains(KeyModifiers::SHIFT) => {
                for _ in 0..3 {
                    app.cycle_tab();
                }
            }
            KeyCode::Tab => {
                app.cycle_tab();
            }
            _ => {}
        },
        InputMode::Command => match key_code {
            KeyCode::Enter => {
                let cmd = app.input.trim_start_matches(':').to_string();
                let response = app.process_command(&cmd);
                app.add_message("System".to_string(), response);
                app.input.clear();
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Backspace => {
                if app.input.len() > 1 {
                    app.input.pop();
                }
            }
            KeyCode::Esc => {
                app.input.clear();
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Up => {
                if app.command_index < app.command_history.len() - 1 {
                    app.command_index += 1;
                    if let Some(cmd) = app.command_history.get(app.command_index) {
                        app.input = format!(":{}", cmd);
                    }
                }
            }
            KeyCode::Down => {
                if app.command_index > 0 {
                    app.command_index -= 1;
                    if let Some(cmd) = app.command_history.get(app.command_index) {
                        app.input = format!(":{}", cmd);
                    }
                } else {
                    app.input = ":".to_string();
                }
            }
            _ => {}
        },
        InputMode::Search => match key_code {
            KeyCode::Enter => {
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Backspace => {
                app.search_query.pop();
            }
            KeyCode::Esc => {
                app.search_query.clear();
                app.input_mode = InputMode::Normal;
                app.status_bar.mode = "NORMAL".to_string();
            }
            KeyCode::Char(c) => {
                app.search_query.push(c);
            }
            _ => {}
        },
    }
}
