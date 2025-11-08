use regex::Regex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum LintSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintResult {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: LintSeverity,
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub tool: String,
}

pub struct LintParser;

impl LintParser {
    pub fn parse_eslint_output(output: &str) -> Vec<LintResult> {
        let mut results = Vec::new();

        let re = Regex::new(
            r"([\w/\-._]+)\s+(\d+):(\d+)\s+(error|warning|info)\s+(.+?)\s+([\w/\-]+)(?:\s+(.+?))?(?:\n|$)",
        )
        .unwrap();

        for cap in re.captures_iter(output) {
            if let (Some(file), Some(line), Some(col), Some(severity), Some(msg), Some(rule)) = (
                cap.get(1),
                cap.get(2),
                cap.get(3),
                cap.get(4),
                cap.get(5),
                cap.get(6),
            ) {
                let severity_enum = match severity.as_str() {
                    "error" => LintSeverity::Error,
                    "warning" => LintSeverity::Warning,
                    "info" => LintSeverity::Info,
                    _ => LintSeverity::Warning,
                };

                results.push(LintResult {
                    file: file.as_str().to_string(),
                    line: line.as_str().parse::<usize>().unwrap_or(0),
                    column: col.as_str().parse::<usize>().unwrap_or(0),
                    severity: severity_enum,
                    rule: rule.as_str().to_string(),
                    message: msg.as_str().to_string(),
                    suggestion: cap.get(7).map(|m| m.as_str().to_string()),
                    tool: "ESLint".to_string(),
                });
            }
        }

        results
    }

    pub fn parse_pylint_output(output: &str) -> Vec<LintResult> {
        let mut results = Vec::new();

        let re = Regex::new(
            r"([\w/\-._]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\((\w+)\)",
        )
        .unwrap();

        for cap in re.captures_iter(output) {
            if let (Some(file), Some(line), Some(col), Some(severity), Some(msg), Some(rule)) = (
                cap.get(1),
                cap.get(2),
                cap.get(3),
                cap.get(4),
                cap.get(5),
                cap.get(6),
            ) {
                let severity_enum = match severity.as_str() {
                    "E" => LintSeverity::Error,
                    "W" => LintSeverity::Warning,
                    "C" => LintSeverity::Info,
                    "R" => LintSeverity::Info,
                    _ => LintSeverity::Warning,
                };

                results.push(LintResult {
                    file: file.as_str().to_string(),
                    line: line.as_str().parse::<usize>().unwrap_or(0),
                    column: col.as_str().parse::<usize>().unwrap_or(0),
                    severity: severity_enum,
                    rule: rule.as_str().to_string(),
                    message: msg.as_str().to_string(),
                    suggestion: None,
                    tool: "Pylint".to_string(),
                });
            }
        }

        results
    }

    pub fn parse_clippy_output(output: &str) -> Vec<LintResult> {
        let mut results = Vec::new();

        let re = Regex::new(
            r"([\w/\-._]+):(\d+):(\d+):\s+(error|warning|note)\[(\w+)\]:\s+(.+?)(?:\n|$)",
        )
        .unwrap();

        for cap in re.captures_iter(output) {
            if let (Some(file), Some(line), Some(col), Some(severity), Some(rule), Some(msg)) = (
                cap.get(1),
                cap.get(2),
                cap.get(3),
                cap.get(4),
                cap.get(5),
                cap.get(6),
            ) {
                let severity_enum = match severity.as_str() {
                    "error" => LintSeverity::Error,
                    "warning" => LintSeverity::Warning,
                    "note" => LintSeverity::Info,
                    _ => LintSeverity::Warning,
                };

                results.push(LintResult {
                    file: file.as_str().to_string(),
                    line: line.as_str().parse::<usize>().unwrap_or(0),
                    column: col.as_str().parse::<usize>().unwrap_or(0),
                    severity: severity_enum,
                    rule: rule.as_str().to_string(),
                    message: msg.as_str().to_string(),
                    suggestion: None,
                    tool: "Clippy".to_string(),
                });
            }
        }

        results
    }

    pub fn summarize_results(results: &[LintResult]) -> LintSummary {
        let mut by_severity: HashMap<String, usize> = HashMap::new();
        let mut by_rule: HashMap<String, usize> = HashMap::new();
        let mut by_file: HashMap<String, usize> = HashMap::new();

        for result in results {
            let severity_str = match result.severity {
                LintSeverity::Info => "info".to_string(),
                LintSeverity::Warning => "warning".to_string(),
                LintSeverity::Error => "error".to_string(),
                LintSeverity::Critical => "critical".to_string(),
            };

            *by_severity.entry(severity_str).or_insert(0) += 1;
            *by_rule.entry(result.rule.clone()).or_insert(0) += 1;
            *by_file.entry(result.file.clone()).or_insert(0) += 1;
        }

        LintSummary {
            total_issues: results.len(),
            by_severity,
            by_rule,
            by_file,
        }
    }

    pub fn filter_by_severity(results: &[LintResult], min_severity: LintSeverity) -> Vec<LintResult> {
        results
            .iter()
            .filter(|r| r.severity >= min_severity)
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct LintSummary {
    pub total_issues: usize,
    pub by_severity: HashMap<String, usize>,
    pub by_rule: HashMap<String, usize>,
    pub by_file: HashMap<String, usize>,
}
