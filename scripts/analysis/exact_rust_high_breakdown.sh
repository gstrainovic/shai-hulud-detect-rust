#!/bin/bash
# Get EXACT Rust HIGH counts per category

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 EXACT RUST HIGH BREAKDOWN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 > /tmp/rust_exact.log

echo "1. Workflows:"
grep -A 10 "Malicious workflow files detected:" /tmp/rust_exact.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "2. Compromised Packages:"
grep -A 30 "Compromised package versions detected:" /tmp/rust_exact.log | grep "   - Package:" | wc -l | xargs echo "   "

echo ""
echo "3. Crypto HIGH (lines with 'HIGH RISK' at end):"
grep -A 50 "Cryptocurrency theft patterns detected:" /tmp/rust_exact.log | grep "HIGH RISK$" | wc -l | xargs echo "   "

echo ""
echo "4. Trufflehog HIGH:"
grep -A 30 "Trufflehog/secret scanning activity detected:" /tmp/rust_exact.log | grep "   - Activity:" | wc -l | xargs echo "   "

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TOTAL = 1 + 7 + 7 + 4 = 19 (expected)"
echo ""
echo "Rust summary says:"
grep "High Risk Issues:" /tmp/rust_exact.log
