# CodeAgent - Phase 1 MVP

A powerful AI-driven code agent that understands your project context and autonomously executes complex, multi-step engineering tasks.

## Installation

### Build from Source

```bash
cargo build --release
./target/release/codeagent --version
```

## Quick Start

### Initialize a Project

```bash
codeagent init /path/to/project
```

This creates a `.codeagent.yml` configuration file with default settings (uses Ollama by default).

### Run a Task

```bash
# Simple task
codeagent run "Find all TODO comments in the codebase"

# With dry-run preview
codeagent run "Refactor deprecated function calls" --dry-run
```

## Features (Phase 1)

- **CLI Interface**: Simple, intuitive command structure
- **Project Context**: Analyzes entire project structure
- **Git Integration**: Stage, commit, and manage changes
- **Test Runner**: Automatic test execution and verification
- **File Search**: Pattern matching and keyword search
- **Local Model Support**: Ollama integration for local LLM inference
- **Dry Run Mode**: Preview changes before applying them

## Configuration

The `.codeagent.yml` file supports:

```yaml
model_provider: ollama
ollama_base_url: "http://localhost:11434"
ollama_model: "mistral"
```

## Architecture

```bash
src/
├── cli.rs           # Command-line interface
├── core/
│   ├── config.rs    # Configuration management
│   ├── executor.rs  # Task execution engine
│   ├── planner.rs   # Task planning
│   └── context.rs   # Project context analysis
├── integrations/
│   ├── git.rs       # Git operations
│   ├── ollama.rs    # Ollama LLM client
│   └── test_runner.rs # Test execution
└── utils/
    ├── file_utils.rs # File operations
    └── search.rs     # Semantic search
```

## Requirements

- **Rust 1.70+**
- **Git** (for version control features)
- **Ollama** (optional, for local LLM support)

## Development

### Building

```bash
cargo build        # Debug build
cargo build --release  # Release build
```

### Running Tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run -- run "Your task description"
cargo run -- init .
```

## Roadmap

### Phase 1 (Current)

- ✅ Core CLI framework
- ✅ Local Ollama integration
- ✅ Git integration basics
- ✅ File utilities and search
- ✅ Test runner integration
- 🔄 Orchestration engine refinement

### Phase 2

- Multi-step planning
- Deep test framework integration
- Linter integration
- Interactive/dry-run modes

### Phase 3

- External LLM APIs (OpenAI, Anthropic)
- Configuration management
- Pull request generation
- TUI interface

## Example Use Cases

1. **Find and replace deprecated functions**

   ```bash
   codeagent run "Replace all instances of deprecated_fn with new_fn"
   ```

2. **Clean up unused code**

   ```bash
   codeagent run "Find and remove all unused functions from the repository"
   ```

3. **Update dependencies**

   ```bash
   codeagent run "Update all dependencies to the latest compatible versions"
   ```

## Contributing

Contributions are welcome! Please submit pull requests or open issues for bugs and feature requests.

## License

MIT License
