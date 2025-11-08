use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[command(
    name = "CodeAgent",
    version = "0.3.0",
    about = "Interactive AI-powered code assistant",
    long_about = "CodeAgent is an interactive coding assistant that helps you write, debug, and improve code using AI and intelligent analysis."
)]
pub struct Cli {
    #[arg(value_name = "FILE", help = "Optional file or directory to load initially")]
    pub path: Option<String>,
}

pub struct InteractiveSession {
    pub project_path: String,
    pub is_active: bool,
}

impl InteractiveSession {
    pub fn new(path: Option<String>) -> Self {
        let project_path = path.unwrap_or_else(|| ".".to_string());
        InteractiveSession {
            project_path,
            is_active: true,
        }
    }

    pub fn run(&mut self) {
        self.print_welcome();
        
        while self.is_active {
            self.print_prompt();
            let mut input = String::new();
            
            if io::stdin().read_line(&mut input).is_err() {
                self.is_active = false;
                continue;
            }

            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }

            match input {
                "help" => self.print_help(),
                "exit" | "quit" => {
                    println!("\nGoodbye!");
                    self.is_active = false;
                }
                "clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                    io::stdout().flush().ok();
                }
                "status" => self.print_status(),
                _ => self.process_command(input),
            }
        }
    }

    fn print_welcome(&self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║         CodeAgent v0.3.0 - Interactive Code Editor         ║");
        println!("║     AI-Powered Coding Assistant at Your Fingertips         ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");
        println!("Project: {}", self.project_path);
        println!("Type 'help' for available commands.\n");
    }

    fn print_prompt(&self) {
        print!("🤖 > ");
        io::stdout().flush().ok();
    }

    fn print_help(&self) {
        println!("\n╭─ Available Commands ─────────────────────────────────────────╮");
        println!("│ help       Show this help message                           │");
        println!("│ status     Show project status                              │");
        println!("│ clear      Clear the screen                                 │");
        println!("│ exit/quit  Exit CodeAgent                                   │");
        println!("│                                                             │");
        println!("│ Ask anything: describe what you want to build or fix        │");
        println!("│ Examples:                                                   │");
        println!("│   - 'fix the bug in login.rs'                               │");
        println!("│   - 'add error handling to utils'                           │");
        println!("│   - 'refactor the payment module'                           │");
        println!("│   - 'write unit tests for database.rs'                      │");
        println!("╰─────────────────────────────────────────────────────────────╯\n");
    }

    fn print_status(&self) {
        println!("\n╭─ Project Status ─────────────────────────────────────────────╮");
        println!("│ Project: {:<48} │", self.project_path);
        println!("│ Features: Phase 1, 2, 3 (35+ total)                         │");
        println!("│ Modules: 20                                                 │");
        println!("│ Status: Ready to assist                                     │");
        println!("╰─────────────────────────────────────────────────────────────╯\n");
    }

    fn process_command(&self, input: &str) {
        println!("\n📝 Processing: {}", input);
        println!("⏳ Analyzing your request...\n");
        println!("✅ Ready to help!\n");
    }
}
