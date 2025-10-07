#!/bin/bash
# Compare what Bash and Rust actually count for HIGH

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 HIGH COUNT COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## BASH METHOD (from code analysis):"
echo "Bash increments high_risk each time it reports a finding."
echo "Let me count how many times 'high_risk++' happens:"
echo ""

# Count findings in each category
echo "### 1. Compromised Packages:"
grep "   - Package:" shai-hulud-detect/bash-testcases.log | wc -l | xargs echo "   Bash shows packages:"

echo ""
echo "### 2. Crypto Theft (HIGH):"
grep "Known attacker wallet address detected - HIGH RISK" shai-hulud-detect/bash-testcases.log | wc -l | xargs echo "   Bash wallet HIGH:"
grep "XMLHttpRequest prototype modification with crypto patterns detected - HIGH RISK" shai-hulud-detect/bash-testcases.log | wc -l | xargs echo "   Bash XMLHttp HIGH:"

echo ""
echo "### 3. Workflow Files:"
grep -A 20 "Malicious workflow files detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | grep -v "NOTE:" | wc -l | xargs echo "   Bash workflow files:"

echo ""
echo "### 4. Trufflehog:"
grep -A 20 "Trufflehog/secret scanning activity detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | grep -v "NOTE:" | wc -l | xargs echo "   Bash trufflehog findings:"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## RUST METHOD:"
echo ""

cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 > /tmp/rust_output.log

echo "### 1. Compromised Packages:"
grep "Contains compromised package version:" /tmp/rust_output.log | wc -l | xargs echo "   Rust shows packages:"

echo ""
echo "### 2. Crypto Theft (HIGH):"
grep "Known attacker wallet" /tmp/rust_output.log | wc -l | xargs echo "   Rust wallet HIGH:"
grep "XMLHttpRequest.*crypto.*HIGH" /tmp/rust_output.log | wc -l | xargs echo "   Rust XMLHttp HIGH:"

echo ""
echo "### 3. Workflow Files:"
grep -A 20 "Malicious workflow files detected:" /tmp/rust_output.log | grep "   - " | wc -l | xargs echo "   Rust workflow files:"

echo ""
echo "### 4. Trufflehog:"
grep -A 20 "Trufflehog/secret scanning activity detected:" /tmp/rust_output.log | grep "   - " | wc -l | xargs echo "   Rust trufflehog findings:"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## TOTALS:"
echo ""
echo "Bash total HIGH: $(grep "High Risk Issues:" ../shai-hulud-detect/bash-testcases.log | tail -1 | awk '{print $NF}')"
echo "Rust total HIGH: $(grep "High Risk Issues:" /tmp/rust_output.log | awk '{print $NF}')"
