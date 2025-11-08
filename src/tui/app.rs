use ratatui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Explorer,
    Editor,
    Output,
    Chat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    CommandPalette,
}

pub struct EditorState {
    pub content: String,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub scroll_x: usize,
    pub scroll_y: usize,
}

pub struct App {
    pub is_running: bool,
    pub input_mode: InputMode,
    pub active_panel: Panel,
    pub input: String,
    pub chat_messages: Vec<ChatMessage>,
    pub output: Vec<String>,
    pub editor: EditorState,
    pub project_path: String,
    pub file_tree: Vec<FileNode>,
    pub selected_file: usize,
    pub command_palette_input: String,
    pub animation_frame: u64,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub author: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub depth: usize,
}

impl App {
    pub fn new(project_path: String) -> Self {
        let file_tree = vec![
            FileNode {
                name: "src/".to_string(),
                path: "src".to_string(),
                is_dir: true,
                expanded: true,
                depth: 0,
            },
            FileNode {
                name: "main.rs".to_string(),
                path: "src/main.rs".to_string(),
                is_dir: false,
                expanded: false,
                depth: 1,
            },
            FileNode {
                name: "cli.rs".to_string(),
                path: "src/cli.rs".to_string(),
                is_dir: false,
                expanded: false,
                depth: 1,
            },
            FileNode {
                name: "Cargo.toml".to_string(),
                path: "Cargo.toml".to_string(),
                is_dir: false,
                expanded: false,
                depth: 0,
            },
            FileNode {
                name: "README.md".to_string(),
                path: "README.md".to_string(),
                is_dir: false,
                expanded: false,
                depth: 0,
            },
        ];

        Self {
            is_running: true,
            input_mode: InputMode::Normal,
            active_panel: Panel::Chat,
            input: String::new(),
            chat_messages: vec![ChatMessage {
                author: "CodeAgent".to_string(),
                content: "👋 Welcome to CodeAgent v0.3.0\nI'm your AI-powered coding assistant.\nDescribe what you want to build or fix!".to_string(),
                timestamp: "just now".to_string(),
            }],
            output: vec!["System ready ✓".to_string()],
            editor: EditorState {
                content: "// Your code here".to_string(),
                cursor_x: 0,
                cursor_y: 0,
                scroll_x: 0,
                scroll_y: 0,
            },
            project_path,
            file_tree,
            selected_file: 0,
            command_palette_input: String::new(),
            animation_frame: 0,
        }
    }

    pub fn add_message(&mut self, author: String, content: String) {
        self.chat_messages.push(ChatMessage {
            author,
            content,
            timestamp: "now".to_string(),
        });
        if self.chat_messages.len() > 200 {
            self.chat_messages.remove(0);
        }
    }

    pub fn submit_input(&mut self) {
        if !self.input.is_empty() {
            self.add_message("You".to_string(), self.input.clone());
            let response = self.process_command(&self.input.clone());
            self.add_message("CodeAgent".to_string(), response);
            self.input.clear();
        }
    }

    pub fn process_command(&mut self, cmd: &str) -> String {
        match cmd.trim() {
            "help" => "Available commands:\n• /files - Show project files\n• /status - Show project status\n• /clear - Clear messages\n• /theme - Change theme".to_string(),
            "status" => {
                format!(
                    "📊 Project Status\nPath: {}\nModules: 20\nFeatures: 35+\nVersion: 0.3.0",
                    self.project_path
                )
            }
            "clear" => {
                self.chat_messages.clear();
                "Messages cleared ✓".to_string()
            }
            _ => {
                self.output.push(format!("Processing: {}", cmd));
                "✅ Command processed".to_string()
            }
        }
    }

    pub fn tick(&mut self) {
        self.animation_frame = self.animation_frame.wrapping_add(1);
    }

    pub fn get_spinner(&self) -> &str {
        match self.animation_frame % 4 {
            0 => "◇",
            1 => "◈",
            2 => "◆",
            _ => "◇",
        }
    }
}
