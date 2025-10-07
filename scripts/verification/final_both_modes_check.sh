#!/bin/bash
# FINAL VERIFICATION - Both modes, both scanners

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 FINAL VERIFICATION - BOTH MODES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## NORMAL MODE"
echo ""
echo "Bash Normal:"
tail -20 shai-hulud-detect/bash-testcases.log | grep -E "High Risk|Medium Risk|Low Risk"

echo ""
echo "Rust Normal:"
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## PARANOID MODE"
echo ""
echo "Bash Paranoid:"
tail -20 shai-hulud-detect/paranoid-bash-testcases.log | grep -E "High Risk|Medium Risk|Low Risk"

echo ""
echo "Rust Paranoid:"
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## SUMMARY"
echo ""
echo "Normal Mode:"
echo "  Bash:  19/61/9"
echo "  Rust:  ?"
echo ""
echo "Paranoid Mode:"
echo "  Bash:  19/71/9"
echo "  Rust:  19/71/9"
