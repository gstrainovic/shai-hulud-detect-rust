#!/bin/bash
# FINAL: Exactly count Bash HIGH items

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 EXACT BASH HIGH COUNT BREAKDOWN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

LOG="shai-hulud-detect/bash-testcases.log"

echo "## 1. Compromised Packages (HIGH RISK section):"
grep -A 100 "🚨 HIGH RISK: Compromised package versions detected:" "$LOG" | \
    grep "^   - Package:" | \
    wc -l | xargs echo "   Count:"

echo ""
echo "## 2. Crypto Theft (HIGH RISK section):"
grep -A 100 "🚨 HIGH RISK: Cryptocurrency theft patterns detected:" "$LOG" | \
    grep "HIGH RISK$" | \
    wc -l | xargs echo "   Count:"

echo ""
echo "## 3. Malicious Workflows:"
grep -A 20 "🚨 HIGH RISK: Malicious workflow files detected:" "$LOG" | \
    grep "^   - " | grep -v "NOTE:" | \
    wc -l | xargs echo "   Count:"

echo ""
echo "## 4. Trufflehog:"
grep -A 20 "🚨 HIGH RISK: Trufflehog/secret scanning activity detected:" "$LOG" | \
    grep "^   - " | grep -v "NOTE:" | \
    wc -l | xargs echo "   Count:"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## MANUAL VERIFICATION:"
echo ""
echo "Let me list each HIGH item:"
echo ""

echo "### Compromised Packages:"
grep -A 100 "🚨 HIGH RISK: Compromised package versions detected:" "$LOG" | \
    grep "^   - Package:" | head -10

echo ""
echo "### Crypto HIGH:"
grep -A 100 "🚨 HIGH RISK: Cryptocurrency theft patterns detected:" "$LOG" | \
    grep "HIGH RISK$" | head -10

echo ""
echo "### Workflows:"
grep -A 20 "🚨 HIGH RISK: Malicious workflow files detected:" "$LOG" | \
    grep "^   - " | grep -v "NOTE:" | head -10

echo ""
echo "### Trufflehog:"
grep -A 20 "🚨 HIGH RISK: Trufflehog/secret scanning activity detected:" "$LOG" | \
    grep "^   - " | grep -v "NOTE:" | head -10

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Bash Summary Line:"
grep "High Risk Issues:" "$LOG" | tail -1
