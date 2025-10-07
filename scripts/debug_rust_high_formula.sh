#!/bin/bash
# Debug exact Rust HIGH counting

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 DEBUGGING RUST HIGH_RISK_COUNT()"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Create debug output
cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 > /tmp/rust_debug.log

echo "According to mod.rs, high_risk_count() sums:"
echo "  - workflow_files.len()"
echo "  - malicious_hashes.len()"
echo "  - compromised_found.len()"
echo "  - postinstall_hooks.len()"
echo "  - shai_hulud_repos.len()"
echo "  - crypto HIGH count"
echo "  - trufflehog HIGH count"
echo ""

echo "Let's count each:"
echo ""

echo "1. Workflows:"
grep -A 20 "Malicious workflow files detected:" /tmp/rust_debug.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "2. Malicious hashes:"
grep -A 20 "Known malicious hashes detected:" /tmp/rust_debug.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "3. Compromised packages (compromised_found.len()):"
echo "   This should be COUNT of package.json files with exact matches"
echo "   Bash: 7 packages from 3 files (chalk-debug-attack, infected-project, lockfile-comprehensive-test, lockfile-compromised)"
echo "   Let me check Rust output..."
grep -A 30 "Compromised package versions detected:" /tmp/rust_debug.log | grep "Found in:" | grep "package.json" | grep -v lockfile | sort -u | wc -l | xargs echo "   Unique package.json files with compromised:"

echo ""
echo "4. Postinstall hooks:"
grep -A 20 "Suspicious postinstall hooks detected:" /tmp/rust_debug.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "5. Shai-hulud repos:"
grep -A 20 "Shai-Hulud repositories detected:" /tmp/rust_debug.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "6. Crypto HIGH (from crypto_patterns with RiskLevel::High):"
grep "HIGH RISK$" /tmp/rust_debug.log | grep -E "wallet|XMLHttpRequest" | wc -l | xargs echo "   "

echo ""
echo "7. Trufflehog HIGH (from trufflehog_activity with RiskLevel::High):"
grep -A 30 "Trufflehog/secret scanning activity detected:" /tmp/rust_debug.log | grep "   - " | wc -l | xargs echo "   "

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TOTAL should be:"
echo "If we add: workflows + hashes + compromised_files + postinstall + repos + crypto_high + truf_high"
