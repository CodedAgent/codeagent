# CodeAgent - Development Index

## 📍 Quick Navigation

### Documentation
- **[README.md](./README.md)** - User documentation and quick start guide
- **[PHASE1_SUMMARY.md](./PHASE1_SUMMARY.md)** - Phase 1 completion report and metrics
- **[PROJECT_STRUCTURE.txt](./PROJECT_STRUCTURE.txt)** - Detailed file and module breakdown

### Source Code Organization
- **src/main.rs** - Application entry point
- **src/cli.rs** - Command-line interface using Clap
- **src/core/** - Core orchestration engine
  - config.rs - Configuration management
  - executor.rs - Task execution
  - planner.rs - Task planning
  - context.rs - Project context analysis
- **src/integrations/** - External integrations
  - git.rs - Git operations
  - ollama.rs - Ollama LLM client
  - test_runner.rs - Test execution
- **src/utils/** - Utility functions
  - file_utils.rs - File I/O operations
  - search.rs - Semantic search capabilities

### Configuration
- **Cargo.toml** - Project manifest and dependencies
- **.gitignore** - Version control exclusions
- **.codeagent.yml** - Project-specific configuration (auto-generated)

---

## 🚀 Quick Commands

```bash
# Build
cargo build                 # Debug build
cargo build --release      # Optimized release

# Run
./target/release/codeagent --help           # Show help
./target/release/codeagent init .           # Initialize project
./target/release/codeagent run "your task"  # Execute task

# Development
cargo run -- run "task"    # Run without building
cargo test                 # Run tests
cargo clean                # Clean builds
```

---

## 📊 Project Statistics

| Metric | Value |
|--------|-------|
| Language | Rust 2021 |
| Source Files | 14 |
| Total LOC | ~1,200 |
| Dependencies | 14 major crates |
| Release Binary | 2.9 MB |
| Build Time | <1s (incremental) |
| Phase Status | MVP Complete ✅ |

---

## 🎯 Phase 1 Features

### ✅ Completed
- CLI interface with argument parsing
- Project initialization
- Ollama LLM integration
- Git operations (stage, commit)
- File discovery and search
- Test runner detection
- Dry-run mode
- Configuration management

### 🔄 Ready for Phase 2
- Multi-step planning
- Deep framework integration
- Linter hooks
- Interactive mode
- Enhanced error handling

---

## 📦 Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.0 | Async runtime |
| clap | 4.4 | CLI parsing |
| git2 | 0.18 | Git operations |
| reqwest | 0.11 | HTTP client |
| serde | 1.0 | Serialization |
| regex | 1.10 | Pattern matching |
| walkdir | 2.4 | Directory traversal |
| tracing | 0.1 | Logging |

---

## 🔗 File Relationships

```
main.rs
├── cli.rs (command parsing)
│   └── core/executor.rs (task execution)
│       ├── core/config.rs (configuration)
│       ├── core/planner.rs (planning)
│       ├── core/context.rs (analysis)
│       ├── integrations/ollama.rs (LLM)
│       ├── integrations/git.rs (version control)
│       ├── integrations/test_runner.rs (tests)
│       ├── utils/file_utils.rs (I/O)
│       └── utils/search.rs (search)
```

---

## 🎓 Getting Started with Development

1. **Read** [README.md](./README.md) for user-facing features
2. **Review** [PHASE1_SUMMARY.md](./PHASE1_SUMMARY.md) for technical metrics
3. **Explore** source code starting with `src/main.rs`
4. **Build** with `cargo build --release`
5. **Test** with `cargo run -- run "test prompt"`

---

## 🔮 Next Phase Goals

- [ ] Implement dynamic multi-step planning
- [ ] Add Jest/Pytest/Go test integration
- [ ] Implement linter hooks
- [ ] Create interactive mode
- [ ] Generate comprehensive test coverage
- [ ] Add YAML configuration parser
- [ ] Support for external LLM APIs

---

## 📝 Important Notes

- All source code is in `src/` directory
- Build outputs go to `target/` (git-ignored)
- Configuration template in `Cargo.toml` 
- Release binary optimized in `target/release/`
- 25 compile warnings (mostly unused) - will clean in Phase 2

---

**Last Updated**: November 8, 2025
**Phase**: MVP (Phase 1) - COMPLETE ✅
**Next**: Phase 2 - Multi-step Planning & Verification
