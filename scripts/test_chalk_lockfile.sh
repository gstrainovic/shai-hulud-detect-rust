#!/bin/bash
# Test if get_lockfile_version finds chalk

cd /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/test-cases/lockfile-comprehensive-test

echo "Testing lockfile-comprehensive-test:"
echo ""
echo "package.json has:"
grep "chalk" package.json

echo ""
echo "package-lock.json has:"
grep -A 2 '"chalk"' package-lock.json | head -10

echo ""
echo "The exact version in lockfile is:"
grep '"version": "5.6.1"' package-lock.json
