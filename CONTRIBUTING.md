# Contributing to CodeAgent

Thank you for your interest in contributing to CodeAgent! We welcome contributions from everyone.

## Code of Conduct

Be respectful and inclusive. We're building a welcoming community for all developers.

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Git
- Cargo

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/CodeAgent.git
cd CodeAgent

# Build the project
cargo build

# Run tests
cargo test

# Build release binary
cargo build --release
```

## Development Workflow

1. **Fork the repository** on GitHub
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** with clear, descriptive commits
4. **Test thoroughly** (`cargo test`)
5. **Check code quality** (`cargo clippy`, `cargo fmt`)
6. **Push to your fork** (`git push origin feature/amazing-feature`)
7. **Open a Pull Request** with a clear description

## Commit Messages

- Use clear, descriptive commit messages
- Start with a type: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `style:`, `perf:`
- Example: `feat: add parallel step execution support`

## Code Style

We follow Rust conventions enforced by:
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- Standard Rust idioms

```bash
# Format code
cargo fmt

# Check linting
cargo clippy

# Fix common issues
cargo fix
```

## Testing

All contributions should include appropriate tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Test specific module
cargo test module_name
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments for public APIs
- Update CHANGELOG.md with your changes

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Ensure all tests pass** (`cargo test`)
4. **Pass code quality checks** (`cargo clippy`, `cargo fmt`)
5. **Update CHANGELOG.md** with your changes
6. **Write a clear PR description** explaining:
   - What changes are made
   - Why these changes are needed
   - How to test the changes
   - Any breaking changes

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.x.0`
4. Push the tag: `git push origin v0.x.0`
5. GitHub Actions automatically builds and releases

## Areas for Contribution

### High Priority
- [ ] Terminal UI (TUI) interface
- [ ] Parallel step execution
- [ ] Plugin system
- [ ] CI/CD integrations

### Medium Priority
- [ ] Additional LLM provider support
- [ ] More linter integrations
- [ ] Performance optimizations
- [ ] Test coverage improvements

### Documentation
- [ ] Improve setup guides
- [ ] Add more examples
- [ ] Create video tutorials
- [ ] Expand API documentation

## Questions?

- **Issues**: [GitHub Issues](https://github.com/yourusername/CodeAgent/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/CodeAgent/discussions)
- **Email**: maintainers@example.com

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for making CodeAgent better! 🚀
