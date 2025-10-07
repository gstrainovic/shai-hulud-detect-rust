#!/bin/bash
# Find ALL compromised packages in ALL lockfiles

cd /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/test-cases

echo "🔍 COMPROMISED PACKAGES IN LOCKFILES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Known compromised versions
COMPROMISED=(
    "chalk:5.6.1"
    "debug:4.4.2"
    "ansi-styles:6.2.2"
    "color-convert:3.1.1"
)

for lockfile in */package-lock.json */pnpm-lock.yaml */yarn.lock; do
    if [ -f "$lockfile" ]; then
        echo "Checking: $lockfile"
        for pkg in "${COMPROMISED[@]}"; do
            name="${pkg%:*}"
            version="${pkg#*:}"
            
            if grep -q "\"$name\".*\"$version\"" "$lockfile" 2>/dev/null || \
               grep -q "$name.*$version" "$lockfile" 2>/dev/null; then
                echo "  ✅ Found: $name@$version"
            fi
        done
        echo ""
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Now let's see what Bash actually finds from lockfiles..."
grep "Compromised package in lockfile:" /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/bash-testcases.log
