#!/bin/bash
# Debug: How many compromised packages does Rust actually find?

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🔍 DEBUGGING RUST COMPROMISED PACKAGE COUNTING"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Running Rust scanner..."
cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 > /tmp/rust_full.log

echo "## Full compromised packages section:"
grep -A 50 "Compromised package versions detected:" /tmp/rust_full.log | head -60

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "## How Rust counts HIGH:"
echo "compromised_found.len() ="
grep "Contains compromised package version:" /tmp/rust_full.log | wc -l

echo ""
echo "## Individual findings listed:"
grep "Found in:.*package.json" /tmp/rust_full.log | grep -v lockfile | wc -l | xargs echo "package.json files with compromised packages:"

echo ""
echo "## Summary says:"
grep "High Risk Issues:" /tmp/rust_full.log
