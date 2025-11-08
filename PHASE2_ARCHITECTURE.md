# CodeAgent Phase 2 - Architecture & Design

## System Architecture Diagram

```
User Input (Natural Language Prompt)
    ↓
┌─────────────────────────────────────────┐
│        TaskDecomposer (Planner)         │
│  - Parse user intent                    │
│  - Generate execution steps             │
│  - Calculate dependencies               │
│  - Estimate complexity/duration         │
└─────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────┐
│       ExecutionContext Manager          │
│  - Track execution state                │
│  - Monitor progress                     │
│  - Manage rollback capabilities         │
└─────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────────────────┐
│                     Execution Engine                             │
├──────────────────────────────────────────────────────────────────┤
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐     │
│  │ File Modifier  │  │ Git Manager    │  │ Test Runner    │     │
│  └────────────────┘  └────────────────┘  └────────────────┘     │
│                                                                   │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐     │
│  │ Linter Runner  │  │ Config Loader  │  │ Ollama Client  │     │
│  └────────────────┘  └────────────────┘  └────────────────┘     │
└──────────────────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────────────────┐
│                       Analysis Phase                             │
├──────────────────────────────────────────────────────────────────┤
│  ┌────────────────────┐  ┌────────────────────┐                  │
│  │  TestParser        │  │  LintParser        │                  │
│  │  - Jest            │  │  - ESLint          │                  │
│  │  - Pytest          │  │  - Pylint          │                  │
│  │  - Go Test         │  │  - Clippy          │                  │
│  │  - Cargo Test      │  └────────────────────┘                  │
│  └────────────────────┘                                          │
└──────────────────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────────────────┐
│                  Error Correction Phase                          │
├──────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────┐                    │
│  │  ErrorAnalyzer                           │                    │
│  │  - Analyze failures                      │                    │
│  │  - Suggest fixes                         │                    │
│  │  - Correlate errors                      │                    │
│  │  - Determine retry strategy              │                    │
│  └──────────────────────────────────────────┘                    │
└──────────────────────────────────────────────────────────────────┘
    ↓
┌──────────────────────────────────────────────────────────────────┐
│                   Interactive Phase                              │
├──────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────┐                    │
│  │  InteractiveMode                         │                    │
│  │  - Display progress                      │                    │
│  │  - Show change previews                  │                    │
│  │  - Prompt for approval                   │                    │
│  │  - Handle user input                     │                    │
│  └──────────────────────────────────────────┘                    │
└──────────────────────────────────────────────────────────────────┘
    ↓
Final Decision (Approve/Reject/Rollback)
    ↓
Apply/Revert Changes & Commit
```

## Module Organization

### Core Modules (core/)

#### planner.rs (250+ lines)
**Structs**:
- `ExecutionStep`: Individual step definition
- `ExecutionPlan`: Complete execution blueprint
- `ExecutionContext`: Execution state manager
- `TaskDecomposer`: Step generator from prompts
- `Complexity`: Enum (Simple, Moderate, Complex, VeryComplex)
- `StepActionType`: Enum (Analyze, Modify, TestRun, LintCheck, Commit, Rollback)
- `StepResult`: Result of step execution

**Key Methods**:
```rust
TaskDecomposer::decompose_task(prompt: &str) -> Result<ExecutionPlan>
ExecutionContext::next_step() -> Option<&ExecutionStep>
ExecutionContext::can_proceed_to_next() -> bool
ExecutionContext::mark_step_complete(result: StepResult)
ExecutionContext::progress_percentage() -> u32
```

#### executor.rs (Enhanced)
- Uses ExecutionPlan from planner
- Manages step-by-step execution
- Handles dry-run mode
- Displays formatted output

### Parser Modules (parsers/)

#### test_parser.rs (180+ lines)
**Supported Frameworks**:
- Jest (JavaScript/TypeScript)
- Pytest (Python)
- Go Test (Go)
- Cargo Test (Rust)

**Output Model**:
```rust
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,        // Passed, Failed, Skipped, Pending
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub stack_trace: Option<String>,
    pub framework: String,
    pub file: String,
    pub line: Option<usize>,
}

pub struct TestSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub success_rate: f64,
}
```

#### lint_parser.rs (150+ lines)
**Supported Linters**:
- ESLint (JavaScript/TypeScript)
- Pylint (Python)
- Clippy (Rust)

**Output Model**:
```rust
pub struct LintResult {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: LintSeverity,    // Info, Warning, Error, Critical
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub tool: String,
}

pub struct LintSummary {
    pub total_issues: usize,
    pub by_severity: HashMap<String, usize>,
    pub by_rule: HashMap<String, usize>,
    pub by_file: HashMap<String, usize>,
}
```

### Error Correction Module (error_correction/)

#### analyzer.rs (120+ lines)
**Structs**:
```rust
pub struct FixSuggestion {
    pub error_pattern: String,
    pub suggested_fix: String,
    pub confidence: f64,           // 0.0-1.0
    pub file: Option<String>,
    pub line: Option<usize>,
    pub auto_fixable: bool,
}

pub struct RetryStrategy {
    pub retry_recommended: bool,
    pub apply_auto_fixes: bool,
    pub escalate_to_user: bool,
    pub suggested_delay_ms: u64,
}
```

**Methods**:
```rust
ErrorAnalyzer::analyze_test_failure(test: &TestResult) -> Option<FixSuggestion>
ErrorAnalyzer::analyze_lint_issue(lint: &LintResult) -> Option<FixSuggestion>
ErrorAnalyzer::correlate_errors(tests, lints) -> Vec<FixSuggestion>
ErrorAnalyzer::extract_common_patterns(failures) -> HashMap<String, usize>
ErrorAnalyzer::generate_retry_strategy(suggestions, attempt) -> RetryStrategy
```

### Interactive Module (interactive/)

#### mode.rs (100+ lines)
**Structs**:
```rust
pub enum UserChoice {
    Approve,
    Reject,
    ReviewChanges,
    RollBack,
    Continue,
    Abort,
}

pub struct InteractiveMode;
```

**Methods**:
```rust
InteractiveMode::prompt_for_approval(message: &str) -> UserChoice
InteractiveMode::prompt_yes_no(question: &str) -> bool
InteractiveMode::display_progress(current, total, status)
InteractiveMode::display_changes(before, after)
InteractiveMode::display_summary(title, items)
InteractiveMode::display_step_info(step_num, description, details)
```

### Configuration Module (config/)

#### yaml_parser.rs (150+ lines)
**Structs**:
```rust
pub struct YamlConfig {
    pub model_provider: String,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub interactive_mode: bool,
    pub auto_fix_enabled: bool,
    pub max_retry_attempts: u32,
    pub excluded_files: Vec<String>,
    pub excluded_dirs: Vec<String>,
    pub custom_tools: HashMap<String, CustomTool>,
}

pub struct CustomTool {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub enabled: bool,
}
```

**Methods**:
```rust
YamlConfig::load(path: &Path) -> Result<Self>
YamlConfig::from_yaml(content: &str) -> Result<Self>
YamlConfig::save(&self, path: &Path) -> Result<()>
YamlConfig::validate(&self) -> Result<()>
YamlConfig::should_exclude_file(file_path: &str) -> bool
YamlConfig::should_exclude_dir(dir_path: &str) -> bool
```

## Data Flow Examples

### Example 1: Multi-Step Task Execution
```
User Input
  ↓
"Refactor deprecated functions and verify with tests"
  ↓
TaskDecomposer::decompose_task()
  ↓
Generated ExecutionPlan:
  Step 1: Analyze (no deps)
  Step 2: Modify (depends on 1)
  Step 3: TestRun (depends on 2)
  ↓
ExecutionContext created with plan
  ↓
Foreach step:
  - Check dependencies met
  - Execute step
  - Record StepResult
  - Check progress (0-100%)
  - Option: Show progress to user
  ↓
All steps complete
  ↓
Summarize results
```

### Example 2: Error Recovery Flow
```
TestResult (FAILED)
  ↓
ErrorAnalyzer::analyze_test_failure()
  ↓
FixSuggestion generated:
  - Pattern: "assertion_error"
  - Suggestion: "Check logic"
  - Confidence: 0.7
  ↓
ErrorAnalyzer::generate_retry_strategy()
  ↓
RetryStrategy:
  - retry: true
  - apply_fixes: false
  - escalate: true
  ↓
InteractiveMode::prompt_for_approval()
  ↓
User decision → Apply/Reject/Review
```

### Example 3: Lint Issue Auto-Fix
```
LintResult:
  - File: "main.rs"
  - Issue: "unused_variable"
  - Suggestion: "Remove variable"
  ↓
ErrorAnalyzer::analyze_lint_issue()
  ↓
FixSuggestion:
  - Pattern: "unused_code"
  - Fix: "Remove the unused variable"
  - Confidence: 0.9
  - Auto-fixable: true
  ↓
YamlConfig::auto_fix_enabled check
  ↓
IF enabled: Apply fix automatically
IF disabled: Prompt user for confirmation
```

## Execution Flow Diagram

```
Start
  ↓
Load Config (YamlConfig)
  ↓
Decompose Task (TaskDecomposer)
  ↓
Create ExecutionContext
  ↓
LOOP: While more steps
  ↓
  Get current step
  ↓
  Check dependencies
  ↓
  YES: Execute step
  │    ↓
  │    Run action (Modify/Test/Lint)
  │    ↓
  │    Parse output (TestParser/LintParser)
  │    ↓
  │    IF errors: ErrorAnalyzer
  │    │    ↓
  │    │    Generate suggestions
  │    │    ↓
  │    │    Determine retry strategy
  │    │    ↓
  │    │    IF interactive: Prompt user
  │    │    ELSE: Auto-fix if enabled
  │    ↓
  │    Mark step complete
  │
  NO: Wait for dependencies
  ↓
  Update progress
  ↓
  Show to user if interactive mode
  ↓
  Next step
  ↓
All steps done
  ↓
Display summary
  ↓
IF dry-run: Stop here
ELSE: Commit changes
  ↓
End
```

## Integration Points

### TestParser Integration
```
Test Runner Output
  ↓
TestParser::parse_X_output()
  ↓
Vec<TestResult>
  ↓
ErrorAnalyzer (if failures)
  ↓
FixSuggestion
  ↓
InteractiveMode (if enabled)
```

### LintParser Integration
```
Linter Output
  ↓
LintParser::parse_X_output()
  ↓
Vec<LintResult>
  ↓
ErrorAnalyzer
  ↓
FixSuggestion
  ↓
Auto-fix or User Decision
```

### Config Integration
```
.codeagent.yml
  ↓
YamlConfig::load()
  ↓
Validate config
  ↓
Apply settings:
  - Model provider
  - Interactive mode
  - Auto-fix settings
  - Excluded files/dirs
  - Custom tools
```

## Design Patterns Used

### 1. Strategy Pattern
- Different test/lint parsers implement same interface
- Easy to add new parsers

### 2. State Pattern
- ExecutionContext maintains execution state
- Tracks completed steps and progress

### 3. Builder Pattern
- TaskDecomposer builds ExecutionPlan step by step
- ExecutionContext builds results incrementally

### 4. Template Method
- ErrorAnalyzer follows pattern for analysis
- Common steps, customizable details

### 5. Observer Pattern (Foundation)
- InteractiveMode listens to execution events
- Can extend for webhooks/notifications (Phase 3)

## Error Handling Strategy

### Multi-Level Error Recovery
```
Level 1: Inline Errors
  - File read errors → Skip file, log warning
  - Regex parse errors → Fall back to generic output

Level 2: Step Errors
  - Test failure → Analyze and suggest fixes
  - Linter issue → Generate recommendations

Level 3: Execution Errors
  - Multiple failures → Escalate to user
  - Critical errors → Rollback changes

Level 4: User Override
  - User can reject suggestions
  - User can force rollback
  - User can continue despite errors
```

## Performance Considerations

### Current (Phase 2)
- Sequential execution
- Regex-based parsing
- No caching

### Optimization (Phase 3)
- Parallel step execution
- Smart dependency handling
- Result caching
- Incremental analysis

## Configuration Schema

```yaml
# Model configuration
model_provider: "ollama|openai|anthropic"
ollama_base_url: "http://localhost:11434"
ollama_model: "mistral|neural-chat|..."
openai_api_key: "sk-..."
anthropic_api_key: "sk-ant-..."

# Execution settings
interactive_mode: true|false
auto_fix_enabled: true|false
max_retry_attempts: 3
dry_run_by_default: false

# File filtering
excluded_files:
  - "*.min.js"
  - "*.lock"
excluded_dirs:
  - "node_modules"
  - "target"

# Custom tools
custom_tools:
  tool_name:
    name: "Display Name"
    command: "command"
    args: ["--flag"]
    enabled: true
```

## Testing Strategy for Phase 2

### Unit Tests
- TaskDecomposer: Generate correct plans
- TestParser: Correctly parse each framework
- LintParser: Correctly parse each linter
- ErrorAnalyzer: Suggest appropriate fixes
- YamlConfig: Load and validate configs

### Integration Tests
- Multi-step execution flow
- Error recovery workflows
- Interactive mode dialogs
- Config application

### Example Test Cases
```rust
#[test]
fn test_jest_parser_extracts_test_duration() {
    let output = "✓ should pass (123ms)";
    let results = TestParser::parse_jest_output(output);
    assert_eq!(results[0].duration_ms, 123);
}

#[test]
fn test_error_analyzer_detects_assertion_error() {
    let test = TestResult { error_message: Some("AssertionError".into()), .. };
    let suggestion = ErrorAnalyzer::analyze_test_failure(&test);
    assert!(suggestion.is_some());
    assert_eq!(suggestion.unwrap().confidence, 0.7);
}

#[test]
fn test_execution_plan_tracks_dependencies() {
    let steps = vec![step1, step2_depends_on_1];
    let can_run = context.can_proceed_to_next();
    assert!(!can_run); // Can't run step 2 yet
}
```

## Future Enhancements

### Phase 3 Candidates
- LLM-based fix generation
- Pull request creation
- TUI interface
- Parallel execution
- Plugin system

### Architecture Readiness
- ✅ Pluggable parsers
- ✅ Modular error handling
- ✅ Configuration system
- ✅ Extensible interactive mode
