#!/bin/bash
# Ultra-simple verification: just compare summary numbers

cd /c/Users/gstra/Code/rust-scanner

BASH_LOG="shai-hulud-detect/bash-testcases.log"
RUST_LOG="dev-rust-scanner-1/logs/rust-testcases-new.log"

if [ ! -f "$RUST_LOG" ]; then
    echo "⚡ Generating Rust log..."
    cd dev-rust-scanner-1
    cargo run --quiet -- ../shai-hulud-detect/test-cases > logs/rust-testcases-new.log 2>&1
    cd ..
fi

echo "🔍 Comparing summary numbers..."
echo ""

# Extract summary
bash_high=$(grep "High Risk Issues:" "$BASH_LOG" | tail -1 | awk '{print $NF}')
bash_med=$(grep "Medium Risk Issues:" "$BASH_LOG" | tail -1 | awk '{print $NF}')
bash_low=$(grep "Low Risk (informational):" "$BASH_LOG" | tail -1 | awk '{print $NF}')

rust_high=$(grep "High Risk Issues:" "$RUST_LOG" | tail -1 | awk '{print $NF}')
rust_med=$(grep "Medium Risk Issues:" "$RUST_LOG" | tail -1 | awk '{print $NF}')
rust_low=$(grep "Low Risk (informational):" "$RUST_LOG" | tail -1 | awk '{print $NF}')

echo "Bash:  HIGH=$bash_high  MEDIUM=$bash_med  LOW=$bash_low"
echo "Rust:  HIGH=$rust_high  MEDIUM=$rust_med  LOW=$rust_low"
echo ""

if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
    echo "✅ 100% MATCH - Summary numbers identical!"
    exit 0
else
    echo "❌ MISMATCH!"
    exit 1
fi
