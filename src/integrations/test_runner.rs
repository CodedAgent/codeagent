use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct TestRunner;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub output: String,
    pub exit_code: i32,
}

impl TestRunner {
    pub async fn run_all_tests(project_root: &Path) -> Result<TestResult> {
        tracing::info!("Running all tests");

        let test_frameworks = vec!["cargo test", "npm test", "pytest", "go test"];

        for framework_cmd in test_frameworks {
            let result = Self::run_command(framework_cmd, project_root);
            
            match result {
                Ok(output) => {
                    return Ok(TestResult {
                        passed: output.status.success(),
                        output: String::from_utf8_lossy(&output.stdout).to_string(),
                        exit_code: output.status.code().unwrap_or(-1),
                    });
                }
                Err(_) => continue,
            }
        }

        Ok(TestResult {
            passed: true,
            output: "No test framework detected".to_string(),
            exit_code: 0,
        })
    }

    pub async fn run_specific_test(project_root: &Path, test_name: &str) -> Result<TestResult> {
        tracing::info!("Running specific test: {}", test_name);

        let output = Command::new("cargo")
            .args(&["test", test_name])
            .current_dir(project_root)
            .output()?;

        Ok(TestResult {
            passed: output.status.success(),
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }

    fn run_command(cmd: &str, project_root: &Path) -> Result<std::process::Output> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let output = Command::new(parts[0])
            .args(&parts[1..])
            .current_dir(project_root)
            .output()?;

        Ok(output)
    }
}
