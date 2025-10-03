#!/bin/bash
# Script: final_comprehensive_test.sh
# Purpose: Finaler umfassender Test ALLER test-cases

BASH_SCRIPT="C:/Users/gstra/Code/shai-hulud-detect/shai-hulud-detector.sh"
RUST_DIR="C:/Users/gstra/Code/rust-scanner-final"
TEST_CASES_DIR="C:/Users/gstra/Code/shai-hulud-detect/test-cases"

cd "$RUST_DIR" || exit 1
cp "C:/Users/gstra/Code/shai-hulud-detect/compromised-packages.txt" . 2>/dev/null

echo "=== FINAL COMPREHENSIVE TEST ==="
echo ""

TOTAL=0
IDENTICAL=0
FAILED_CASES=""

for test_case in "$TEST_CASES_DIR"/*; do
    [ -d "$test_case" ] || continue
    
    test_name=$(basename "$test_case")
    
    # Bash
    BASH_OUT=$(bash "$BASH_SCRIPT" "$test_case" 2>&1)
    bash_medium=$(echo "$BASH_OUT" | awk '/Medium Risk Issues:/ {print $NF}' | tr -d '\r')
    [ -z "$bash_medium" ] && bash_medium=0
    
    # Rust
    RUST_OUT=$(cargo run --quiet -- "$test_case" 2>&1)
    rust_medium=$(echo "$RUST_OUT" | awk '/Medium Risk Issues:/ {print $NF}' | tr -d '\r')
    [ -z "$rust_medium" ] && rust_medium=0
    
    if [ "$bash_medium" = "$rust_medium" ]; then
        echo "✅ $test_name"
        IDENTICAL=$((IDENTICAL + 1))
    else
        echo "❌ $test_name: Bash=$bash_medium Rust=$rust_medium"
        FAILED_CASES="$FAILED_CASES\n  - $test_name: Bash=$bash_medium Rust=$rust_medium"
    fi
    
    TOTAL=$((TOTAL + 1))
done

echo ""
echo "=== ERGEBNIS ==="
echo "Identisch: $IDENTICAL / $TOTAL"
echo "Fehlgeschlagen: $((TOTAL - IDENTICAL))"

if [ "$IDENTICAL" = "$TOTAL" ]; then
    echo ""
    echo "🎉🎉🎉 100% PARITÄT ERREICHT! 🎉🎉🎉"
else
    echo ""
    echo "Verbleibende Unterschiede:"
    echo -e "$FAILED_CASES"
fi
