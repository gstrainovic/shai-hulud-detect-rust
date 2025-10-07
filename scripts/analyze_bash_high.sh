#!/bin/bash
# Analyze Bash log to understand HIGH count method

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 ANALYZING BASH HIGH COUNT METHOD"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## Method 1: Count compromised package lines"
grep -c "Contains compromised package version:" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 2: Count crypto theft findings"
grep -c "Known attacker wallet address detected" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 3: Count XMLHttpRequest modifications"
grep -c "XMLHttpRequest prototype modification" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 4: Count workflow files"
grep -c "Known malicious workflow filename" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 5: Count trufflehog"
grep -c "Trufflehog binary found" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 6: Count credential patterns"
grep -c "Credential patterns with potential exfiltration" shai-hulud-detect/bash-testcases.log

echo ""
echo "## Method 7: Count environment scanning"
grep -c "Environment scanning with exfiltration" shai-hulud-detect/bash-testcases.log

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Bash reports total HIGH as:"
grep "High Risk Issues:" shai-hulud-detect/bash-testcases.log | tail -1

echo ""
echo "## Let's count all 'Context: HIGH RISK:' lines:"
grep "Context: HIGH RISK:" shai-hulud-detect/bash-testcases.log | wc -l

echo ""
echo "## List all HIGH RISK Context lines:"
grep "Context: HIGH RISK:" shai-hulud-detect/bash-testcases.log | sed 's/.*Context: HIGH RISK: //' | sort | uniq -c
