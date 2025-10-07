#!/bin/bash
# Parallel per-test-case PARANOID MODE scanner with detailed logging
# This creates baseline data for each test case subfolder in paranoid mode

cd /c/Users/gstra/Code/rust-scanner

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="dev-rust-scanner-1/scripts/analyze/per-testcase-logs-paranoid/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "🚀 Starting parallel per-test-case PARANOID scans..."
echo "📁 Logs will be in: $LOG_DIR"
echo ""

# Function to run bash scanner on a single test case (PARANOID)
run_bash_testcase_paranoid() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/bash_${testname}.log"
    
    echo "⏳ [$(date +%H:%M:%S)] Starting: $testname (PARANOID)"
    
    # Run bash scanner (PARANOID mode) - use absolute path
    cd shai-hulud-detect
    local abs_testdir=$(realpath "../$testdir")
    timeout 300 ./shai-hulud-detector.sh --paranoid "$abs_testdir" > "../$logfile" 2>&1
    local exit_code=$?
    cd ..
    
    if [ $exit_code -eq 124 ]; then
        echo "⏱️  [$(date +%H:%M:%S)] TIMEOUT: $testname (>5min)" | tee -a "$logfile"
    elif [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (PARANOID)" 
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (exit $exit_code)" | tee -a "$logfile"
    fi
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_${testname}_summary.txt"
}

# Function to run rust scanner on a single test case (PARANOID)
run_rust_testcase_paranoid() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/rust_${testname}.log"
    
    echo "⚡ [$(date +%H:%M:%S)] Starting: $testname (Rust PARANOID)"
    
    # Run rust scanner (PARANOID mode) - use absolute path
    cd dev-rust-scanner-1
    local abs_testdir=$(realpath "../$testdir")
    cargo run --quiet --release -- --paranoid "$abs_testdir" > "../$logfile" 2>&1
    local exit_code=$?
    cd ..
    
    if [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust PARANOID)"
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (Rust, exit $exit_code)" | tee -a "$logfile"
    fi
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_${testname}_summary.txt"
}

export -f run_bash_testcase_paranoid
export -f run_rust_testcase_paranoid
export LOG_DIR

# Find all test cases
TEST_CASES=$(find shai-hulud-detect/test-cases -mindepth 1 -maxdepth 1 -type d | sort)
TEST_COUNT=$(echo "$TEST_CASES" | wc -l)

echo "Found $TEST_COUNT test cases"
echo ""

# Phase 1: Run Bash scanners in parallel (max 4 concurrent)
echo "🔵 Phase 1: Running Bash scanners in PARANOID mode (max 4 concurrent)..."
echo "$TEST_CASES" | xargs -I {} -P 4 bash -c 'run_bash_testcase_paranoid "$@"' _ {}

echo ""
echo "🔵 Phase 2: Running Rust scanners in PARANOID mode (max 4 concurrent)..."
echo "$TEST_CASES" | xargs -I {} -P 4 bash -c 'run_rust_testcase_paranoid "$@"' _ {}

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ PARANOID MODE: All test cases scanned!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📁 Logs: $LOG_DIR"
echo ""
echo "Next step: bash dev-rust-scanner-1/scripts/analyze/verify_100_percent_paranoid.sh"
