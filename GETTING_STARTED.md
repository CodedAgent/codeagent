# Getting Started with CodeAgent

## What is CodeAgent?

CodeAgent is a powerful, AI-driven command-line agent that understands your entire project context and autonomously executes complex, multi-step engineering tasks. Built in Rust with full async support.

## Quick Setup (2 minutes)

### Prerequisites
- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs))
- **Git** (for version control features)
- **Ollama** (optional, for local LLM support)

### Verify Your Setup

```bash
cd /Users/teck/Desktop/CodeAgent
./verify-setup.sh
```

All checks should pass ✅

### First Command

```bash
./target/release/codeagent --help
```

You should see:
```
A powerful CLI agent that understands your project context and 
autonomously executes complex, multi-step engineering tasks.

Usage: codeagent <COMMAND>

Commands:
  run   Run a task with a natural language prompt
  init  Initialize a new CodeAgent project
  help  Print this message or the help of the given subcommand(s)
```

## Basic Usage

### 1. Initialize a Project

```bash
./target/release/codeagent init /path/to/your/project
```

This creates a `.codeagent.yml` configuration file with:
- Default model provider (Ollama)
- Base URL settings
- Model selection

### 2. Run Your First Task

```bash
./target/release/codeagent run "Find all TODO comments in the codebase"
```

Output shows:
- Model provider configured
- Execution plan
- Files analyzed
- Ready status

### 3. Preview Changes (Dry Run)

```bash
./target/release/codeagent run "Refactor function names" --dry-run
```

The `--dry-run` flag previews changes without applying them.

## Example Use Cases

### Find and Report

```bash
# Find all imports
codeagent run "Find all import statements"

# Locate deprecated functions
codeagent run "Find all uses of deprecated_function"

# Search for patterns
codeagent run "Find all TODO and FIXME comments"
```

### Modify and Verify

```bash
# Replace patterns (safe with dry-run first!)
codeagent run "Replace all var with const" --dry-run
codeagent run "Replace all var with const"

# Update dependencies
codeagent run "Update all npm dependencies to latest versions"
```

### Analyze and Report

```bash
# Code quality
codeagent run "Find all functions longer than 50 lines"

# Security
codeagent run "Find all hardcoded secrets or API keys"

# Structure
codeagent run "List all public functions and their purposes"
```

## Configuration

### .codeagent.yml

The configuration file is created automatically during `init`:

```yaml
# CodeAgent Configuration
model_provider: ollama
ollama_base_url: "http://localhost:11434"
ollama_model: "mistral"
```

**Supported Model Providers:**
- `ollama` - Local models via Ollama
- `openai` - GPT-4o via OpenAI API (Phase 3)
- `anthropic` - Claude 3.5 via Anthropic (Phase 3)

## Available Commands

### `codeagent run <PROMPT>`

Execute a task with a natural language prompt.

**Options:**
- `--dry-run` - Preview changes without applying
- `--help` - Show command help

**Examples:**
```bash
codeagent run "Find all hardcoded URLs"
codeagent run "Refactor variable names" --dry-run
codeagent run "Create test file" --dry-run
```

### `codeagent init <PATH>`

Initialize CodeAgent in a project directory.

**Arguments:**
- `<PATH>` - Project directory (default: current directory)

**Examples:**
```bash
codeagent init .
codeagent init /home/user/my-project
codeagent init ~/Desktop/app
```

## Features Available in Phase 1

✅ **CLI Interface**
- Natural language task prompts
- Dry-run preview mode
- Help documentation

✅ **Project Analysis**
- File discovery and categorization
- Language detection
- Project structure mapping

✅ **Git Integration**
- Automatic staging of changes
- Commit with auto-generated messages
- Status checking

✅ **Test Support**
- Automatic test framework detection
- Multi-framework support (Jest, Pytest, Go, Cargo)
- Test result parsing

✅ **Search & Replace**
- Regex pattern matching
- Keyword search
- Line-accurate results

✅ **Local LLM Support**
- Ollama integration
- Async API client
- Health checking

## Troubleshooting

### "Command not found: codeagent"

Make sure you're using the full path:
```bash
./target/release/codeagent --help
```

Or add it to your PATH:
```bash
export PATH="$PATH:/Users/teck/Desktop/CodeAgent/target/release"
codeagent --help
```

### "Ollama connection refused"

Make sure Ollama is running:
```bash
# Start Ollama (if installed)
ollama serve

# Or check if it's already running
curl http://localhost:11434/api/tags
```

### Build errors

Clean and rebuild:
```bash
cargo clean
cargo build --release
```

## Development & Contributing

### Building from Source

```bash
# Clone or navigate to project
cd /Users/teck/Desktop/CodeAgent

# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Checking Code

```bash
cargo check          # Type checking
cargo clippy         # Lint warnings
cargo fmt -- --check # Format checking
```

## Project Structure

```
CodeAgent/
├── src/              # Rust source code
│   ├── main.rs       # Entry point
│   ├── cli.rs        # Command interface
│   ├── core/         # Orchestration engine
│   ├── integrations/ # Git, Ollama, tests
│   └── utils/        # File ops, search
├── Cargo.toml        # Project manifest
├── README.md         # Full documentation
└── target/           # Build outputs
```

## Documentation

| Document | Purpose |
|----------|---------|
| [README.md](./README.md) | Full user guide |
| [PHASE1_SUMMARY.md](./PHASE1_SUMMARY.md) | Technical overview |
| [INDEX.md](./INDEX.md) | Development guide |
| [DEVELOPMENT_LOG.md](./DEVELOPMENT_LOG.md) | Session log |

## Next Steps

1. **Read the README**: `cat README.md`
2. **Try a command**: `./target/release/codeagent run "list all files"`
3. **Review code**: Check out `src/main.rs`
4. **Explore docs**: Read [PHASE1_SUMMARY.md](./PHASE1_SUMMARY.md)
5. **Build and test**: `cargo build && cargo test`

## Getting Help

### Built-in Help

```bash
codeagent --help           # General help
codeagent run --help       # Run command help
codeagent init --help      # Init command help
```

### Documentation

- User Guide: [README.md](./README.md)
- Technical Details: [PHASE1_SUMMARY.md](./PHASE1_SUMMARY.md)
- Development: [INDEX.md](./INDEX.md)

## Version Information

- **Project**: CodeAgent
- **Phase**: 1 (MVP)
- **Version**: 0.1.0
- **Build Date**: November 8, 2025
- **Status**: Production Ready ✅

---

**You're all set!** Start using CodeAgent with your first prompt:

```bash
./target/release/codeagent run "What would you like me to do?"
```
