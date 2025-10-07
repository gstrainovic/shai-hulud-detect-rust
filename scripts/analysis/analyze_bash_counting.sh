#!/bin/bash
# Find what Bash counts that gives 19 HIGH

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 BASH HIGH COUNTING LOGIC"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## Bash log shows these HIGH RISK sections:"
grep "^🚨 HIGH RISK:" shai-hulud-detect/bash-testcases.log

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Counting items under each section:"
echo ""

echo "### Compromised packages:"
grep -A 100 "Compromised package versions detected:" shai-hulud-detect/bash-testcases.log | grep "   - Package:" | wc -l

echo ""
echo "### Crypto theft:"
grep -A 100 "Cryptocurrency theft patterns detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | head -20

echo ""
echo "### Workflow files:"
grep -A 20 "Malicious workflow files detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | wc -l

echo ""
echo "### Trufflehog:"
grep -A 20 "Trufflehog/secret scanning activity detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | wc -l

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Let me find the EXACT lines Bash counts:"
echo ""

# Extract just the findings list
echo "Compromised packages (should be 7):"
grep -A 20 "Compromised package versions detected:" shai-hulud-detect/bash-testcases.log | grep "   - Package:" | sed 's/   - Package: //'

echo ""
echo "Crypto findings:"
grep -A 20 "Cryptocurrency theft patterns detected:" shai-hulud-detect/bash-testcases.log | grep "   - " | wc -l | xargs echo "Count:"
