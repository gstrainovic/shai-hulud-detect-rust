#!/bin/bash
# Search for LOW RISK sections in ALL Bash logs

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 Searching for LOW RISK sections in ALL Bash logs..."
echo ""

FOUND_LOW_RISK=0
TOTAL_LOGS=0

for bash_log in scripts/analyze/per-testcase-logs/*/bash_*.log; do
    if [ -f "$bash_log" ]; then
        TOTAL_LOGS=$((TOTAL_LOGS + 1))
        testname=$(basename "$bash_log" .log | sed 's/bash_//')
        
        # Check if log has "LOW RISK:" section (not just "Low Risk (informational)")
        if grep -q "^⚠️.*LOW RISK:" "$bash_log" || grep -q "^🔵.*LOW RISK:" "$bash_log"; then
            echo "✅ $testname: Has LOW RISK section"
            FOUND_LOW_RISK=$((FOUND_LOW_RISK + 1))
            grep -A 10 "LOW RISK:" "$bash_log" | head -15
            echo ""
        fi
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 SUMMARY:"
echo "   Total Bash logs: $TOTAL_LOGS"
echo "   With LOW RISK section: $FOUND_LOW_RISK"
echo "   Without LOW RISK section: $((TOTAL_LOGS - FOUND_LOW_RISK))"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
