#!/bin/bash
# Verify 100% identical findings (order-independent)
# Sorts both outputs and compares - findings must match exactly

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 SORTED COMPARISON - Order-Independent Verification"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Use existing logs if available
BASH_LOG="shai-hulud-detect/bash-testcases.log"
RUST_LOG="dev-rust-scanner-1/logs/rust-testcases-new.log"

if [ ! -f "$BASH_LOG" ]; then
    echo "❌ Bash log not found: $BASH_LOG"
    echo "   Run: cd shai-hulud-detect && ./shai-hulud-detector.sh test-cases/ | tee bash-testcases.log"
    exit 1
fi

if [ ! -f "$RUST_LOG" ]; then
    echo "⚡ Rust log not found, generating..."
    cd dev-rust-scanner-1
    mkdir -p logs
    cargo run --quiet -- ../shai-hulud-detect/test-cases > logs/rust-testcases-new.log 2>&1
    cd ..
fi

echo "✅ Using Bash log: $BASH_LOG"
echo "✅ Using Rust log: $RUST_LOG"
echo ""
echo "📊 Extracting and sorting findings..."

echo ""
echo "📊 Extracting and sorting findings..."

# Extract findings sections (skip headers/footers)
# Skip: 📦 Loaded, Starting, Scanning, 🔍 Checking, ═══, SUMMARY

extract_findings() {
    local file=$1
    # Extract ONLY the actual finding lines (- Package:, - Pattern:, - Activity:, - Issue:, etc.)
    # Skip advice lines that also start with "   -" but don't have colons
    grep "^   - " "$file" | grep -E "(Package|Pattern|Activity|Issue|Warning|Hook|Repository|file|Found in):" | sort
}

extract_findings "$BASH_LOG" > /tmp/bash_sorted.txt
extract_findings "$RUST_LOG" > /tmp/rust_sorted.txt

# Compare sorted findings
echo "🔍 Comparing sorted findings..."
if diff -q /tmp/bash_sorted.txt /tmp/rust_sorted.txt > /dev/null; then
    echo ""
    echo "✅ 100% MATCH!"
    echo "   All findings identical (order-independent)"
    echo ""
    echo "   Bash findings: $(wc -l < /tmp/bash_sorted.txt)"
    echo "   Rust findings: $(wc -l < /tmp/rust_sorted.txt)"
    exit 0
else
    echo ""
    echo "❌ DIFFERENCES FOUND!"
    echo ""
    echo "Showing first 50 differences:"
    diff -u /tmp/bash_sorted.txt /tmp/rust_sorted.txt | head -50
    echo ""
    echo "Full diff saved to /tmp/findings_diff.txt"
    diff -u /tmp/bash_sorted.txt /tmp/rust_sorted.txt > /tmp/findings_diff.txt
    exit 1
fi
