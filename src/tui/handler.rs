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
            KeyCode::Char('c') => {
                app.messages.clear();
                app.add_message("Terminal cleared.".to_string());
            }
            _ => {}
        },
        InputMode::Editing => match key_code {
            KeyCode::Enter => {
                app.submit_input();
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            _ => {}
        },
    }
}
