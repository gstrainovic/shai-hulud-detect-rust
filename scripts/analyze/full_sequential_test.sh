#!/bin/bash
# Sequential (non-parallel) full test - baseline for comparison
# Runs ALL test cases one-by-one to measure total time

cd /c/Users/gstra/Code/rust-scanner

START_TIME=$(date +%s)
START_READABLE=$(date "+%Y-%m-%d %H:%M:%S")

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="dev-rust-scanner-1/scripts/analyze/sequential-logs/$TIMESTAMP"
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐢 SEQUENTIAL FULL TEST (Baseline for Comparison)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $START_READABLE"
echo "📁 Logs: $LOG_DIR"
echo ""

# Get all test cases
TESTCASES=($(find shai-hulud-detect/test-cases -mindepth 1 -maxdepth 1 -type d | sort))
TOTAL_TESTS=${#TESTCASES[@]}

echo "Found $TOTAL_TESTS test cases"
echo ""

# Phase 1: Bash sequential
echo "🔵 Phase 1: Running Bash scanners sequentially..."
bash_count=0
for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    bash_count=$((bash_count + 1))
    
    echo "  [$bash_count/$TOTAL_TESTS] Testing: $testname (Bash)"
    
    cd shai-hulud-detect
    abs_testdir=$(realpath "../$testdir")
    timeout 300 ./shai-hulud-detector.sh "$abs_testdir" > "../$LOG_DIR/bash_${testname}.log" 2>&1
    cd ..
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/bash_${testname}.log" > "$LOG_DIR/bash_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/bash_${testname}_summary.txt"
done

echo ""
echo "🟢 Phase 2: Running Rust scanners sequentially..."
rust_count=0
for testdir in "${TESTCASES[@]}"; do
    testname=$(basename "$testdir")
    rust_count=$((rust_count + 1))
    
    echo "  [$rust_count/$TOTAL_TESTS] Testing: $testname (Rust)"
    
    cd dev-rust-scanner-1
    abs_testdir=$(realpath "../$testdir")
    cargo run --quiet --release -- "$abs_testdir" > "../$LOG_DIR/rust_${testname}.log" 2>&1
    cd ..
    
    # Extract summary
    grep -E "High Risk Issues:|Medium Risk Issues:|Low Risk.*informational" "$LOG_DIR/rust_${testname}.log" > "$LOG_DIR/rust_${testname}_summary.txt" 2>/dev/null || echo "NO SUMMARY" > "$LOG_DIR/rust_${testname}_summary.txt"
done

# Create comparison
echo ""
echo "📊 Creating comparison..."

strip_ansi() {
    sed 's/\x1b\[[0-9;]*m//g'
}

cat > "$LOG_DIR/comparison.csv" << 'CSVHEADER'
TestCase,Bash_High,Bash_Medium,Bash_Low,Rust_High,Rust_Medium,Rust_Low,Match
CSVHEADER

matched=0
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
        matched=$((matched + 1))
    else
        match="❌"
    fi
    
    echo "$testname,$bash_high,$bash_med,$bash_low,$rust_high,$rust_med,$rust_low,$match" >> "$LOG_DIR/comparison.csv"
done

END_TIME=$(date +%s)
END_READABLE=$(date "+%Y-%m-%d %H:%M:%S")
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 SEQUENTIAL TEST RESULTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "%-35s %12s %12s %8s\n" "Test Case" "Bash (H/M/L)" "Rust (H/M/L)" "Match"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

tail -n +2 "$LOG_DIR/comparison.csv" | while IFS=, read -r testname bash_h bash_m bash_l rust_h rust_m rust_l match; do
    printf "%-35s %4s/%2s/%2s      %4s/%2s/%2s    %s\n" "$testname" "$bash_h" "$bash_m" "$bash_l" "$rust_h" "$rust_m" "$rust_l" "$match"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📈 Match Rate: $matched / $TOTAL_TESTS test cases"
echo ""
echo "⏱️  TIMING:"
echo "   Started:  $START_READABLE"
echo "   Finished: $END_READABLE"
echo "   Duration: ${MINUTES}m ${SECONDS}s"
echo ""
echo "💾 Results saved: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
