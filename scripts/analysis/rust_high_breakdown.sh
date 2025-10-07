#!/bin/bash
# Final count: Rust HIGH breakdown

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 RUST HIGH COUNT BREAKDOWN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 > /tmp/rust_full_output.log

echo "## 1. Workflows:"
grep -c "Known malicious workflow" /tmp/rust_full_output.log | xargs echo "   Count:"

echo ""
echo "## 2. Compromised Packages (package.json only):"
grep -A 50 "Compromised package versions detected:" /tmp/rust_full_output.log | \
    grep "^   - Package:" | \
    wc -l | xargs echo "   Count:"

echo ""
echo "## 3. Crypto HIGH:"
grep "Known attacker wallet" /tmp/rust_full_output.log | grep "HIGH RISK" | wc -l | xargs echo "   Wallet HIGH:"
grep "XMLHttpRequest.*crypto.*HIGH" /tmp/rust_full_output.log | wc -l | xargs echo "   XMLHttp HIGH:"

echo ""
echo "## 4. Trufflehog:"
grep "Trufflehog binary found" /tmp/rust_full_output.log | wc -l | xargs echo "   Binary:"
grep "Credential patterns with potential exfiltration" /tmp/rust_full_output.log | wc -l | xargs echo "   Credential:"
grep "Environment scanning with exfiltration" /tmp/rust_full_output.log | wc -l | xargs echo "   Environment:"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## RUST TOTAL:"
grep "High Risk Issues:" /tmp/rust_full_output.log

echo ""
echo "## EXPECTED (to match Bash 19):"
echo "   Workflows: 1"
echo "   Packages: 7"
echo "   Crypto: 7"
echo "   Trufflehog: 4"
echo "   Total: 19"
