#!/bin/bash

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║        CodeAgent Phase 1 Setup Verification Script             ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

check_mark="✅"
cross_mark="❌"

# Check Rust installation
echo "Checking Rust installation..."
if command -v cargo &> /dev/null; then
    echo "$check_mark Rust and Cargo found"
    cargo --version | sed 's/^/   /'
else
    echo "$cross_mark Rust not installed - please install from rustup.rs"
    exit 1
fi
echo ""

# Check project files
echo "Checking project files..."
files=("Cargo.toml" "README.md" "PHASE1_SUMMARY.md" "src/main.rs" ".gitignore")
all_present=true

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "$check_mark $file"
    else
        echo "$cross_mark $file (missing)"
        all_present=false
    fi
done
echo ""

if [ "$all_present" = false ]; then
    echo "Some files are missing!"
    exit 1
fi

# Check Rust files
echo "Counting Rust source files..."
rust_count=$(find src -name "*.rs" | wc -l | tr -d ' ')
echo "$check_mark Found $rust_count Rust source files"
echo ""

# Try to build
echo "Building project..."
if cargo build --release &> /dev/null; then
    echo "$check_mark Release build successful"
    binary_size=$(ls -lh target/release/codeagent | awk '{print $5}')
    echo "   Binary size: $binary_size"
else
    echo "$cross_mark Build failed - run 'cargo build' for details"
    exit 1
fi
echo ""

# Test CLI
echo "Testing CLI..."
if ./target/release/codeagent --version &> /dev/null; then
    echo "$check_mark CLI is working"
    ./target/release/codeagent --version | sed 's/^/   /'
else
    echo "$cross_mark CLI test failed"
    exit 1
fi
echo ""

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              ✅ All checks passed! Setup complete.             ║"
echo "║                                                                ║"
echo "║  Next steps:                                                   ║"
echo "║  1. Run: ./target/release/codeagent --help                     ║"
echo "║  2. Read: README.md for documentation                          ║"
echo "║  3. Try: ./target/release/codeagent run \"your task\"           ║"
echo "╚════════════════════════════════════════════════════════════════╝"
