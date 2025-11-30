#!/bin/bash
# Full sequential test - scans ENTIRE test-cases directory at once
# Usage: ./full_sequential_test.sh [--paranoid] [--verify]

# Parse modes
PARANOID_MODE=""
VERIFY_MODE=""
LOG_SUBDIR="sequential-logs"
MODE_LABEL="Normal Mode"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --paranoid)
            PARANOID_MODE="--paranoid"
            shift
            ;;
        --verify)
            VERIFY_MODE="--verify"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--paranoid] [--verify]"
            exit 1
            ;;
    esac
done

# Set log directory and label based on modes
if [[ -n "$PARANOID_MODE" && -n "$VERIFY_MODE" ]]; then
    LOG_SUBDIR="sequential-logs-paranoid-verify"
    MODE_LABEL="PARANOID Mode + --verify"
elif [[ -n "$PARANOID_MODE" ]]; then
    LOG_SUBDIR="sequential-logs-paranoid"
    MODE_LABEL="PARANOID Mode"
elif [[ -n "$VERIFY_MODE" ]]; then
    LOG_SUBDIR="sequential-logs-verify"
    MODE_LABEL="Normal Mode + --verify"
else
    LOG_SUBDIR="sequential-logs"
    MODE_LABEL="Normal Mode"
fi

# Get the absolute path of the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TESTCASES_ROOT="$(cd "$PROJECT_ROOT/../shai-hulud-detect/test-cases" && pwd)"
BASH_SCANNER="$PROJECT_ROOT/../shai-hulud-detect/shai-hulud-detector.sh"

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="$SCRIPT_DIR/$LOG_SUBDIR/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔧 FULL SEQUENTIAL TEST - $MODE_LABEL"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs: $LOG_DIR"
echo "📂 Target: shai-hulud-detect/test-cases/ (ALL test cases at once)"
echo ""

# Start building Rust scanner in background
echo "🔨 Building Rust scanner binary in background..."
cd "$PROJECT_ROOT"
cargo build --release --quiet &
BUILD_PID=$!
echo "✅ Build started (PID: $BUILD_PID)"
echo ""

# Phase 1: Bash scanner (runs while Rust builds)
# Timeout: 30 minutes (1800s) - scanning 32+ test cases takes time
echo "🔵 Phase 1: Running Bash scanner on ENTIRE test-cases directory..."
timeout 1800 "$BASH_SCANNER" $PARANOID_MODE "$TESTCASES_ROOT" > "$LOG_DIR/bash_full_scan.log" 2>&1
bash_exit=$?

if [ $bash_exit -eq 124 ]; then
    echo "⏱️  Bash TIMEOUT (>30 min)"
elif [ $bash_exit -eq 0 ]; then
    echo "✅ Bash completed"
else
    echo "⚠️  Bash exit code: $bash_exit"
fi

grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/bash_full_scan.log" > "$LOG_DIR/bash_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_summary.txt"

# Wait for Rust build
echo ""
echo "⏳ Waiting for Rust build to complete..."
wait $BUILD_PID
BUILD_EXIT=$?
if [ $BUILD_EXIT -ne 0 ]; then
    echo "❌ Rust build failed with exit code $BUILD_EXIT!"
    exit 1
fi
echo "✅ Rust binary ready"

echo ""

# Phase 2: Rust scanner
echo "🟢 Phase 2: Running Rust scanner on ENTIRE test-cases directory..."
"$PROJECT_ROOT/target/release/shai-hulud-detector" $PARANOID_MODE $VERIFY_MODE "$TESTCASES_ROOT" > "$LOG_DIR/rust_full_scan.log" 2>&1
rust_exit=$?

if [ -f "scan_results.json" ]; then
    mv "scan_results.json" "$LOG_DIR/rust_full_scan.json"
    echo "💾 JSON saved"
fi

if [ $rust_exit -eq 0 ]; then
    echo "✅ Rust completed"
else
    echo "⚠️  Rust exit code: $rust_exit"
fi

grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/rust_full_scan.log" > "$LOG_DIR/rust_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_summary.txt"

echo ""
echo "📊 Comparing results..."

strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

bash_high=$(grep "High Risk Issues:" "$LOG_DIR/bash_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_med=$(grep "Medium Risk Issues:" "$LOG_DIR/bash_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
bash_low=$(grep "Low Risk" "$LOG_DIR/bash_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')

rust_high=$(grep "High Risk Issues:" "$LOG_DIR/rust_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_med=$(grep "Medium Risk Issues:" "$LOG_DIR/rust_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
rust_low=$(grep "Low Risk" "$LOG_DIR/rust_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')

bash_high=${bash_high:-0}
bash_med=${bash_med:-0}
bash_low=${bash_low:-0}
rust_high=${rust_high:-0}
rust_med=${rust_med:-0}
rust_low=${rust_low:-0}

if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
    match="✅ MATCH"
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
echo "📊 RESULTS - $MODE_LABEL"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "Bash: %s/%s/%s | Rust: %s/%s/%s | %s\n" "$bash_high" "$bash_med" "$bash_low" "$rust_high" "$rust_med" "$rust_low" "$match"

# Pattern verification
# NOTE: Sequential scans show truncated output in Bash (max 5 per category)
# so pattern-level verification is skipped. H/M/L counts are still verified.
echo ""
echo "🔬 Pattern-level verification..."
echo "   ⏭️  Skipped for sequential scans (Bash truncates output to max 5 per category)"
echo "   ✅ H/M/L count verification is sufficient for sequential tests"
PATTERN_OK=true

echo ""
echo "⏱️  Duration: ${MINUTES}m ${SECONDS}s"
echo "💾 Logs: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$match" = "✅ MATCH" ] && [ "$PATTERN_OK" = true ]; then
    echo "🎉 FULL VERIFICATION PASSED!"
    exit 0
else
    echo "⚠️  Differences detected - review logs"
    exit 1
fi
