#!/bin/bash
# Parallel per-test-case Bash scanner with detailed logging
# This creates baseline data for each test case subfolder

cd /c/Users/gstra/Code/rust-scanner

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="dev-rust-scanner-1/scripts/analyze/per-testcase-logs/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 PARALLEL TEST (Normal Mode)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs will be in: $LOG_DIR"
echo ""

# Build Rust scanner binary once at the start
echo "🔨 Building Rust scanner binary..."
cd dev-rust-scanner-1
cargo build --release --quiet
if [ $? -ne 0 ]; then
    echo "❌ Failed to build Rust scanner!"
    exit 1
fi
cd ..
echo "✅ Binary built: dev-rust-scanner-1/target/release/shai-hulud-detector"
echo ""

# Function to run bash scanner on a single test case
run_bash_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/bash_${testname}.log"
    
    echo "⏳ [$(date +%H:%M:%S)] Starting: $testname"
    
    # Run bash scanner (normal mode) - use absolute path
    cd shai-hulud-detect
    local abs_testdir=$(realpath "../$testdir")
    timeout 300 ./shai-hulud-detector.sh "$abs_testdir" > "../$logfile" 2>&1
    local exit_code=$?
    cd ..
    
    if [ $exit_code -eq 124 ]; then
        echo "⏱️  [$(date +%H:%M:%S)] TIMEOUT: $testname (>5min)" | tee -a "$logfile"
    elif [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname" 
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (exit $exit_code)" | tee -a "$logfile"
    fi
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_${testname}_summary.txt"
}

# Function to run rust scanner on a single test case
run_rust_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/rust_${testname}.log"
    
    echo "⚡ [$(date +%H:%M:%S)] Starting: $testname (Rust)"
    
    # Create temp directory for this scan to avoid JSON conflicts
    local temp_scan_dir="dev-rust-scanner-1/temp_scan_$$_${testname}"
    mkdir -p "$temp_scan_dir"
    
    # Run rust scanner (normal mode) - use pre-built binary
    cd "$temp_scan_dir"
    local abs_testdir=$(realpath "../../$testdir")
    ../target/release/shai-hulud-detector "$abs_testdir" > "../../$logfile" 2>&1
    local exit_code=$?
    
    # Copy JSON output to log directory
    if [ -f "scan_results.json" ]; then
        mv "scan_results.json" "../../$LOG_DIR/rust_${testname}.json"
    fi
    
    cd ../..
    rm -rf "$temp_scan_dir"
    
    if [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust)"
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (Rust, exit $exit_code)" | tee -a "$logfile"
    fi
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_${testname}_summary.txt"
}

export -f run_bash_testcase
export -f run_rust_testcase
export LOG_DIR

# Get all test case directories
TESTCASES=($(find shai-hulud-detect/test-cases -mindepth 1 -maxdepth 1 -type d | sort))

echo "Found ${#TESTCASES[@]} test cases"
echo ""

# Run Bash scans in parallel (CPU-based scaling with 2.25x multiplier for all 25 parallel)
CPU_CORES_RAW=$(nproc 2>/dev/null || echo 4)  # fallback to 4 if nproc unavailable
CPU_CORES=$((CPU_CORES_RAW * 9 / 4))  # 2.25x scaling (integer math: 9/4 = 2.25)
echo "🔵 Phase 1: Running Bash scanners in parallel (max $CPU_CORES concurrent)..."
printf '%s\n' "${TESTCASES[@]}" | xargs -P $CPU_CORES -I {} bash -c 'run_bash_testcase "$@"' _ {}

echo ""
echo "🟢 Phase 2: Running Rust scanners in parallel (max $CPU_CORES concurrent - optimal)..."
printf '%s\n' "${TESTCASES[@]}" | xargs -P $CPU_CORES -I {} bash -c 'run_rust_testcase "$@"' _ {}

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
    else
        match="❌"
    fi
    
    echo "$testname,$bash_high,$bash_med,$bash_low,$rust_high,$rust_med,$rust_low,$match" >> "$LOG_DIR/comparison.csv"
done

echo ""
echo "✅ Done! Results in: $LOG_DIR"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "� PER-TEST-CASE COMPARISON"
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
matched=$(grep "✅" "$LOG_DIR/comparison.csv" | wc -l)

END_TIME=$(date +%s)
END_READABLE=$(date "+%Y-%m-%d %H:%M:%S")
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo "📈 Match Rate: $matched / $total_tests test cases"
echo ""
echo "⏱️  TIMING:"
echo "   Started:  $START_READABLE"
echo "   Finished: $END_READABLE"
echo "   Duration: ${MINUTES}m ${SECONDS}s"
echo ""
echo "💾 Results saved: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Pattern-level verification for test cases with findings
echo ""
echo "🔬 Running pattern-level verification (nom-based parser)..."
echo ""

# Build bash-log-parser once
echo "🔨 Building bash-log-parser..."
cd dev-rust-scanner-1/bash-log-parser
cargo build --release --quiet
if [ $? -ne 0 ]; then
    echo "❌ Failed to build bash-log-parser!"
    exit 1
fi
cd ../../

PATTERN_FAILED=0
PATTERN_TOTAL=0
TOTAL_BASH_FINDINGS=0
TOTAL_RUST_FINDINGS=0
TOTAL_MATCHES=0

for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    
    bash_log="$LOG_DIR/bash_${testname}.log"
    rust_json="$LOG_DIR/rust_${testname}.json"
    
    # Skip if no JSON (scan may have failed)
    if [ ! -f "$rust_json" ]; then
        continue
    fi
    
    PATTERN_TOTAL=$((PATTERN_TOTAL + 1))
    
    # Run Rust bash-log-parser verification
    verification_output=$(dev-rust-scanner-1/bash-log-parser/target/release/bash-log-parser "$bash_log" "$rust_json" 2>&1)
    verification_exit=$?
    
    # Extract findings counts with defaults
    bash_count=$(echo "$verification_output" | grep "Bash:" | grep -o '[0-9]\+' | head -1)
    rust_count=$(echo "$verification_output" | grep "Rust:" | grep -o '[0-9]\+' | head -1)
    matches=$(echo "$verification_output" | grep "✓ Matches:" | grep -o '[0-9]\+' | head -1)
    
    # Default to 0 if empty
    bash_count=${bash_count:-0}
    rust_count=${rust_count:-0}
    matches=${matches:-0}
    
    TOTAL_BASH_FINDINGS=$((TOTAL_BASH_FINDINGS + bash_count))
    TOTAL_RUST_FINDINGS=$((TOTAL_RUST_FINDINGS + rust_count))
    TOTAL_MATCHES=$((TOTAL_MATCHES + matches))
    
    if [ $verification_exit -ne 0 ] || echo "$verification_output" | grep -q "⚠️"; then
        echo "⚠️  $testname: B:$bash_count R:$rust_count M:$matches"
        PATTERN_FAILED=$((PATTERN_FAILED + 1))
    else
        echo "✅ $testname: Perfect match ($matches findings)"
    fi
done

echo ""
echo "📊 VERIFICATION SUMMARY:"
echo "   Test Cases: $PATTERN_TOTAL"
echo "   Perfect Matches: $((PATTERN_TOTAL - PATTERN_FAILED))"
echo "   Issues: $PATTERN_FAILED"
echo ""
echo "📈 FINDINGS TOTALS:"
echo "   Bash Findings: $TOTAL_BASH_FINDINGS"
echo "   Rust Findings: $TOTAL_RUST_FINDINGS" 
echo "   Matches: $TOTAL_MATCHES"
if [ $TOTAL_BASH_FINDINGS -gt 0 ]; then
    MATCH_RATE=$((TOTAL_MATCHES * 100 / TOTAL_BASH_FINDINGS))
    echo "   Overall Match Rate: $MATCH_RATE%"
fi

if [ $PATTERN_FAILED -eq 0 ]; then
    echo ""
    echo "🎉 ALL TEST CASES ACHIEVED 100% FINDING-LEVEL VERIFICATION!"
else
    echo ""
    echo "⚠️  $PATTERN_FAILED test case(s) had finding mismatches"
    echo "   Run bash-log-parser manually on failed cases for details"
fi

echo ""


