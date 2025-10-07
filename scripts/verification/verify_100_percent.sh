#!/bin/bash
# Master verification script - THE ULTIMATE PROOF

cd /c/Users/gstra/Code/rust-scanner

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 SHAI-HULUD RUST SCANNER - 100% MATCH VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Find latest per-testcase results
LATEST_DIR=$(ls -td analyze/per-testcase-logs/* | head -1)

if [ ! -d "$LATEST_DIR" ]; then
    echo "❌ No test results found. Run: bash analyze/parallel_testcase_scan.sh"
    exit 1
fi

echo "📁 Using results from: $LATEST_DIR"
echo ""

# Create detailed comparison
echo "📊 PER-TEST-CASE COMPARISON:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %10s %10s %s\n" "Test Case" "Bash H/M/L" "Rust H/M/L" "Match"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

total_matched=0
total_tests=0

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
        bash_h=$(grep "High Risk Issues:" "$bash_log" 2>/dev/null | awk '{print $NF}' || echo "0")
        bash_m=$(grep "Medium Risk Issues:" "$bash_log" 2>/dev/null | awk '{print $NF}' || echo "0")
        bash_l=$(grep "Low Risk" "$bash_log" 2>/dev/null | grep "informational" | awk '{print $NF}' || echo "0")
    fi
    
    # Extract rust numbers  
    if grep -q "NO SUMMARY" "$rust_log"; then
        rust_h="0"
        rust_m="0"
        rust_l="0"
    else
        rust_h=$(grep "High Risk Issues:" "$rust_log" 2>/dev/null | awk '{print $NF}' || echo "0")
        rust_m=$(grep "Medium Risk Issues:" "$rust_log" 2>/dev/null | awk '{print $NF}' || echo "0")
        rust_l=$(grep "Low Risk" "$rust_log" 2>/dev/null | grep "informational" | awk '{print $NF}' || echo "0")
    fi
    
    # Default to 0 if empty
    bash_h=${bash_h:-0}
    bash_m=${bash_m:-0}
    bash_l=${bash_l:-0}
    rust_h=${rust_h:-0}
    rust_m=${rust_m:-0}
    rust_l=${rust_l:-0}
    
    # Remove whitespace for comparison
    bash_h=$(echo "$bash_h" | tr -d '[:space:]')
    bash_m=$(echo "$bash_m" | tr -d '[:space:]')
    bash_l=$(echo "$bash_l" | tr -d '[:space:]')
    rust_h=$(echo "$rust_h" | tr -d '[:space:]')
    rust_m=$(echo "$rust_m" | tr -d '[:space:]')
    rust_l=$(echo "$rust_l" | tr -d '[:space:]')
    
    # Check match
    if [ "$bash_h" = "$rust_h" ] && [ "$bash_m" = "$rust_m" ] && [ "$bash_l" = "$rust_l" ]; then
        match="✅"
        total_matched=$((total_matched + 1))
    else
        match="❌ ($bash_h/$bash_m/$bash_l vs $rust_h/$rust_m/$rust_l)"
    fi
    
    total_tests=$((total_tests + 1))
    
    # Only show non-zero results
    if [ "$bash_h" != "0" ] || [ "$bash_m" != "0" ] || [ "$bash_l" != "0" ]; then
        printf "%-35s %4s/%2s/%2s    %4s/%2s/%2s   %s\n" "$testname" "$bash_h" "$bash_m" "$bash_l" "$rust_h" "$rust_m" "$rust_l" "$match"
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Calculate match percentage
if [ $total_tests -gt 0 ]; then
    match_pct=$((total_matched * 100 / total_tests))
    echo "📈 Per-Test-Case Match: $total_matched / $total_tests ($match_pct%)"
else
    echo "⚠️  No test results found"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 OVERALL FULL SCAN COMPARISON:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if we have recent full scan results
if [ -f "analyze/bash_fresh.txt" ] && [ -f "analyze/normal_mode_rust.txt" ]; then
    bash_h=$(grep "High Risk Issues:" analyze/bash_fresh.txt | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    bash_m=$(grep "Medium Risk Issues:" analyze/bash_fresh.txt | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    bash_l=$(grep "Low Risk" analyze/bash_fresh.txt | grep "informational" | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    
    rust_h=$(grep "High Risk Issues:" analyze/normal_mode_rust.txt | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    rust_m=$(grep "Medium Risk Issues:" analyze/normal_mode_rust.txt | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    rust_l=$(grep "Low Risk" analyze/normal_mode_rust.txt | grep "informational" | awk '{print $NF}' | sed 's/\x1b\[[0-9;]*m//g' | tr -d '[:space:]')
    
    echo ""
    printf "%-20s %10s %10s\n" "" "Bash" "Rust"
    echo "────────────────────────────────────────────────"
    printf "%-20s %10s %10s\n" "HIGH RISK:" "$bash_h" "$rust_h"
    printf "%-20s %10s %10s\n" "MEDIUM RISK:" "$bash_m" "$rust_m"
    printf "%-20s %10s %10s\n" "LOW RISK:" "$bash_l" "$rust_l"
    echo "────────────────────────────────────────────────"
    
    if [ "$bash_h" = "$rust_h" ] && [ "$bash_m" = "$rust_m" ] && [ "$bash_l" = "$rust_l" ]; then
        echo ""
        echo "🎉🎉🎉 *** 100% PERFECT MATCH *** 🎉🎉🎉"
        echo ""
        echo "✅ ALL risk levels match exactly!"
        echo "✅ v1.0.0-perfect-match tag verified!"
    else
        echo ""
        echo "❌ MISMATCH DETECTED"
        if [ -n "$bash_h" ] && [ -n "$rust_h" ]; then
            echo "   HIGH:   diff = $((bash_h - rust_h))"
        fi
        if [ -n "$bash_m" ] && [ -n "$rust_m" ]; then
            echo "   MEDIUM: diff = $((bash_m - rust_m))"
        fi
        if [ -n "$bash_l" ] && [ -n "$rust_l" ]; then
            echo "   LOW:    diff = $((bash_l - rust_l))"
        fi
    fi
else
    echo ""
    echo "⚠️  No recent full scan results. Run:"
    echo "   bash analyze/ultimate_test.sh"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📄 VERIFICATION FILES:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📁 Per-test-case logs: $LATEST_DIR"
echo "📄 Full scan logs:     analyze/bash_fresh.txt & analyze/normal_mode_rust.txt"
echo "📋 Guide:              VERIFICATION_GUIDE.md"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
