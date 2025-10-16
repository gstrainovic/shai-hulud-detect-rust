#!/bin/bash
# Compare logs from normal mode vs verify mode to ensure --verify doesn't change results
# Usage: ./compare_normal_vs_verify.sh

set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 COMPARING NORMAL MODE vs VERIFY MODE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This script compares logs to verify that --verify flag ONLY adds"
echo "verification tags and does NOT change H/M/L counts or findings."
echo ""

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

# Find latest log directories
NORMAL_DIR=$(find scripts/analyze/per-testcase-logs -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort -r | head -1)
VERIFY_DIR=$(find scripts/analyze/per-testcase-logs-verify -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort -r | head -1)

if [ -z "$NORMAL_DIR" ] || [ -z "$VERIFY_DIR" ]; then
    echo "❌ Missing log directories!"
    echo ""
    echo "Please run:"
    echo "  1. bash scripts/analyze/parallel_testcase_scan.sh"
    echo "  2. bash scripts/analyze/parallel_testcase_scan_verify.sh"
    echo ""
    exit 1
fi

echo "📁 Normal mode logs: $NORMAL_DIR"
echo "📁 Verify mode logs: $VERIFY_DIR"
echo ""

# Strip verification tags and timestamps for comparison
strip_verification_data() {
    sed 's/\[VERIFIED[^]]*\]//g' | \
    sed 's/\[.*confidence\]://g' | \
    sed 's/VERIFICATION SUMMARY.*//g' | \
    sed 's/🔍 VERIFICATION SUMMARY.*//g' | \
    sed 's/━━━.*//g' | \
    sed 's/\x1b\[[0-9;]*m//g' | \
    grep -v "^$" | \
    grep -v "Runtime resolver" | \
    grep -v "Querying package manager" | \
    grep -v "VERIFIED" | \
    grep -v "Verified:" | \
    grep -v "Total critical findings analyzed" | \
    grep -v "verified as false positives"
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 COUNT COMPARISON (H/M/L)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

MISMATCH_COUNT=0
MATCH_COUNT=0
TOTAL_TESTS=0

for normal_log in "$NORMAL_DIR"/rust_*.log; do
    testname=$(basename "$normal_log" | sed 's/rust_//' | sed 's/.log$//')
    verify_log="$VERIFY_DIR/rust_${testname}.log"
    
    if [ ! -f "$verify_log" ]; then
        echo "⚠️  Missing verify log for: $testname"
        continue
    fi
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Extract counts
    normal_h=$(grep "High Risk Issues:" "$normal_log" 2>/dev/null | awk '{print $NF}' || echo "0")
    normal_m=$(grep "Medium Risk Issues:" "$normal_log" 2>/dev/null | awk '{print $NF}' || echo "0")
    normal_l=$(grep "Low Risk" "$normal_log" 2>/dev/null | grep informational | awk '{print $NF}' || echo "0")
    
    verify_h=$(grep "High Risk Issues:" "$verify_log" 2>/dev/null | awk '{print $NF}' || echo "0")
    verify_m=$(grep "Medium Risk Issues:" "$verify_log" 2>/dev/null | awk '{print $NF}' || echo "0")
    verify_l=$(grep "Low Risk" "$verify_log" 2>/dev/null | grep informational | awk '{print $NF}' || echo "0")
    
    if [ "$normal_h" = "$verify_h" ] && [ "$normal_m" = "$verify_m" ] && [ "$normal_l" = "$verify_l" ]; then
        echo "✅ $testname: $normal_h/$normal_m/$normal_l (identical)"
        MATCH_COUNT=$((MATCH_COUNT + 1))
    else
        echo "❌ $testname: Normal=$normal_h/$normal_m/$normal_l  Verify=$verify_h/$verify_m/$verify_l  (MISMATCH!)"
        MISMATCH_COUNT=$((MISMATCH_COUNT + 1))
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Total test cases: $TOTAL_TESTS"
echo "Perfect matches: $MATCH_COUNT"
echo "Mismatches: $MISMATCH_COUNT"
echo ""

if [ $MISMATCH_COUNT -eq 0 ]; then
    echo "🎉 SUCCESS! --verify does NOT change H/M/L counts!"
    echo "   All $TOTAL_TESTS test cases have identical counts."
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🔬 CONTENT COMPARISON (without verification tags)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Comparing actual findings (ignoring verification metadata)..."
    echo ""
    
    CONTENT_MISMATCH=0
    for normal_log in "$NORMAL_DIR"/rust_*.log; do
        testname=$(basename "$normal_log" | sed 's/rust_//' | sed 's/.log$//')
        verify_log="$VERIFY_DIR/rust_${testname}.log"
        
        if [ ! -f "$verify_log" ]; then
            continue
        fi
        
        # Strip verification data and compare
        strip_verification_data < "$normal_log" > /tmp/normal_stripped.txt
        strip_verification_data < "$verify_log" > /tmp/verify_stripped.txt
        
        if diff -q /tmp/normal_stripped.txt /tmp/verify_stripped.txt > /dev/null 2>&1; then
            echo "  ✅ $testname: Content identical (excluding verification tags)"
        else
            echo "  ⚠️  $testname: Content differs (checking details...)"
            CONTENT_MISMATCH=$((CONTENT_MISMATCH + 1))
            
            # Show first few differences
            echo "      First 5 differences:"
            diff /tmp/normal_stripped.txt /tmp/verify_stripped.txt | head -10 | sed 's/^/      /'
        fi
    done
    
    echo ""
    if [ $CONTENT_MISMATCH -eq 0 ]; then
        echo "🎉 PERFECT! All findings are identical (excluding verification tags)!"
    else
        echo "⚠️  $CONTENT_MISMATCH test cases have content differences."
        echo "   This might be expected if verification adds contextual information."
    fi
else
    echo "❌ FAILURE! --verify CHANGED the H/M/L counts!"
    echo "   This is a BUG! --verify should ONLY add tags, not change results."
    echo ""
    echo "Action required:"
    echo "  1. Review mismatched test cases"
    echo "  2. Fix the verification logic"
    echo "  3. Re-run tests"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Done!"
echo ""
