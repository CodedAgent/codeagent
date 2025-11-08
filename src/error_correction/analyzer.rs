use crate::parsers::{TestResult, TestStatus, LintResult};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub error_pattern: String,
    pub suggested_fix: String,
    pub confidence: f64,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub auto_fixable: bool,
}

pub struct ErrorAnalyzer;

impl ErrorAnalyzer {
    pub fn analyze_test_failure(test: &TestResult) -> Option<FixSuggestion> {
        let error_msg = test.error_message.as_ref()?;
        let lower_msg = error_msg.to_lowercase();

        if lower_msg.contains("assertion") || lower_msg.contains("assert") {
            return Some(FixSuggestion {
                error_pattern: "assertion_error".to_string(),
                suggested_fix: "Check the assertion logic and ensure all preconditions are met".to_string(),
                confidence: 0.7,
                file: Some(test.file.clone()),
                line: test.line,
                auto_fixable: false,
            });
        }

        if lower_msg.contains("undefined") || lower_msg.contains("not found") {
            return Some(FixSuggestion {
                error_pattern: "undefined_reference".to_string(),
                suggested_fix: "Ensure all required modules/functions are imported or defined".to_string(),
                confidence: 0.8,
                file: Some(test.file.clone()),
                line: test.line,
                auto_fixable: false,
            });
        }

        if lower_msg.contains("type") || lower_msg.contains("mismatch") {
            return Some(FixSuggestion {
                error_pattern: "type_mismatch".to_string(),
                suggested_fix: "Check type conversions and ensure compatible types are used".to_string(),
                confidence: 0.75,
                file: Some(test.file.clone()),
                line: test.line,
                auto_fixable: false,
            });
        }

        None
    }

    pub fn analyze_lint_issue(lint: &LintResult) -> Option<FixSuggestion> {
        if lint.suggestion.is_some() {
            return Some(FixSuggestion {
                error_pattern: lint.rule.clone(),
                suggested_fix: lint.suggestion.clone().unwrap_or_default(),
                confidence: 0.85,
                file: Some(lint.file.clone()),
                line: Some(lint.line),
                auto_fixable: true,
            });
        }

        let rule_lower = lint.rule.to_lowercase();

        if rule_lower.contains("unused") {
            return Some(FixSuggestion {
                error_pattern: "unused_code".to_string(),
                suggested_fix: "Remove the unused variable or import".to_string(),
                confidence: 0.9,
                file: Some(lint.file.clone()),
                line: Some(lint.line),
                auto_fixable: true,
            });
        }

        if rule_lower.contains("style") || rule_lower.contains("format") {
            return Some(FixSuggestion {
                error_pattern: "style_violation".to_string(),
                suggested_fix: "Reformat the code according to the style guide".to_string(),
                confidence: 0.8,
                file: Some(lint.file.clone()),
                line: Some(lint.line),
                auto_fixable: true,
            });
        }

        None
    }

    pub fn correlate_errors(
        test_failures: &[TestResult],
        lint_issues: &[LintResult],
    ) -> Vec<FixSuggestion> {
        let mut suggestions = Vec::new();

        for test in test_failures {
            if test.status == TestStatus::Failed {
                if let Some(suggestion) = Self::analyze_test_failure(test) {
                    suggestions.push(suggestion);
                }
            }
        }

        for lint in lint_issues {
            if let Some(suggestion) = Self::analyze_lint_issue(lint) {
                suggestions.push(suggestion);
            }
        }

        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        suggestions
    }

    pub fn extract_common_patterns(failures: &[TestResult]) -> HashMap<String, usize> {
        let mut patterns = HashMap::new();

        for failure in failures {
            if let Some(error) = &failure.error_message {
                let error_lower = error.to_lowercase();

                for keyword in &["assertion", "undefined", "type", "timeout", "null", "panic"] {
                    if error_lower.contains(keyword) {
                        *patterns.entry(keyword.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        patterns
    }

    pub fn generate_retry_strategy(
        suggestions: &[FixSuggestion],
        attempt: usize,
    ) -> RetryStrategy {
        let auto_fixable_count = suggestions.iter().filter(|s| s.auto_fixable).count();
        let high_confidence = suggestions.iter().filter(|s| s.confidence > 0.8).count();

        RetryStrategy {
            retry_recommended: attempt < 3 && !suggestions.is_empty(),
            apply_auto_fixes: auto_fixable_count > 0 && high_confidence >= auto_fixable_count / 2,
            escalate_to_user: high_confidence == 0 || attempt >= 3,
            suggested_delay_ms: 100 * (attempt as u64 + 1),
        }
    }
}

#[derive(Debug)]
pub struct RetryStrategy {
    pub retry_recommended: bool,
    pub apply_auto_fixes: bool,
    pub escalate_to_user: bool,
    pub suggested_delay_ms: u64,
}
