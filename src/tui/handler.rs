use crossterm::event::KeyCode;
use crate::tui::app::{App, InputMode};

pub fn handle_input(app: &mut App, key_code: KeyCode) {
    match app.input_mode {
        InputMode::Normal => match key_code {
            KeyCode::Char('i') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                app.is_running = false;
            }
            KeyCode::Char(':') => {
                app.input_mode = InputMode::CommandPalette;
                app.command_palette_input.push(':');
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
            KeyCode::Tab => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('?') => {
                app.add_message(
                    "Help".to_string(),
                    "🎯 Keybindings:\ni - Enter edit mode\nj/k - Navigate files\nq - Quit\n: - Command palette\nESC - Return to normal".to_string(),
                );
            }
            _ => {}
        },
        InputMode::Editing => match key_code {
            KeyCode::Enter => {
                app.submit_input();
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => {
                app.input.clear();
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                if app.input.len() < 500 {
                    app.input.push(c);
                }
            }
            KeyCode::Tab => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
        InputMode::CommandPalette => match key_code {
            KeyCode::Enter => {
                let cmd = app.command_palette_input.trim_start_matches(':').to_string();
                let response = app.process_command(&cmd);
                app.add_message("System".to_string(), response);
                app.command_palette_input.clear();
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                app.command_palette_input.pop();
                if app.command_palette_input == ":" {
                    app.command_palette_input.clear();
                    app.input_mode = InputMode::Normal;
                }
            }
            KeyCode::Esc => {
                app.command_palette_input.clear();
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                app.command_palette_input.push(c);
            }
            _ => {}
        },
    }
}
