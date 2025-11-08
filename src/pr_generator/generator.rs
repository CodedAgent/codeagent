use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRMetadata {
    pub title: String,
    pub description: String,
    pub branch_name: String,
    pub base_branch: String,
    pub target_branch: String,
    pub reviewers: Vec<String>,
    pub labels: Vec<String>,
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDescription {
    pub summary: String,
    pub changes: Vec<String>,
    pub testing: Vec<String>,
    pub notes: Option<String>,
    pub breaking_changes: bool,
    pub related_issues: Vec<String>,
}

pub struct PRGenerator;

impl PRGenerator {
    pub fn generate_from_task(
        task_description: &str,
        changes_summary: &str,
        files_changed: Vec<String>,
    ) -> PRMetadata {
        let branch_name = Self::generate_branch_name(task_description);
        let title = Self::generate_title(task_description);
        
        PRMetadata {
            title,
            description: changes_summary.to_string(),
            branch_name,
            base_branch: "main".to_string(),
            target_branch: "main".to_string(),
            reviewers: vec![],
            labels: vec!["automated".to_string()],
            files_changed: files_changed.len(),
            lines_added: 0,
            lines_removed: 0,
        }
    }

    pub fn generate_description(
        summary: &str,
        changes: Vec<String>,
        testing: Vec<String>,
    ) -> PRDescription {
        PRDescription {
            summary: summary.to_string(),
            changes,
            testing,
            notes: None,
            breaking_changes: false,
            related_issues: vec![],
        }
    }

    pub fn generate_title(task: &str) -> String {
        if task.len() > 72 {
            format!("{}...", &task[..69])
        } else {
            task.to_string()
        }
    }

    pub fn generate_branch_name(task: &str) -> String {
        task
            .to_lowercase()
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join("-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
    }

    pub fn format_description_markdown(desc: &PRDescription) -> String {
        let mut md = format!("## {}\n\n", desc.summary);
        
        md.push_str("### Changes\n");
        for change in &desc.changes {
            md.push_str(&format!("- {}\n", change));
        }
        
        md.push_str("\n### Testing\n");
        for test in &desc.testing {
            md.push_str(&format!("- {}\n", test));
        }

        if let Some(notes) = &desc.notes {
            md.push_str(&format!("\n### Notes\n{}\n", notes));
        }

        if desc.breaking_changes {
            md.push_str("\n⚠️ **Breaking Changes**: This PR contains breaking changes.\n");
        }

        if !desc.related_issues.is_empty() {
            md.push_str("\n### Related Issues\n");
            for issue in &desc.related_issues {
                md.push_str(&format!("- {}\n", issue));
            }
        }

        md
    }

    pub fn format_metadata_markdown(meta: &PRMetadata) -> String {
        let mut md = format!("# {}\n\n", meta.title);
        
        md.push_str(&format!("**Branch**: `{}` → `{}`\n\n", meta.target_branch, meta.base_branch));
        md.push_str(&format!("{}\n\n", meta.description));
        
        md.push_str(&format!("**Files Changed**: {}\n", meta.files_changed));
        md.push_str(&format!("**Lines Added**: +{}\n", meta.lines_added));
        md.push_str(&format!("**Lines Removed**: -{}\n", meta.lines_removed));
        
        if !meta.labels.is_empty() {
            md.push_str(&format!("\n**Labels**: {}\n", meta.labels.join(", ")));
        }
        
        if !meta.reviewers.is_empty() {
            md.push_str(&format!("\n**Reviewers**: {}\n", meta.reviewers.join(", ")));
        }
        
        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_title() {
        let task = "Refactor deprecated API calls";
        assert_eq!(PRGenerator::generate_title(task), task);
    }

    #[test]
    fn test_generate_branch_name() {
        let task = "Fix authentication bug in login module";
        let branch = PRGenerator::generate_branch_name(task);
        assert_eq!(branch, "fix-authentication-bug");
    }
}
