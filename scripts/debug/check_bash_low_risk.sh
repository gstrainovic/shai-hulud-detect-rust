#!/bin/bash
# Check LOW RISK patterns in bash log

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 Checking LOW RISK / informational patterns in Bash log..."
echo ""

grep -A 20 "LOW RISK\|informational" scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log 2>&1 | head -25
