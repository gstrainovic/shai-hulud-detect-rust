#!/bin/bash
# Find where chalk@5.6.1 appears

cd /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/test-cases

echo "🔍 Finding chalk@5.6.1 locations"
echo ""

for dir in */; do
    if [ -f "$dir/package.json" ]; then
        if grep -q '"chalk".*"5.6.1"' "$dir/package.json"; then
            echo "Found in: $dir"
            grep -A 1 -B 1 "chalk" "$dir/package.json" | head -5
            echo ""
        fi
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Also checking lockfiles:"
for dir in */; do
    if [ -f "$dir/package-lock.json" ]; then
        if grep -q '"chalk".*"5.6.1"' "$dir/package-lock.json"; then
            echo "Lock found in: $dir"
        fi
    fi
done
