#!/bin/bash
# Search for LOW RISK details in Bash log

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 Searching for LOW RISK patterns in Bash log..."
echo ""

# Check for "LOW RISK" section
echo "=== Checking for 'LOW RISK' section ==="
grep -n "LOW RISK" scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log

echo ""
echo "=== Checking lines around 'informational' ==="
grep -B 5 -A 5 "informational" scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log | head -30

echo ""
echo "=== Checking for namespace warnings ==="
grep -i "namespace\|@ctrl\|@nativescript" scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log
