#!/bin/bash
# Parallel per-test-case PARANOID MODE scanner with detailed logging
# This creates baseline data for each test case subfolder in paranoid mode

cd /c/Users/gstra/Code/rust-scanner

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="dev-rust-scanner-1/scripts/analyze/per-testcase-logs-paranoid/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 PARALLEL TEST (PARANOID Mode)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"

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
    
    # Create temp directory for this scan to avoid JSON conflicts
    local temp_scan_dir="dev-rust-scanner-1/temp_scan_$$_${testname}"
    mkdir -p "$temp_scan_dir"
    
    # Run rust scanner (PARANOID mode) - use absolute path
    cd "$temp_scan_dir"
    local abs_testdir=$(realpath "../../$testdir")
    cargo run --quiet --release --manifest-path ../Cargo.toml -- --paranoid "$abs_testdir" > "../../$logfile" 2>&1
    local exit_code=$?
    
    # Copy JSON output to log directory
    if [ -f "scan_results.json" ]; then
        mv "scan_results.json" "../../$LOG_DIR/rust_${testname}.json"
    fi
    
    cd ../..
    rm -rf "$temp_scan_dir"
    
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
echo "📊 Creating comparison report..."

# Strip ANSI codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

# Create comparison CSV
cat > "$LOG_DIR/comparison.csv" << 'CSVHEADER'
TestCase,Bash_High,Bash_Medium,Bash_Low,Rust_High,Rust_Medium,Rust_Low,Match
CSVHEADER

# Get test cases array
TESTCASES=($(find shai-hulud-detect/test-cases -mindepth 1 -maxdepth 1 -type d | sort))

matched=0
for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    
    # Extract bash numbers with ANSI stripping
    bash_high=$(grep "High Risk Issues:" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    bash_med=$(grep "Medium Risk Issues:" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    bash_low=$(grep "Low Risk" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')
    
    # Extract rust numbers with ANSI stripping
    rust_high=$(grep "High Risk Issues:" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    rust_med=$(grep "Medium Risk Issues:" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    rust_low=$(grep "Low Risk" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')
    
    # Default to 0 for empty values
    bash_high=${bash_high:-0}
    bash_med=${bash_med:-0}
    bash_low=${bash_low:-0}
    rust_high=${rust_high:-0}
    rust_med=${rust_med:-0}
    rust_low=${rust_low:-0}
    
    # Check match
    if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
        match="✅"
        matched=$((matched + 1))
    else
        match="❌"
    fi
    
    echo "$testname,$bash_high,$bash_med,$bash_low,$rust_high,$rust_med,$rust_low,$match" >> "$LOG_DIR/comparison.csv"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 PARANOID MODE - PER-TEST-CASE COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %12s %12s %8s\n" "Test Case" "Bash (H/M/L)" "Rust (H/M/L)" "Match"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Print formatted table from CSV (skip header)
tail -n +2 "$LOG_DIR/comparison.csv" | while IFS=, read -r testname bash_h bash_m bash_l rust_h rust_m rust_l match; do
    printf "%-35s %4s/%2s/%2s      %4s/%2s/%2s    %s\n" "$testname" "$bash_h" "$bash_m" "$bash_l" "$rust_h" "$rust_m" "$rust_l" "$match"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Summary
total_tests=${#TESTCASES[@]}

END_TIME=$(date +%s)
END_READABLE=$(date "+%Y-%m-%d %H:%M:%S")
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo "📈 Match Rate: $matched / $total_tests test cases (PARANOID)"
echo ""
echo "⏱️  TIMING:"
echo "   Started:  $START_READABLE"
echo "   Finished: $END_READABLE"
echo "   Duration: ${MINUTES}m ${SECONDS}s"
echo ""
echo "💾 Results saved: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Pattern-level verification (PARANOID)
echo ""
echo "🔬 Running pattern-level verification (detailed - PARANOID)..."
echo ""

PATTERN_FAILED=0
for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    
    bash_log="$LOG_DIR/bash_${testname}.log"
    rust_json="$LOG_DIR/rust_${testname}.json"
    
    # Skip if no JSON (scan may have failed)
    if [ ! -f "$rust_json" ]; then
        continue
    fi
    
    # Run Python verification (suppress output, only show failures)
    python dev-rust-scanner-1/scripts/verify_pattern_match.py "$bash_log" "$rust_json" > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "⚠️  $testname: Pattern mismatch detected!"
        PATTERN_FAILED=$((PATTERN_FAILED + 1))
    fi
done

if [ $PATTERN_FAILED -eq 0 ]; then
    echo "✅ All test cases passed pattern-level verification (PARANOID)!"
else
    echo "⚠️  $PATTERN_FAILED test case(s) had pattern mismatches"
    echo "   Run verify_pattern_match.py manually on failed cases for details"
fi

echo ""


