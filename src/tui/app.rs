use ratatui::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Chat,
    Editor,
    Terminal,
    Settings,
}

pub struct LineNumber {
    pub number: usize,
    pub modified: bool,
}

pub struct CodeLine {
    pub content: String,
    pub syntax_type: SyntaxType,
    pub breakpoint: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SyntaxType {
    Normal,
    Keyword,
    String,
    Comment,
    Function,
}

pub struct EditorTab {
    pub name: String,
    pub path: String,
    pub content: Vec<CodeLine>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub modified: bool,
}

pub struct App {
    pub is_running: bool,
    pub input_mode: InputMode,
    pub active_tab: Tab,
    pub input: String,
    pub command_history: VecDeque<String>,
    pub command_index: usize,
    pub search_query: String,
    pub search_results: Vec<(usize, usize)>,
    pub current_result: usize,
    
    pub chat_messages: Vec<ChatMessage>,
    pub editor_tabs: Vec<EditorTab>,
    pub active_editor_tab: usize,
    pub terminal_output: VecDeque<TerminalLine>,
    pub status_bar: StatusBar,
    pub file_tree: Vec<FileTreeNode>,
    pub selected_file: usize,
    pub project_path: String,
    pub animation_frame: u64,
    pub notification: Option<Notification>,
    pub show_help: bool,
    pub ollama_model: String,
    pub available_models: Vec<String>,
    pub is_loading: bool,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub author: String,
    pub content: String,
    pub timestamp: String,
    pub ai_streaming: bool,
}

#[derive(Clone)]
pub struct TerminalLine {
    pub content: String,
    pub style: LineStyle,
}

#[derive(Clone, Copy)]
pub enum LineStyle {
    Normal,
    Error,
    Success,
    Warning,
}

pub struct StatusBar {
    pub file_info: String,
    pub position: String,
    pub mode: String,
    pub git_branch: String,
    pub diagnostics: String,
}

#[derive(Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub depth: usize,
    pub children: Vec<FileTreeNode>,
}

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub level: NotificationLevel,
    pub timeout: u64,
}

#[derive(Clone, Copy)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

impl App {
    pub fn new(project_path: String) -> Self {
        let editor_tabs = vec![EditorTab {
            name: "main.rs".to_string(),
            path: "src/main.rs".to_string(),
            content: vec![
                CodeLine {
                    content: "fn main() {".to_string(),
                    syntax_type: SyntaxType::Keyword,
                    breakpoint: false,
                },
                CodeLine {
                    content: "    println!(\"Hello, CodeAgent!\");".to_string(),
                    syntax_type: SyntaxType::Normal,
                    breakpoint: false,
                },
                CodeLine {
                    content: "}".to_string(),
                    syntax_type: SyntaxType::Keyword,
                    breakpoint: false,
                },
            ],
            cursor_x: 0,
            cursor_y: 0,
            scroll_x: 0,
            scroll_y: 0,
            modified: false,
        }];

        Self {
            is_running: true,
            input_mode: InputMode::Normal,
            active_tab: Tab::Chat,
            input: String::new(),
            command_history: VecDeque::new(),
            command_index: 0,
            search_query: String::new(),
            search_results: Vec::new(),
            current_result: 0,
            
            chat_messages: vec![ChatMessage {
                author: "CodeAgent".to_string(),
                content: "🚀 CodeAgent v0.3.0 Pro Edition\nYour AI-powered development environment.\nPress '?' for help or describe your task.".to_string(),
                timestamp: "now".to_string(),
                ai_streaming: false,
            }],
            editor_tabs,
            active_editor_tab: 0,
            terminal_output: VecDeque::new(),
            status_bar: StatusBar {
                file_info: "src/main.rs | Rust | UTF-8 | LF".to_string(),
                position: "1:1".to_string(),
                mode: "NORMAL".to_string(),
                git_branch: "main".to_string(),
                diagnostics: "✓ 0 errors".to_string(),
            },
            file_tree: create_file_tree(),
            selected_file: 0,
            project_path,
            animation_frame: 0,
            notification: None,
            show_help: false,
            ollama_model: String::new(),
            available_models: Vec::new(),
            is_loading: false,
        }
    }

    pub fn add_message(&mut self, author: String, content: String) {
        self.chat_messages.push(ChatMessage {
            author,
            content,
            timestamp: "now".to_string(),
            ai_streaming: false,
        });
    }

    pub fn add_notification(&mut self, title: String, message: String, level: NotificationLevel) {
        self.notification = Some(Notification {
            title,
            message,
            level,
            timeout: 300,
        });
    }

    pub fn submit_input(&mut self) {
        if !self.input.is_empty() {
            self.command_history.push_front(self.input.clone());
            if self.command_history.len() > 100 {
                self.command_history.pop_back();
            }
            
            self.add_message("You".to_string(), self.input.clone());
            let response = self.process_command(&self.input.clone());
            self.add_message("CodeAgent".to_string(), response);
            self.input.clear();
            self.command_index = 0;
        }
    }

    pub fn process_command(&mut self, cmd: &str) -> String {
        match cmd.trim() {
            "help" => self.get_help_text(),
            "status" => format!("📊 Status:\n▪ Modules: 20\n▪ Features: 35+\n▪ Lines: 2,600+\n▪ Project: {}", self.project_path),
            "clear" => {
                self.chat_messages.clear();
                "✓ Cleared".to_string()
            }
            _ => {
                self.add_notification(
                    "Processing".to_string(),
                    "Analyzing your request...".to_string(),
                    NotificationLevel::Info,
                );
                "✓ Processing your request...".to_string()
            }
        }
    }

    fn get_help_text(&self) -> String {
        "┌─ CodeAgent Help ─────────────────────┐\n\
         │ i       - Insert mode               │\n\
         │ :       - Command palette           │\n\
         │ /       - Search                    │\n\
         │ Tab     - Switch tabs               │\n\
         │ Ctrl-s  - Save file                 │\n\
         │ Ctrl-p  - Find file                 │\n\
         │ ?       - Toggle help               │\n\
         │ q       - Quit                      │\n\
         └─────────────────────────────────────┘".to_string()
    }

    pub fn tick(&mut self) {
        self.animation_frame = self.animation_frame.wrapping_add(1);
        if let Some(ref mut notif) = self.notification {
            if notif.timeout > 0 {
                notif.timeout -= 1;
            } else {
                self.notification = None;
            }
        }
    }

    pub fn cycle_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Chat => Tab::Editor,
            Tab::Editor => Tab::Terminal,
            Tab::Terminal => Tab::Settings,
            Tab::Settings => Tab::Chat,
        };
    }

    pub fn get_spinner(&self) -> &str {
        match (self.animation_frame / 4) % 4 {
            0 => "▁",
            1 => "▂",
            2 => "▃",
            _ => "▄",
        }
    }
}

fn create_file_tree() -> Vec<FileTreeNode> {
    vec![
        FileTreeNode {
            name: "📁 src/".to_string(),
            path: "src".to_string(),
            is_dir: true,
            expanded: true,
            depth: 0,
            children: vec![
                FileTreeNode {
                    name: "📄 main.rs".to_string(),
                    path: "src/main.rs".to_string(),
                    is_dir: false,
                    expanded: false,
                    depth: 1,
                    children: vec![],
                },
                FileTreeNode {
                    name: "📄 cli.rs".to_string(),
                    path: "src/cli.rs".to_string(),
                    is_dir: false,
                    expanded: false,
                    depth: 1,
                    children: vec![],
                },
            ],
        },
        FileTreeNode {
            name: "📄 Cargo.toml".to_string(),
            path: "Cargo.toml".to_string(),
            is_dir: false,
            expanded: false,
            depth: 0,
            children: vec![],
        },
    ]
}
