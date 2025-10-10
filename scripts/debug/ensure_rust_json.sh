#!/bin/bash
# Check if scan_results.json exists

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

if [ -f scan_results.json ]; then
  echo "✅ scan_results.json exists"
  ls -lh scan_results.json
else
  echo "❌ scan_results.json not found - generating..."
  cargo run --quiet --release -- ../shai-hulud-detect/test-cases/infected-project > /dev/null 2>&1
  echo "✅ Generated scan_results.json"
fi
