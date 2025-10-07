#!/bin/bash
# Quick paranoid verification - key test cases only

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 PARANOID MODE - QUICK VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

CASES=("comprehensive-test" "infected-project" "typosquatting-project" "network-exfiltration-project")

for name in "${CASES[@]}"; do
    echo "Testing: $name"
    
    # Bash
    bash_result=$(cd shai-hulud-detect && ./shai-hulud-detector.sh --paranoid "test-cases/$name" 2>&1 | grep -E "High Risk|Medium Risk|Low Risk")
    
    # Rust
    rust_result=$(cd dev-rust-scanner-1 && cargo run --quiet --release -- "../shai-hulud-detect/test-cases/$name" --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk")
    
    echo "  Bash:"
    echo "$bash_result" | sed 's/^/    /'
    
    echo "  Rust:"
    echo "$rust_result" | sed 's/^/    /'
    
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total counts (all test-cases):"
echo ""

echo "Bash Paranoid Total:"
tail -20 shai-hulud-detect/paranoid-bash-testcases.log | grep -E "High Risk|Medium Risk|Low Risk"

echo ""
echo "Rust Paranoid Total:"
cd dev-rust-scanner-1 && cargo run --quiet --release -- ../shai-hulud-detect/test-cases --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
