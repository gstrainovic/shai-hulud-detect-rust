#!/bin/bash
# Wrapper for verify mode (normal + --verify) - calls main script with --verify flag
exec "$(dirname "${BASH_SOURCE[0]}")/parallel_testcase_scan.sh" --verify "$@"

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="tests/per-testcase-logs-verify/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 PARALLEL TEST (Normal Mode + --verify)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs will be in: $LOG_DIR"
echo ""

# Start building Rust scanner in background
echo "🔨 Building Rust scanner binary in background..."
# cd dev-rust-scanner-1 # REMOVED
cargo build --release --quiet &
BUILD_PID=$!
# cd .. # REMOVED
echo "✅ Build started (PID: $BUILD_PID) - will complete during bash scans"
echo ""

# Find test cases
TESTCASES=($(find ../shai-hulud-detect/test-cases -mindepth 1 -maxdepth 1 -type d | sort))

export -f run_bash_testcase
export -f run_rust_testcase
export LOG_DIR

echo "Found ${#TESTCASES[@]} test cases"
echo ""

# Function to run bash scanner on a single test case (NO CHANGES - for comparison)
run_bash_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/bash_${testname}.log"
    
    echo "⏳ [$(date +%H:%M:%S)] Starting: $testname"
    
    # cd shai-hulud-detect # REMOVED
    local abs_testdir=$(realpath "../shai-hulud-detect/test-cases/$testname")
    # NOTE: --use-grep is required because git-grep searches entire repo instead of specified directory
    timeout 300 ../shai-hulud-detect/shai-hulud-detector.sh --use-grep "$abs_testdir" > "$logfile" 2>&1
    local exit_code=$?
    # cd .. # REMOVED
    
    if [ $exit_code -eq 124 ]; then
        echo "⏱️  [$(date +%H:%M:%S)] TIMEOUT: $testname (>5min)" | tee -a "$logfile"
    elif [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname" 
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (exit $exit_code)" | tee -a "$logfile"
    fi
    
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_${testname}_summary.txt"
}

# Function to run rust scanner WITH --verify flag
run_rust_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/rust_${testname}.log"
    
    echo "⚡ [$(date +%H:%M:%S)] Starting: $testname (Rust + --verify)"
    
    local temp_scan_dir="temp_scan_$$_${testname}"
    mkdir -p "$temp_scan_dir"
    
    cd "$temp_scan_dir"
    local abs_testdir=$(realpath "../../shai-hulud-detect/test-cases/$testname")
    # KEY CHANGE: Add --verify flag
    ../../target/release/shai-hulud-detector --verify "$abs_testdir" > "../../$logfile" 2>&1
    local exit_code=$?
    
    if [ -f "scan_results.json" ]; then
        mv "scan_results.json" "../../$LOG_DIR/rust_${testname}.json"
    fi
    
    cd ../..
    rm -rf "$temp_scan_dir"
    
    if [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust + --verify)"
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (Rust + --verify, exit $exit_code)" | tee -a "$logfile"
    fi
    
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_${testname}_summary.txt"
}

# Run Bash scans in parallel
CPU_CORES_RAW=$(nproc 2>/dev/null || echo 4)
CPU_CORES=$((CPU_CORES_RAW * 9 / 4))
echo "🔵 Phase 1: Running Bash scanners in parallel (max $CPU_CORES concurrent)..."
printf '%s\n' "${TESTCASES[@]}" | xargs -P $CPU_CORES -I {} bash -c 'run_bash_testcase "$@"' _ {}

echo ""
echo "⏳ Waiting for Rust build to complete..."
wait $BUILD_PID
BUILD_EXIT=$?
if [ $BUILD_EXIT -ne 0 ]; then
    echo "❌ Rust build failed with exit code $BUILD_EXIT!"
    exit 1
fi
echo "✅ Rust binary ready: target/release/shai-hulud-detector"
echo ""
echo "🟢 Phase 2: Running Rust scanners WITH --verify (max $CPU_CORES concurrent)..."
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
    
    bash_high=$(grep "High Risk Issues:" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    bash_med=$(grep "Medium Risk Issues:" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    bash_low=$(grep "Low Risk" "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')
    
    rust_high=$(grep "High Risk Issues:" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    rust_med=$(grep "Medium Risk Issues:" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | strip_ansi | awk '{print $NF}' | tr -d ' ')
    rust_low=$(grep "Low Risk" "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null | grep "informational" | strip_ansi | awk '{print $NF}' | tr -d ' ')
    
    bash_high=${bash_high:-0}
    bash_med=${bash_med:-0}
    bash_low=${bash_low:-0}
    rust_high=${rust_high:-0}
    rust_med=${rust_med:-0}
    rust_low=${rust_low:-0}
    
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
echo "📊 PER-TEST-CASE COMPARISON (Bash vs Rust+--verify)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %12s %12s %8s\n" "Test Case" "Bash (H/M/L)" "Rust (H/M/L)" "Match"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

tail -n +2 "$LOG_DIR/comparison.csv" | while IFS=, read -r testname bash_h bash_m bash_l rust_h rust_m rust_l match; do
    printf "%-35s %4s/%2s/%2s      %4s/%2s/%2s    %s\n" "$testname" "$bash_h" "$bash_m" "$bash_l" "$rust_h" "$rust_m" "$rust_l" "$match"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

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
echo ""

# Pattern-level verification with bash-log-parser
echo "🔬 Running pattern-level verification (nom-based parser)..."
echo ""
echo "🔨 Building bash-log-parser..."
# Adjusted path
cd bash-log-parser
cargo build --release --quiet 2>/dev/null
cd ..

for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    
    bash_log="$LOG_DIR/bash_${testname}.log"
    rust_json="$LOG_DIR/rust_${testname}.json"
    
    if [ -f "$bash_log" ] && [ -f "$rust_json" ]; then
        # Adjusted path
        result=$(bash-log-parser/target/release/bash-log-parser "$bash_log" "$rust_json" 2>&1 || true)
        
        if echo "$result" | grep -q "Perfect match"; then
            findings=$(echo "$result" | grep "findings" | head -1 | awk '{print $1}')
            echo "✅ $testname: Perfect match ($findings findings)"
        elif echo "$result" | grep -q "MISMATCH"; then
            echo "❌ $testname: Mismatch detected!"
            echo "$result" | head -5
        else
            echo "⚠️  $testname: Could not verify"
        fi
    fi
done

echo ""
bash_total=$(find "$LOG_DIR" -name "bash_*.log" -exec grep -h "High Risk Issues:\|Medium Risk Issues:\|Low Risk.*informational" {} \; 2>/dev/null | awk '{sum+=$NF} END {print sum}')
rust_total=$(find "$LOG_DIR" -name "rust_*.json" -exec jq -r '.high_risk_count + .medium_risk_count + .low_risk_count' {} \; 2>/dev/null | awk '{sum+=$1} END {print sum}')

echo "📊 VERIFICATION SUMMARY:"
echo "   Test Cases: ${#TESTCASES[@]}"
echo "   Perfect Matches: $matched"
echo "   Issues: $((${#TESTCASES[@]} - matched))"
echo ""
echo "📈 FINDINGS TOTALS:"
echo "   Bash Findings: ${bash_total:-0}"
echo "   Rust Findings: ${rust_total:-0}"
echo "   Match Rate: $(awk "BEGIN {if (${bash_total:-0} > 0) printf \"%.0f%%\", (${rust_total:-0}/${bash_total:-0})*100; else print \"N/A\"}")"
echo ""

if [ "$matched" -eq "${#TESTCASES[@]}" ]; then
    echo "🎉 ALL TEST CASES ACHIEVED 100% VERIFICATION WITH --verify FLAG!"
else
    echo "⚠️  Some test cases have mismatches. Review logs for details."
fi
echo ""
