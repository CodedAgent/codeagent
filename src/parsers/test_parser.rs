use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub stack_trace: Option<String>,
    pub framework: String,
    pub file: String,
    pub line: Option<usize>,
}

pub struct TestParser;

impl TestParser {
    pub fn parse_jest_output(output: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        let pass_re = Regex::new(r"✓\s+(.+?)\s+\((\d+)\s*ms\)").unwrap();
        let fail_re = Regex::new(r"✕\s+(.+?)[\s\n]").unwrap();
        let _error_re = Regex::new(r"●\s+(.+?)[\s\n](.+?)(?:at\s+(.+?))?(?:$|\n)").unwrap();

        for cap in pass_re.captures_iter(output) {
            if let (Some(name), Some(duration)) = (cap.get(1), cap.get(2)) {
                let duration_ms = duration.as_str().parse::<u64>().unwrap_or(0);
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Passed,
                    duration_ms,
                    error_message: None,
                    stack_trace: None,
                    framework: "Jest".to_string(),
                    file: "test.js".to_string(),
                    line: None,
                });
            }
        }

        for cap in fail_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Failed,
                    duration_ms: 0,
                    error_message: Some("Test failed".to_string()),
                    stack_trace: None,
                    framework: "Jest".to_string(),
                    file: "test.js".to_string(),
                    line: None,
                });
            }
        }

        results
    }

    pub fn parse_pytest_output(output: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        let pass_re = Regex::new(r"([\w:]+)\s+PASSED\s*\[(\d+)%\]").unwrap();
        let fail_re = Regex::new(r"([\w:]+)\s+FAILED").unwrap();
        let error_re = Regex::new(r"(AssertionError|Error):\s+(.+?)(?:\n|$)").unwrap();

        for cap in pass_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Passed,
                    duration_ms: 0,
                    error_message: None,
                    stack_trace: None,
                    framework: "Pytest".to_string(),
                    file: "test.py".to_string(),
                    line: None,
                });
            }
        }

        for cap in fail_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                let error = error_re
                    .captures(output)
                    .and_then(|c| c.get(2).map(|m| m.as_str().to_string()));

                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Failed,
                    duration_ms: 0,
                    error_message: error,
                    stack_trace: None,
                    framework: "Pytest".to_string(),
                    file: "test.py".to_string(),
                    line: None,
                });
            }
        }

        results
    }

    pub fn parse_go_test_output(output: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        let pass_re = Regex::new(r"=== RUN\s+(Test\w+).*?--- PASS:\s+Test\w+\s+\((\d+\.\d+)s\)").unwrap();
        let fail_re = Regex::new(r"=== RUN\s+(Test\w+).*?--- FAIL:\s+Test\w+").unwrap();

        for cap in pass_re.captures_iter(output) {
            if let (Some(name), Some(duration)) = (cap.get(1), cap.get(2)) {
                let dur_secs = duration.as_str().parse::<f64>().unwrap_or(0.0);
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Passed,
                    duration_ms: (dur_secs * 1000.0) as u64,
                    error_message: None,
                    stack_trace: None,
                    framework: "Go".to_string(),
                    file: "test.go".to_string(),
                    line: None,
                });
            }
        }

        for cap in fail_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Failed,
                    duration_ms: 0,
                    error_message: Some("Test failed".to_string()),
                    stack_trace: None,
                    framework: "Go".to_string(),
                    file: "test.go".to_string(),
                    line: None,
                });
            }
        }

        results
    }

    pub fn parse_cargo_test_output(output: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        let pass_re = Regex::new(r"test\s+([\w:]+)\s+\.\.\.\s+ok").unwrap();
        let fail_re = Regex::new(r"test\s+([\w:]+)\s+\.\.\.\s+FAILED").unwrap();

        for cap in pass_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Passed,
                    duration_ms: 0,
                    error_message: None,
                    stack_trace: None,
                    framework: "Cargo".to_string(),
                    file: "test.rs".to_string(),
                    line: None,
                });
            }
        }

        for cap in fail_re.captures_iter(output) {
            if let Some(name) = cap.get(1) {
                results.push(TestResult {
                    name: name.as_str().to_string(),
                    status: TestStatus::Failed,
                    duration_ms: 0,
                    error_message: Some("Test failed".to_string()),
                    stack_trace: None,
                    framework: "Cargo".to_string(),
                    file: "test.rs".to_string(),
                    line: None,
                });
            }
        }

        results
    }

    pub fn summarize_results(results: &[TestResult]) -> TestSummary {
        let total = results.len();
        let passed = results.iter().filter(|r| r.status == TestStatus::Passed).count();
        let failed = results.iter().filter(|r| r.status == TestStatus::Failed).count();
        let skipped = results.iter().filter(|r| r.status == TestStatus::Skipped).count();

        TestSummary {
            total,
            passed,
            failed,
            skipped,
            success_rate: if total == 0 {
                100.0
            } else {
                (passed as f64 / total as f64) * 100.0
            },
        }
    }
}

#[derive(Debug)]
pub struct TestSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub success_rate: f64,
}
