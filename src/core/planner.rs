use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub id: String,
    pub description: String,
    pub action_type: StepActionType,
    pub target_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub estimated_complexity: Complexity,
    pub rollback_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepActionType {
    Analyze,
    Modify,
    TestRun,
    LintCheck,
    Commit,
    Rollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Complexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
    pub total_complexity: Complexity,
    pub estimated_duration_ms: u64,
    pub requires_user_approval: bool,
    pub rollback_available: bool,
}

pub struct TaskDecomposer;

impl TaskDecomposer {
    pub fn decompose_task(prompt: &str) -> Result<ExecutionPlan> {
        let steps = Self::analyze_prompt(prompt);
        let plan = Self::sequence_steps(steps)?;
        Ok(plan)
    }

    fn analyze_prompt(prompt: &str) -> Vec<ExecutionStep> {
        let prompt_lower = prompt.to_lowercase();
        let mut steps = Vec::new();

        steps.push(ExecutionStep {
            id: "analyze_0".to_string(),
            description: "Analyze project structure and context".to_string(),
            action_type: StepActionType::Analyze,
            target_files: vec![],
            dependencies: vec![],
            estimated_complexity: Complexity::Simple,
            rollback_enabled: false,
        });

        if prompt_lower.contains("refactor") || prompt_lower.contains("replace") {
            steps.push(ExecutionStep {
                id: "modify_1".to_string(),
                description: "Apply code modifications".to_string(),
                action_type: StepActionType::Modify,
                target_files: vec![],
                dependencies: vec!["analyze_0".to_string()],
                estimated_complexity: Complexity::Moderate,
                rollback_enabled: true,
            });
        }

        if prompt_lower.contains("test") || prompt_lower.contains("verify") {
            steps.push(ExecutionStep {
                id: "test_2".to_string(),
                description: "Run tests to verify changes".to_string(),
                action_type: StepActionType::TestRun,
                target_files: vec![],
                dependencies: vec!["modify_1".to_string()],
                estimated_complexity: Complexity::Moderate,
                rollback_enabled: false,
            });
        }

        if prompt_lower.contains("lint") || prompt_lower.contains("quality") {
            steps.push(ExecutionStep {
                id: "lint_3".to_string(),
                description: "Run linter checks".to_string(),
                action_type: StepActionType::LintCheck,
                target_files: vec![],
                dependencies: vec!["modify_1".to_string()],
                estimated_complexity: Complexity::Simple,
                rollback_enabled: false,
            });
        }

        steps
    }

    fn sequence_steps(steps: Vec<ExecutionStep>) -> Result<ExecutionPlan> {
        let mut total_complexity = Complexity::Simple;

        for step in &steps {
            if step.estimated_complexity > total_complexity {
                total_complexity = step.estimated_complexity;
            }
        }

        let step_count = steps.len() as u64;
        let estimated_duration_ms = match total_complexity {
            Complexity::Simple => 1000 + (step_count * 500),
            Complexity::Moderate => 2000 + (step_count * 1000),
            Complexity::Complex => 5000 + (step_count * 2000),
            Complexity::VeryComplex => 10000 + (step_count * 5000),
        };

        Ok(ExecutionPlan {
            steps,
            total_complexity,
            estimated_duration_ms,
            requires_user_approval: total_complexity >= Complexity::Complex,
            rollback_available: true,
        })
    }
}

pub struct ExecutionContext {
    pub plan: ExecutionPlan,
    pub completed_steps: HashMap<String, StepResult>,
    pub current_step_index: usize,
    pub is_dry_run: bool,
    pub changes_staged: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StepResult {
    #[allow(dead_code)]
    pub step_id: String,
    #[allow(dead_code)]
    pub success: bool,
    #[allow(dead_code)]
    pub output: String,
    #[allow(dead_code)]
    pub duration_ms: u64,
    #[allow(dead_code)]
    pub error_message: Option<String>,
}

impl ExecutionContext {
    pub fn new(plan: ExecutionPlan, is_dry_run: bool) -> Self {
        ExecutionContext {
            plan,
            completed_steps: HashMap::new(),
            current_step_index: 0,
            is_dry_run,
            changes_staged: Vec::new(),
        }
    }

    pub fn next_step(&self) -> Option<&ExecutionStep> {
        if self.current_step_index < self.plan.steps.len() {
            Some(&self.plan.steps[self.current_step_index])
        } else {
            None
        }
    }

    pub fn mark_step_complete(&mut self, result: StepResult) {
        self.completed_steps.insert(result.step_id.clone(), result);
        self.current_step_index += 1;
    }

    pub fn can_proceed_to_next(&self) -> bool {
        if let Some(step) = self.next_step() {
            step.dependencies
                .iter()
                .all(|dep| self.completed_steps.contains_key(dep))
        } else {
            false
        }
    }

    pub fn progress_percentage(&self) -> u32 {
        if self.plan.steps.is_empty() {
            return 0;
        }
        ((self.current_step_index as f32 / self.plan.steps.len() as f32) * 100.0) as u32
    }

    pub fn rollback_enabled(&self) -> bool {
        self.plan.rollback_available && !self.changes_staged.is_empty()
    }
}
