#!/bin/bash
# Run Python verification test

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

# Ensure we have Rust JSON
bash scripts/debug/ensure_rust_json.sh > /dev/null 2>&1

echo "🐍 Running Python pattern verification..."
echo ""

python scripts/verify_pattern_match.py \
  scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log \
  scan_results.json
