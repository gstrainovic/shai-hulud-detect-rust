#!/bin/bash
# Summary: Compare ONLY the totals for paranoid mode

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 PARANOID MODE SUMMARY COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "BASH PARANOID (from log):"
tail -10 shai-hulud-detect/paranoid-bash-testcases.log | grep -E "High Risk|Medium Risk|Low Risk"

echo ""
echo "RUST PARANOID (fresh run):"
cd dev-rust-scanner-1
./target/release/shai-hulud-detector.exe ../shai-hulud-detect/test-cases --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Match? (19/71/9 expected for both)"
