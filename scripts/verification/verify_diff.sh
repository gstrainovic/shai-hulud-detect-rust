#!/bin/bash
# Simple: sort both logs and diff them

cd /c/Users/gstra/Code/rust-scanner

BASH_LOG="shai-hulud-detect/bash-testcases.log"
RUST_LOG="dev-rust-scanner-1/logs/rust-testcases-new.log"

echo "🔍 Sorting and comparing logs..."

# Sort both logs
sort "$BASH_LOG" > /tmp/bash_sorted.log
sort "$RUST_LOG" > /tmp/rust_sorted.log

# Diff
diff /tmp/bash_sorted.log /tmp/rust_sorted.log

if [ $? -eq 0 ]; then
    echo "✅ 100% IDENTICAL (after sorting)"
    exit 0
else
    echo "❌ DIFFERENCES FOUND"
    exit 1
fi
