use ratatui::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub scroll: usize,
    pub is_running: bool,
    pub project_path: String,
}

impl App {
    pub fn new(project_path: String) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: vec![
                "Welcome to CodeAgent v0.3.0 - Interactive AI Coding Assistant".to_string(),
                "Type 'help' for commands or describe your coding task.".to_string(),
            ],
            scroll: 0,
            is_running: true,
            project_path,
        }
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }

    pub fn submit_input(&mut self) {
        if !self.input.is_empty() {
            self.add_message(format!("🤖 You: {}", self.input.clone()));
            self.process_command();
            self.input.clear();
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn process_command(&mut self) {
        match self.input.trim() {
            "help" => self.show_help(),
            "clear" => {
                self.messages.clear();
                self.add_message("Terminal cleared.".to_string());
            }
            "status" => self.show_status(),
            "exit" | "quit" => self.is_running = false,
            cmd => {
                self.add_message(format!("⏳ Processing: {}", cmd));
                self.add_message("✅ Command processed (feature coming soon)".to_string());
            }
        }
    }

    fn show_help(&mut self) {
        self.add_message("╔════ Available Commands ════╗".to_string());
        self.add_message("│ help   - Show this message │".to_string());
        self.add_message("│ clear  - Clear screen      │".to_string());
        self.add_message("│ status - Show project info │".to_string());
        self.add_message("│ exit   - Exit CodeAgent    │".to_string());
        self.add_message("╚════════════════════════════╝".to_string());
    }

    fn show_status(&mut self) {
        self.add_message(format!("📁 Project: {}", self.project_path));
        self.add_message("📊 Modules: 20 | Features: 35+ | LOC: 2,600+".to_string());
        self.add_message("✅ Status: Ready to assist".to_string());
    }
}
