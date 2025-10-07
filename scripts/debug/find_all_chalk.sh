#!/bin/bash
# Find ALL files with chalk

cd /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/test-cases

echo "🔍 Finding all chalk occurrences"
echo ""

for file in */package.json */package-lock.json; do
    if [ -f "$file" ]; then
        if grep -q "chalk" "$file"; then
            echo "File: $file"
            grep "chalk" "$file" | head -3
            echo ""
        fi
    fi
done
