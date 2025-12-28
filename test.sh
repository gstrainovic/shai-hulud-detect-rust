#!/bin/bash
# test.sh - Parallel execution of all run_full_*.sh scripts
# Runs each run_full_*.sh in parallel, creates individual logs,
# and evaluates results at the end.

set -e

# Ask user about deleting test logs in tests/ to speed up tests
echo "Do you want to delete existing test logs in tests/ to speed up tests, or keep them? (d/k): "
read -r choice
if [[ "$choice" == "d" ]]; then
    find tests/ -type d -name "*-logs*" -exec rm -rf {} + 2>/dev/null || true
    echo "Deleted existing log directories in tests/."
else
    echo "Keeping existing log directories in tests/."
fi
echo ""

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

LOG_DIR="test-logs"
mkdir -p "$LOG_DIR"

# Get all run_full_*.sh scripts
TEST_SCRIPTS=($(find tests -name "run_full_*.sh" | sort))

echo "Found ${#TEST_SCRIPTS[@]} run_full_*.sh scripts:"
printf '%s\n' "${TEST_SCRIPTS[@]}"
echo ""

# Function to run a single test script
run_test_script() {
    local script_path=$1
    local script_name=$(basename "$script_path" .sh)
    local log_file="$LOG_DIR/${script_name}.log"
    
    echo "🔍 [$(date +%H:%M:%S)] Starting: $script_name"
    
    # Run the script and capture output while showing it
    # Use pipefail to ensure we catch the exit code of the script, not tee
    set -o pipefail
    if bash "$script_path" 2>&1 | tee "$log_file"; then
        echo "✅ [$(date +%H:%M:%S)] Completed: $script_name"
    else
        local exit_code=$?
        echo "❌ [$(date +%H:%M:%S)] Failed (exit $exit_code): $script_name"
        echo "Exit code: $exit_code" >> "$log_file"
    fi
    set +o pipefail
}

export -f run_test_script
export LOG_DIR

# Run scripts sequentially to avoid resource contention
# (The suites themselves already parallelize the test cases internally, using 100% CPU)
echo "🚀 Running ${#TEST_SCRIPTS[@]} test suites sequentially..."
echo ""

# Execute sequentially (-P 1)
printf '%s\n' "${TEST_SCRIPTS[@]}" | xargs -P 1 -I {} bash -c 'run_test_script "$@"' _ {}

echo ""
echo "📊 Evaluating results..."
echo ""

# Function to extract summary from a log file
extract_summary() {
    local log_file=$1
    local script_name=$(basename "$log_file" .log)
    
    # Check for success/failure patterns
    # IMPORTANT: Check failures FIRST - a log can have both ✅ and ❌
    local status="UNKNOWN"
    local details=""
    
    if grep -q "FAILURE\|❌ FAILURE\|MISMATCH" "$log_file"; then
        status="FAILED"
        details=$(grep -o "FAILURE.*\|❌.*\|MISMATCH.*" "$log_file" | head -1 || echo "")
    elif grep -q "Exit code: [1-9]" "$log_file"; then
        status="FAILED"
        details="Exit code: $(grep "Exit code:" "$log_file" | tail -1)"
    elif grep -q "All tests passed\|🎉 ALL TEST\|100% FINDING-LEVEL\|🎉 FULL VERIFICATION PASSED!\|🎉 SUCCESS!\|✅ SEQUENTIAL.*VERIFICATION COMPLETE" "$log_file"; then
        status="PASSED"
        details=$(grep -o "All tests passed\|🎉.*\|100%.*" "$log_file" | head -1 || echo "")
    elif grep -q "Match Rate: 33 / 33\|Perfect Matches: 33" "$log_file"; then
        status="PASSED"
        details="33/33 test cases matched"
    else
        # Check for test counts or other indicators
        local passed=$(grep -o "[0-9]* passed" "$log_file" | grep -o "[0-9]*" | head -1 || echo "")
        local failed=$(grep -o "[0-9]* failed" "$log_file" | grep -o "[0-9]*" | head -1 || echo "")
        # Default to 0 if empty
        passed=${passed:-0}
        failed=${failed:-0}
        if [ "$passed" -gt 0 ] && [ "$failed" -eq 0 ]; then
            status="PASSED"
            details="$passed passed"
        elif [ "$failed" -gt 0 ]; then
            status="FAILED"
            details="$failed failed"
        fi
    fi
    
    echo "$script_name|$status|$details"
}

# Collect all summaries
echo "Test Script | Status | Details"
echo "------------|--------|--------"

passed_count=0
failed_count=0

for log_file in "$LOG_DIR"/*.log; do
    if [ -f "$log_file" ]; then
        summary=$(extract_summary "$log_file")
        IFS='|' read -r name status details <<< "$summary"
        
        printf "%-11s | %-6s | %s\n" "$name" "$status" "$details"
        
        if [ "$status" = "PASSED" ]; then
            passed_count=$((passed_count + 1))
        elif [ "$status" = "FAILED" ]; then
            failed_count=$((failed_count + 1))
        fi
    fi
done

echo ""
echo "📈 SUMMARY:"
echo "   Total Scripts: ${#TEST_SCRIPTS[@]}"
echo "   Passed: $passed_count"
echo "   Failed: $failed_count"
echo ""
echo "💾 Logs saved in: $LOG_DIR"
echo ""

# Check for failures
if [ "$failed_count" -gt 0 ]; then
    echo "⚠️  $failed_count script(s) failed - check logs for details"
    exit 1
else
    echo "✅ All scripts completed successfully"
fi