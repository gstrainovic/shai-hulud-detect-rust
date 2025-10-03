#!/bin/bash
# Script: detailed_comparison.sh
# Purpose: Detaillierter Vergleich zwischen Bash und Rust Scanner

BASH_SCRIPT="C:/Users/gstra/Code/shai-hulud-detect/shai-hulud-detector.sh"
RUST_BIN="C:/Users/gstra/Code/rust-scanner-final/target/debug/shai-hulud-detector.exe"
TEST_CASE="C:/Users/gstra/Code/shai-hulud-detect/test-cases/infected-project"
OUTPUT_DIR="C:/Users/gstra/Code/rust-scanner-final"

echo "=== Detailed Comparison: Bash vs Rust ==="
echo ""

echo "Running Bash scanner..."
bash "$BASH_SCRIPT" "$TEST_CASE" > "$OUTPUT_DIR/bash_output.txt" 2>&1

echo "Running Rust scanner..."
"$RUST_BIN" "$TEST_CASE" > "$OUTPUT_DIR/rust_output.txt" 2>&1

echo ""
echo "=== Bash Findings Summary ==="
grep -E "HIGH RISK:|MEDIUM RISK:" "$OUTPUT_DIR/bash_output.txt" | sort | uniq -c

echo ""
echo "=== Rust Findings Summary ==="
grep -E "HIGH RISK:|MEDIUM RISK:" "$OUTPUT_DIR/rust_output.txt" | sort | uniq -c

echo ""
echo "=== Missing in Rust (found in Bash but not Rust) ==="
echo "1. Crypto patterns - TODO: Implement crypto detector fully"
echo "2. Trufflehog activity - TODO: Implement trufflehog detector fully"
echo "3. Postinstall hooks - TODO: Check why not detected"
echo ""
echo "Files created:"
echo "  - $OUTPUT_DIR/bash_output.txt"
echo "  - $OUTPUT_DIR/rust_output.txt"
