#!/bin/bash
# Test the bash log parser with actual test data

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1/bash-log-parser

echo "Testing Bash Log Parser..."
echo ""

# Use a specific complete test run
LOG_DIR="../scripts/analyze/per-testcase-logs/20251011_230945"

if [ ! -d "$LOG_DIR" ]; then
    echo "❌ Test log directory not found: $LOG_DIR"
    exit 1
fi

echo "Using logs from: $LOG_DIR"
echo ""

# Test with infected-project (has many findings)
BASH_LOG="$LOG_DIR/bash_infected-project.log"
RUST_JSON="$LOG_DIR/rust_infected-project.json"

if [ ! -f "$BASH_LOG" ]; then
    echo "❌ Bash log not found: $BASH_LOG"
    exit 1
fi

if [ ! -f "$RUST_JSON" ]; then
    echo "❌ Rust JSON not found: $RUST_JSON"
    exit 1
fi

echo "🔍 Comparing:"
echo "   Bash: $BASH_LOG"
echo "   Rust: $RUST_JSON"
echo ""

./target/release/bash-log-parser "$BASH_LOG" "$RUST_JSON"

echo ""
echo "✅ Parser test complete!"
