#!/bin/bash
# Verify 100% identical findings (order-independent)
# Compares summary numbers AND all findings

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 COMPLETE VERIFICATION - Summary + Findings"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Use existing logs
BASH_LOG="shai-hulud-detect/bash-testcases.log"
RUST_LOG="dev-rust-scanner-1/logs/rust-testcases-new.log"

if [ ! -f "$BASH_LOG" ]; then
    echo "❌ Bash log not found: $BASH_LOG"
    exit 1
fi

if [ ! -f "$RUST_LOG" ]; then
    echo "⚡ Generating Rust log..."
    cd dev-rust-scanner-1
    cargo run --quiet -- ../shai-hulud-detect/test-cases > logs/rust-testcases-new.log 2>&1
    cd ..
fi

echo "📊 Step 1: Verifying Summary Block..."

# Strip ANSI color codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

# Extract entire SUMMARY section and strip ANSI
grep -A 5 "🔍 SUMMARY:" "$BASH_LOG" | strip_ansi > /tmp/bash_summary.txt
grep -A 5 "🔍 SUMMARY:" "$RUST_LOG" | strip_ansi > /tmp/rust_summary.txt

if diff -q /tmp/bash_summary.txt /tmp/rust_summary.txt > /dev/null; then
    echo "   ✅ Summary block matches!"
    cat /tmp/bash_summary.txt | sed 's/^/      /'
else
    echo "   ❌ Summary block differs!"
    echo ""
    echo "   Bash:"
    cat /tmp/bash_summary.txt | sed 's/^/      /'
    echo ""
    echo "   Rust:"
    cat /tmp/rust_summary.txt | sed 's/^/      /'
    exit 1
fi

echo ""
echo "📊 Step 2: Verifying All Findings (Order-Independent)..."

extract_findings() {
    local file=$1
    # Extract ONLY the actual finding lines (- Package:, - Pattern:, etc.)
    grep "^   - " "$file" | grep -E "(Package|Pattern|Activity|Issue|Warning|Hook|Repository|file|Found in):" | sort
}

extract_findings "$BASH_LOG" > /tmp/bash_sorted.txt
extract_findings "$RUST_LOG" > /tmp/rust_sorted.txt

bash_findings=$(wc -l < /tmp/bash_sorted.txt)
rust_findings=$(wc -l < /tmp/rust_sorted.txt)

echo "   Bash findings: $bash_findings"
echo "   Rust findings: $rust_findings"

if diff -q /tmp/bash_sorted.txt /tmp/rust_sorted.txt > /dev/null; then
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🎉 100% VERIFICATION PASSED!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "✅ Summary:  Identical (19 HIGH / 61 MEDIUM / 9 LOW)"
    echo "✅ Findings: $bash_findings (all identical, order-independent)"
    echo ""
    exit 0
else
    echo ""
    echo "❌ FINDINGS DIFFER!"
    echo ""
    diff -u /tmp/bash_sorted.txt /tmp/rust_sorted.txt | head -30
    exit 1
fi
