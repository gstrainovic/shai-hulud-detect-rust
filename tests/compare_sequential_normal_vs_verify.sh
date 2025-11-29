#!/bin/bash
# Compare logs from sequential normal mode vs verify mode
# Usage: ./compare_sequential_normal_vs_verify.sh

set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 COMPARING SEQUENTIAL NORMAL vs VERIFY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1 # REMOVED

# Find latest log directories
NORMAL_DIR=$(find tests/sequential-logs -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort -r | head -1)
VERIFY_DIR=$(find tests/sequential-logs-verify -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort -r | head -1)

if [ -z "$NORMAL_DIR" ] || [ -z "$VERIFY_DIR" ]; then
    echo "❌ Missing log directories!"
    echo ""
    echo "Please run:"
    echo "  1. bash tests/full_sequential_test.sh"
    echo "  2. bash tests/full_sequential_test_verify.sh"
    echo ""
    exit 1
fi

echo "📁 Normal logs: $NORMAL_DIR"
echo "📁 Verify logs: $VERIFY_DIR"
echo ""

# Strip verification tags and timestamps for comparison
# These are Rust-only --verify mode features that Bash doesn't have
strip_verification_data() {
    sed 's/\[VERIFIED[^]]*\]//g' | \
    sed 's/\[.*confidence\]://g' | \
    sed 's/VERIFICATION SUMMARY.*//g' | \
    sed 's/🔍 VERIFICATION SUMMARY.*//g' | \
    sed 's/━━━.*//g' | \
    sed 's/\x1b\[[0-9;]*m//g' | \
    grep -v "^$" | \
    grep -v "Runtime resolver" | \
    grep -v "Runtime resolution" | \
    grep -v "Querying package manager" | \
    grep -v "Lockfile loaded" | \
    grep -v "VERIFIED" | \
    grep -v "Verified:" | \
    grep -v "Total critical findings analyzed" | \
    grep -v "verified as false positives" | \
    grep -v "⏱️  TIMING" | \
    grep -v "Started: " | \
    grep -v "Finished: " | \
    grep -v "Duration: "
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 COUNT COMPARISON (H/M/L)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Extract counts
normal_h=$(grep "High Risk Issues:" "$NORMAL_DIR/rust_full_scan.log" 2>/dev/null | awk '{print $NF}' || echo "0")
normal_m=$(grep "Medium Risk Issues:" "$NORMAL_DIR/rust_full_scan.log" 2>/dev/null | awk '{print $NF}' || echo "0")
normal_l=$(grep "Low Risk" "$NORMAL_DIR/rust_full_scan.log" 2>/dev/null | grep informational | awk '{print $NF}' || echo "0")

verify_h=$(grep "High Risk Issues:" "$VERIFY_DIR/rust_full_scan.log" 2>/dev/null | awk '{print $NF}' || echo "0")
verify_m=$(grep "Medium Risk Issues:" "$VERIFY_DIR/rust_full_scan.log" 2>/dev/null | awk '{print $NF}' || echo "0")
verify_l=$(grep "Low Risk" "$VERIFY_DIR/rust_full_scan.log" 2>/dev/null | grep informational | awk '{print $NF}' || echo "0")

if [ "$normal_h" = "$verify_h" ] && [ "$normal_m" = "$verify_m" ] && [ "$normal_l" = "$verify_l" ]; then
    echo "✅ Counts Match: $normal_h/$normal_m/$normal_l (identical)"
    MATCH_COUNT=1
else
    echo "❌ Counts Mismatch: Normal=$normal_h/$normal_m/$normal_l  Verify=$verify_h/$verify_m/$verify_l"
    MATCH_COUNT=0
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔬 CONTENT COMPARISON (without verification tags)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

strip_verification_data < "$NORMAL_DIR/rust_full_scan.log" > /tmp/normal_seq_stripped.txt
strip_verification_data < "$VERIFY_DIR/rust_full_scan.log" > /tmp/verify_seq_stripped.txt

if diff -q /tmp/normal_seq_stripped.txt /tmp/verify_seq_stripped.txt > /dev/null 2>&1; then
    echo "✅ Content identical (excluding verification tags)"
    CONTENT_MATCH=1
else
    echo "⚠️  Content differs (checking details...)"
    CONTENT_MATCH=0
    echo "      First 5 differences:"
    diff /tmp/normal_seq_stripped.txt /tmp/verify_seq_stripped.txt | head -10 | sed 's/^/      /' || true
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $MATCH_COUNT -eq 1 ] && [ $CONTENT_MATCH -eq 1 ]; then
    echo "🎉 SUCCESS! --verify does NOT change results in sequential normal mode!"
    exit 0
else
    echo "❌ FAILURE! Differences detected!"
    exit 1
fi
