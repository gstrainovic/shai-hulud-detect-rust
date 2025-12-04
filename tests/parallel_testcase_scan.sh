#!/bin/bash
# Parallel per-test-case scanner - supports normal, paranoid, and verify modes
# Usage: ./parallel_testcase_scan.sh [--paranoid] [--verify]

# Parse modes
PARANOID_MODE=""
VERIFY_MODE=""
LOG_SUBDIR="per-testcase-logs"
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
    LOG_SUBDIR="per-testcase-logs-paranoid-verify"
    MODE_LABEL="PARANOID Mode + --verify"
elif [[ -n "$PARANOID_MODE" ]]; then
    LOG_SUBDIR="per-testcase-logs-paranoid"
    MODE_LABEL="PARANOID Mode"
elif [[ -n "$VERIFY_MODE" ]]; then
    LOG_SUBDIR="per-testcase-logs-verify"
    MODE_LABEL="Normal Mode + --verify"
else
    LOG_SUBDIR="per-testcase-logs"
    MODE_LABEL="Normal Mode"
fi

# cd /c/Users/gstra/Code/rust-scanner # REMOVED: Don't hardcode absolute paths

# Get the absolute path of the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TESTCASES_ROOT="$(cd "$PROJECT_ROOT/../shai-hulud-detect/test-cases" && pwd)"

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

# Check for existing logs from today to reuse
TODAY=$(date +%Y%m%d)
# Use absolute path for LOG_DIR
LATEST_LOG_DIR=$(find "$SCRIPT_DIR/$LOG_SUBDIR" -maxdepth 1 -type d -name "${TODAY}_*" 2>/dev/null | sort -r | head -n 1)

if [[ -n "$LATEST_LOG_DIR" ]]; then
    LOG_DIR="$LATEST_LOG_DIR"
    echo "📂 Reusing existing log directory: $LOG_DIR"
    REUSING_LOGS=true
else
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    LOG_DIR="$SCRIPT_DIR/$LOG_SUBDIR/$TIMESTAMP"
    mkdir -p "$LOG_DIR"
    echo "📁 Created new log directory: $LOG_DIR"
    REUSING_LOGS=false
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 PARALLEL TEST ($MODE_LABEL)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs will be in: $LOG_DIR"
echo ""

# Start building Rust scanner in background (can happen during bash scans)
echo "🔨 Building Rust scanner binary in background..."
# cd dev-rust-scanner-1 # REMOVED
cargo build --release --quiet &
BUILD_PID=$!
# cd .. # REMOVED
echo "✅ Build started (PID: $BUILD_PID) - will complete during bash scans"
echo ""

# Function to run bash scanner on a single test case
run_bash_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/bash_${testname}.log"
    local summaryfile="$LOG_DIR/bash_${testname}_summary.txt"
    local exitfile="$LOG_DIR/bash_${testname}_exit.txt"
    
    # Check if log file already exists and appears valid (contains completion markers)
    local skip_scan=false
    if [[ -f "$logfile" && -s "$logfile" && -f "$exitfile" ]]; then
        if grep -q "SUMMARY:" "$logfile" || grep -q "No indicators of Shai-Hulud compromise detected" "$logfile"; then
            skip_scan=true
        fi
    fi

    if [ "$skip_scan" = true ]; then
        echo "⏩ [$(date +%H:%M:%S)] Skipping Bash scan (log exists & valid): $testname"
    else
        if [[ -f "$logfile" ]]; then
            echo "🔄 [$(date +%H:%M:%S)] Re-running Bash scan (log invalid/incomplete): $testname"
        else
            echo "⏳ [$(date +%H:%M:%S)] Starting: $testname"
        fi
        
        # Run bash scanner - use absolute path
        local abs_testdir="$TESTCASES_ROOT/$testname"
        # Execute bash scanner from sibling directory
        # NOTE: --use-grep is required because git-grep searches entire repo instead of specified directory
        timeout 600 "$PROJECT_ROOT/../shai-hulud-detect/shai-hulud-detector.sh" --use-grep "$abs_testdir" $PARANOID_MODE > "$logfile" 2>&1
        local exit_code=$?
        
        # Save exit code to file for comparison
        echo "$exit_code" > "$exitfile"
        
        if [ $exit_code -eq 124 ]; then
            echo "⏱️  [$(date +%H:%M:%S)] TIMEOUT: $testname (>10min)" | tee -a "$logfile"
        elif [ $exit_code -eq 0 ]; then
            echo "✅ [$(date +%H:%M:%S)] Done: $testname (Clean)" 
        elif [ $exit_code -eq 1 ]; then
            echo "✅ [$(date +%H:%M:%S)] Done: $testname (High Risk)" 
        elif [ $exit_code -eq 2 ]; then
            echo "✅ [$(date +%H:%M:%S)] Done: $testname (Medium Risk)" 
        else
            echo "❌ [$(date +%H:%M:%S)] Error: $testname (exit $exit_code)" | tee -a "$logfile"
        fi
    fi
    
    # ALWAYS generate summary from the log file (whether skipped or run)
    if [[ -f "$logfile" ]]; then
        grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$summaryfile" 2>/dev/null || echo "NO SUMMARY" > "$summaryfile"
    fi
}

# Function to run rust scanner on a single test case
run_rust_testcase() {
    local testdir=$1
    local testname=$(basename "$testdir")
    local logfile="$LOG_DIR/rust_${testname}.log"
    local exitfile="$LOG_DIR/rust_${testname}_exit.txt"
    
    echo "⚡ [$(date +%H:%M:%S)] Starting: $testname (Rust)"
    
    # Create temp directory for this scan to avoid JSON conflicts
    local temp_scan_dir=$(mktemp -d)
    
    # Run rust scanner - use pre-built binary with absolute paths
    cd "$temp_scan_dir"
    "$PROJECT_ROOT/target/release/shai-hulud-detector" "$TESTCASES_ROOT/$testname" $PARANOID_MODE $VERIFY_MODE > "$logfile" 2>&1
    local exit_code=$?
    
    # Save exit code to file for comparison
    echo "$exit_code" > "$exitfile"
    
    # Copy JSON output to log directory
    if [ -f "scan_results.json" ]; then
        mv "scan_results.json" "$LOG_DIR/rust_${testname}.json"
    fi
    
    cd "$PROJECT_ROOT"
    rm -rf "$temp_scan_dir"
    
    if [ $exit_code -eq 0 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust, Clean)"
    elif [ $exit_code -eq 1 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust, High Risk)"
    elif [ $exit_code -eq 2 ]; then
        echo "✅ [$(date +%H:%M:%S)] Done: $testname (Rust, Medium Risk)"
    else
        echo "❌ [$(date +%H:%M:%S)] Error: $testname (Rust, exit $exit_code)" | tee -a "$logfile"
    fi
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$logfile" > "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_${testname}_summary.txt"
}

export -f run_bash_testcase
export -f run_rust_testcase
export LOG_DIR
export PARANOID_MODE
export PROJECT_ROOT
export TESTCASES_ROOT

# Get all test case directories using absolute path
TESTCASES=($(find "$TESTCASES_ROOT" -mindepth 1 -maxdepth 1 -type d | sort))

echo "Found ${#TESTCASES[@]} test cases"
echo ""

# Run Bash scans in parallel (CPU-based scaling with 2.25x multiplier for all 25 parallel)
CPU_CORES_RAW=$(nproc 2>/dev/null || echo 4)  # fallback to 4 if nproc unavailable
CPU_CORES=$((CPU_CORES_RAW * 9 / 4))  # 2.25x scaling (integer math: 9/4 = 2.25)
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
echo "🟢 Phase 2: Running Rust scanners in parallel (max $CPU_CORES concurrent - optimal)..."
printf '%s\n' "${TESTCASES[@]}" | xargs -P $CPU_CORES -I {} bash -c 'run_rust_testcase "$@"' _ {}

echo ""
echo "📊 Creating comparison report..."

# Strip ANSI codes
strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

# Create comparison CSV with exit codes
cat > "$LOG_DIR/comparison.csv" << 'CSVHEADER'
TestCase,Bash_Exit,Rust_Exit,Bash_High,Bash_Medium,Bash_Low,Rust_High,Rust_Medium,Rust_Low,Exit_Match,Count_Match
CSVHEADER

for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    
    # Extract exit codes
    bash_exit=$(cat "$LOG_DIR/bash_${testname}_exit.txt" 2>/dev/null || echo "?")
    rust_exit=$(cat "$LOG_DIR/rust_${testname}_exit.txt" 2>/dev/null || echo "?")
    
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
    
    # Check exit code match
    if [ "$bash_exit" = "$rust_exit" ]; then
        exit_match="✅"
    else
        exit_match="❌"
    fi
    
    # Check count match
    if [ "$bash_high" = "$rust_high" ] && [ "$bash_med" = "$rust_med" ] && [ "$bash_low" = "$rust_low" ]; then
        count_match="✅"
    else
        count_match="❌"
    fi
    
    echo "$testname,$bash_exit,$rust_exit,$bash_high,$bash_med,$bash_low,$rust_high,$rust_med,$rust_low,$exit_match,$count_match" >> "$LOG_DIR/comparison.csv"
done

echo ""
echo "✅ Done! Results in: $LOG_DIR"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 PER-TEST-CASE COMPARISON (Exit Codes + Counts)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %6s %6s %12s %12s %6s %6s\n" "Test Case" "B.Exit" "R.Exit" "Bash (H/M/L)" "Rust (H/M/L)" "Exit" "Count"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Print formatted table from CSV (skip header)
tail -n +2 "$LOG_DIR/comparison.csv" | while IFS=, read -r testname bash_exit rust_exit bash_h bash_m bash_l rust_h rust_m rust_l exit_match count_match; do
    printf "%-35s %6s %6s  %4s/%2s/%2s    %4s/%2s/%2s   %s    %s\n" "$testname" "$bash_exit" "$rust_exit" "$bash_h" "$bash_m" "$bash_l" "$rust_h" "$rust_m" "$rust_l" "$exit_match" "$count_match"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Summary - now with exit code stats
total_tests=${#TESTCASES[@]}
exit_matched=$(grep -c "✅" <<< "$(cut -d',' -f10 "$LOG_DIR/comparison.csv" | tail -n +2)")
count_matched=$(grep -c "✅" <<< "$(cut -d',' -f11 "$LOG_DIR/comparison.csv" | tail -n +2)")

END_TIME=$(date +%s)
END_READABLE=$(date "+%Y-%m-%d %H:%M:%S")
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo "📈 Match Rate:"
echo "   Exit Codes: $exit_matched / $total_tests"
echo "   H/M/L Counts: $count_matched / $total_tests"
echo ""
echo "⏱️  TIMING:"
echo "   Started:  $START_READABLE"
echo "   Finished: $END_READABLE"
echo "   Duration: ${MINUTES}m ${SECONDS}s"
echo ""
echo "💾 Results saved: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Pattern-level verification for test cases with findings
echo ""
echo "🔬 Running pattern-level verification (nom-based parser)..."
echo ""

# Build bash-log-parser once
echo "🔨 Building bash-log-parser..."
cd "$PROJECT_ROOT/bash-log-parser"
cargo build --release --quiet
if [ $? -ne 0 ]; then
    echo "❌ Failed to build bash-log-parser!"
    exit 1
fi
cd "$PROJECT_ROOT"

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
    verification_output=$("$PROJECT_ROOT/bash-log-parser/target/release/bash-log-parser" "$bash_log" "$rust_json" 2>&1)
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
    
    # Check if perfect match (exit 0) or pattern mismatch (exit != 0)
    if [ $verification_exit -ne 0 ]; then
        echo "⚠️  $testname: Pattern mismatch detected!"
        # PR #50 merged - no special webhook.site handling needed anymore
        PATTERN_FAILED=$((PATTERN_FAILED + 1))
    else
        if [ $bash_count -eq 0 ]; then
            echo "✅ $testname: Perfect match (0 findings)"
        else
            echo "✅ $testname: Perfect match ($bash_count findings)"
        fi
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
    echo "⚠️  $PATTERN_FAILED test case(s) had pattern mismatches"
    echo "   Run bash-log-parser manually on failed cases for details"
fi

echo ""

# Final exit code based on all checks
if [ "$exit_matched" -ne "$total_tests" ] || [ "$count_matched" -ne "$total_tests" ] || [ $PATTERN_FAILED -gt 0 ]; then
    echo "❌ TESTS FAILED!"
    echo "   Exit Code Matches: $exit_matched / $total_tests"
    echo "   Count Matches: $count_matched / $total_tests"
    echo "   Pattern Matches: $((PATTERN_TOTAL - PATTERN_FAILED)) / $PATTERN_TOTAL"
    exit 1
else
    echo "✅ ALL TESTS PASSED!"
    exit 0
fi
