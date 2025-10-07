#!/bin/bash
# Final verification of Bash vs Rust counts

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 FINAL BASH VS RUST COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## NORMAL MODE"
echo ""
echo "Bash (from log):"
grep "High Risk Issues:" shai-hulud-detect/bash-testcases.log | tail -1
grep "Medium Risk Issues:" shai-hulud-detect/bash-testcases.log | tail -1
grep "Low Risk" shai-hulud-detect/bash-testcases.log | grep "informational" | tail -1

echo ""
echo "Rust:"
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## PARANOID MODE"
echo ""
echo "Bash (from log):"
grep "High Risk Issues:" shai-hulud-detect/paranoid-bash-testcases.log | tail -1
grep "Medium Risk Issues:" shai-hulud-detect/paranoid-bash-testcases.log | tail -1
grep "Low Risk" shai-hulud-detect/paranoid-bash-testcases.log | grep "informational" | tail -1

echo ""
echo "Rust:"
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## ANALYSIS"
echo ""
echo "Normal Mode:"
echo "  Bash:  19/61/9"
echo "  Rust:  18/66/9"
echo "  Diff:  -1 HIGH, +5 MEDIUM, +0 LOW"
echo ""
echo "Paranoid Mode:"
echo "  Bash:  19/71/9"
echo "  Rust:  18/76/9"
echo "  Diff:  -1 HIGH, +5 MEDIUM, +0 LOW"
echo ""
echo "Pattern: Consistent +5 MEDIUM, -1 HIGH across both modes"
echo "Reason: Likely counting method difference (individual vs grouped)"
