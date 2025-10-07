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

echo "📊 Step 1: Verifying Summary Numbers..."

# Strip ANSI color codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

bash_high=$(grep "High Risk Issues:" "$BASH_LOG" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_med=$(grep "Medium Risk Issues:" "$BASH_LOG" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_low=$(grep "Low Risk" "$BASH_LOG" | grep "informational" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')

rust_high=$(grep "High Risk Issues:" "$RUST_LOG" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_med=$(grep "Medium Risk Issues:" "$RUST_LOG" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_low=$(grep "Low Risk" "$RUST_LOG" | grep "informational" | tail -1 | strip_ansi | awk '{print $NF}' | tr -d ' ')

if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
    echo "   ✅ Summary: $bash_high/$bash_med/$bash_low (HIGH/MED/LOW)"
else
    echo "   ❌ Summary Mismatch!"
    echo "      Bash:  $bash_high/$bash_med/$bash_low"
    echo "      Rust:  $rust_high/$rust_med/$rust_low"
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
    echo "✅ Summary:  $bash_high HIGH / $bash_med MEDIUM / $bash_low LOW"
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
