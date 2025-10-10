#!/bin/bash
# Full sequential test PARANOID MODE - scans ENTIRE test-cases directory at once
# This tests how scanners handle the complete collection in paranoid mode

cd /c/Users/gstra/Code/rust-scanner

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="dev-rust-scanner-1/scripts/analyze/sequential-logs-paranoid/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "� FULL SEQUENTIAL TEST - PARANOID MODE - ENTIRE test-cases/ Directory"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs: $LOG_DIR"
echo "📂 Target: shai-hulud-detect/test-cases/ (ALL test cases at once - PARANOID)"
echo ""

# Phase 1: Bash scanner PARANOID on ENTIRE test-cases directory
echo "🔵 Phase 1: Running Bash scanner (PARANOID) on ENTIRE test-cases directory..."
cd shai-hulud-detect
timeout 600 ./shai-hulud-detector.sh --paranoid test-cases/ > "../$LOG_DIR/bash_full_scan.log" 2>&1
bash_exit=$?
cd ..

if [ $bash_exit -eq 124 ]; then
    echo "⏱️  Bash scanner TIMEOUT (>10 min)" | tee -a "$LOG_DIR/bash_full_scan.log"
elif [ $bash_exit -eq 0 ]; then
    echo "✅ Bash scanner completed"
else
    echo "⚠️  Bash scanner exit code: $bash_exit"
fi

# Extract bash summary
grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/bash_full_scan.log" > "$LOG_DIR/bash_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_summary.txt"

echo ""

# Phase 2: Rust scanner PARANOID on ENTIRE test-cases directory
echo "🟢 Phase 2: Running Rust scanner (PARANOID) on ENTIRE test-cases directory..."
cd dev-rust-scanner-1
cargo run --quiet --release -- --paranoid ../shai-hulud-detect/test-cases/ > "../$LOG_DIR/rust_full_scan.log" 2>&1
rust_exit=$?

# Save JSON output
if [ -f "scan_results.json" ]; then
    mv "scan_results.json" "../$LOG_DIR/rust_full_scan.json"
    echo "💾 JSON results saved: $LOG_DIR/rust_full_scan.json"
fi

cd ..

if [ $rust_exit -eq 0 ]; then
    echo "✅ Rust scanner completed"
else
    echo "⚠️  Rust scanner exit code: $rust_exit"
fi

# Extract rust summary
grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/rust_full_scan.log" > "$LOG_DIR/rust_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_summary.txt"

echo ""
echo "📊 Comparing results..."

# Strip ANSI codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

# Extract numbers
bash_high=$(grep "High Risk Issues:" "$LOG_DIR/bash_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_med=$(grep "Medium Risk Issues:" "$LOG_DIR/bash_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_low=$(grep "Low Risk" "$LOG_DIR/bash_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')

rust_high=$(grep "High Risk Issues:" "$LOG_DIR/rust_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_med=$(grep "Medium Risk Issues:" "$LOG_DIR/rust_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_low=$(grep "Low Risk" "$LOG_DIR/rust_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')

# Default to 0
bash_high=${bash_high:-0}
bash_med=${bash_med:-0}
bash_low=${bash_low:-0}
rust_high=${rust_high:-0}
rust_med=${rust_med:-0}
rust_low=${rust_low:-0}

# Check match
if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
    match="✅ PERFECT MATCH"
else
    match="❌ MISMATCH"
fi

END_TIME=$(date +%s)
END_READABLE=$(date "+%Y-%m-%d %H:%M:%S")
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 FULL SCAN RESULTS - PARANOID MODE (Entire test-cases/ Directory)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
printf "%-20s %15s %15s\n" "Scanner" "Findings (H/M/L)" "Status"
echo "──────────────────────────────────────────────────────────"
printf "%-20s %4s/%3s/%3s      %s\n" "Bash (PARANOID)" "$bash_high" "$bash_med" "$bash_low" "Exit: $bash_exit"
printf "%-20s %4s/%3s/%3s      %s\n" "Rust (PARANOID)" "$rust_high" "$rust_med" "$rust_low" "Exit: $rust_exit"
echo "──────────────────────────────────────────────────────────"
echo ""
echo "🎯 Result: $match"
echo ""
echo "⏱️  TIMING:"
echo "   Started:  $START_READABLE"
echo "   Finished: $END_READABLE"
echo "   Duration: ${MINUTES}m ${SECONDS}s"
echo ""
echo "💾 Logs saved:"
echo "   Bash:     $LOG_DIR/bash_full_scan.log"
echo "   Rust:     $LOG_DIR/rust_full_scan.log"
echo "   Summary:  $LOG_DIR/*_summary.txt"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Save comparison
cat > "$LOG_DIR/comparison.txt" << EOF
FULL SCAN COMPARISON - PARANOID MODE (Entire test-cases/ directory)
=====================================================================

Bash Scanner (PARANOID): $bash_high HIGH / $bash_med MEDIUM / $bash_low LOW
Rust Scanner (PARANOID): $rust_high HIGH / $rust_med MEDIUM / $rust_low LOW

Match: $match

Started:  $START_READABLE
Finished: $END_READABLE
Duration: ${MINUTES}m ${SECONDS}s
EOF

if [ "$match" = "✅ PERFECT MATCH" ]; then
    exit 0
else
    echo "⚠️  Scanners produced different results - review logs for details"
    exit 1
fi

