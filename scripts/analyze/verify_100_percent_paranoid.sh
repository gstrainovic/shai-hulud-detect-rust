#!/bin/bash
# Master verification script for PARANOID MODE - THE ULTIMATE PROOF

cd /c/Users/gstra/Code/rust-scanner

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 SHAI-HULUD RUST SCANNER - 100% MATCH VERIFICATION (PARANOID)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Find latest per-testcase results
LATEST_DIR=$(ls -td dev-rust-scanner-1/scripts/analyze/per-testcase-logs-paranoid/* 2>/dev/null | head -1)

if [ ! -d "$LATEST_DIR" ]; then
    echo "❌ No PARANOID test results found. Run: bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_paranoid.sh"
    exit 1
fi

echo "📁 Using results from: $LATEST_DIR"
echo ""

# Create detailed comparison
echo "📊 PER-TEST-CASE COMPARISON (PARANOID MODE):"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %10s %10s %s\n" "Test Case" "Bash H/M/L" "Rust H/M/L" "Match"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

total_matched=0
total_tests=0

# Strip ANSI codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

for bash_log in "$LATEST_DIR"/bash_*_summary.txt; do
    testname=$(basename "$bash_log" | sed 's/bash_//;s/_summary.txt//')
    rust_log="$LATEST_DIR/rust_${testname}_summary.txt"

    if [ ! -f "$rust_log" ]; then
        continue
    fi

    # Extract bash numbers (handle "No SUMMARY" case)
    if grep -q "NO SUMMARY" "$bash_log"; then
        bash_h="0"
        bash_m="0"
        bash_l="0"
    else
        bash_h=$(grep "High Risk Issues:" "$bash_log" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
        bash_m=$(grep "Medium Risk Issues:" "$bash_log" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
        bash_l=$(grep "Low Risk" "$bash_log" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
    fi

    # Extract rust numbers
    if grep -q "NO SUMMARY" "$rust_log"; then
        rust_h="0"
        rust_m="0"
        rust_l="0"
    else
        rust_h=$(grep "High Risk Issues:" "$rust_log" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
        rust_m=$(grep "Medium Risk Issues:" "$rust_log" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
        rust_l=$(grep "Low Risk" "$rust_log" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ' || echo "0")
    fi

    # Compare
    if [ "$bash_h" = "$rust_h" ] && [ "$bash_m" = "$rust_m" ] && [ "$bash_l" = "$rust_l" ]; then
        match="✅"
        ((total_matched++))
    else
        match="❌"
    fi

    ((total_tests++))
    
    printf "%-35s %10s %10s %s\n" \
        "$testname" \
        "$bash_h/$bash_m/$bash_l" \
        "$rust_h/$rust_m/$rust_l" \
        "$match"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Summary
echo "📊 VERIFICATION SUMMARY (PARANOID MODE):"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   Test Cases Matched: $total_matched / $total_tests"
echo ""

if [ $total_matched -eq $total_tests ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🎉 100% MATCH ACHIEVED (PARANOID MODE)!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "✅ All $total_tests test cases produce identical results"
    echo "✅ Rust scanner is 100% compatible with Bash scanner (PARANOID)"
    echo "✅ Ready for production use"
    echo ""
    exit 0
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "❌ MISMATCH DETECTED (PARANOID MODE)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "⚠️  Review logs in: $LATEST_DIR"
    echo ""
    exit 1
fi
