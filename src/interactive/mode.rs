use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
pub enum UserChoice {
    Approve,
    Reject,
    ReviewChanges,
    RollBack,
    Continue,
    Abort,
}

pub struct InteractiveMode;

impl InteractiveMode {
    pub fn prompt_for_approval(message: &str) -> UserChoice {
        println!("\n{}", message);
        println!("\nOptions:");
        println!("  [a] Approve and continue");
        println!("  [r] Reject and rollback");
        println!("  [v] Review changes");
        println!("  [q] Quit");
        
        print!("\nYour choice: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        
        match line.trim().to_lowercase().as_str() {
            "a" | "approve" => UserChoice::Approve,
            "r" | "reject" => UserChoice::Reject,
            "v" | "review" => UserChoice::ReviewChanges,
            "q" | "quit" => UserChoice::Abort,
            _ => UserChoice::Continue,
        }
    }

    pub fn prompt_yes_no(question: &str) -> bool {
        println!("\n{} [y/n]", question);
        print!("> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        
        matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
    }

    pub fn display_progress(current: usize, total: usize, status: &str) {
        let percentage = (if total == 0 {
            0.0
        } else {
            (current as f64 / total as f64) * 100.0
        }) as u32;
        
        let bar_length = 30;
        let filled = (bar_length * percentage) / 100;
        let empty = bar_length - filled;

        print!("\r[");
        for _ in 0..filled {
            print!("=");
        }
        for _ in 0..empty {
            print!("-");
        }
        print!("] {}% - {}", percentage, status);
        io::Write::flush(&mut io::stdout()).unwrap();
    }

    pub fn display_changes(before: &str, after: &str) {
        println!("\n📝 Changes Preview:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("BEFORE:");
        println!("{}", before);
        println!("\nAFTER:");
        println!("{}", after);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    pub fn display_summary(title: &str, items: &[(String, String)]) {
        println!("\n📊 {}", title);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        for (key, value) in items {
            println!("  {:<30} {}", key, value);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    pub fn display_step_info(step_num: usize, description: &str, details: &[&str]) {
        println!("\n▶ Step {}: {}", step_num, description);
        for detail in details {
            println!("  • {}", detail);
        }
    }
}
