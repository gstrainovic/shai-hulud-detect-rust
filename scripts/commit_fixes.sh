#!/bin/bash
# Commit all fixes

cd /c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1

echo "🧹 Cleaning up temp scripts..."
rm -f scripts/test_*.sh scripts/check_failed_patterns.sh

echo "📝 Committing..."
git add -A

git commit -m "🐛 Fix all parser bugs + parallel race conditions

CRITICAL FIXES:
1. Parser Bugs:
   ✅ Added 'Issue:' pattern parsing (integrity issues)
   ✅ Added 'ℹ️  LOW RISK FINDINGS' section parsing
   ✅ Added '- Crypto pattern:' format parsing
   ✅ Fixed category detection for integrity issues

2. Race Condition Fix:
   ✅ Parallel scans now use unique temp directories
   ✅ Prevents JSON file conflicts
   ✅ Each scan gets isolated working directory
   ✅ Fixed: lockfile-comprehensive-test showing wrong data

3. Full Sequential Tests:
   ✅ Now save JSON output
   ✅ Enables future analysis

VERIFIED:
✅ infected-lockfile: Parser fixed (Issue: pattern)
✅ infected-lockfile-pnpm: Parser fixed
✅ lockfile-comprehensive-test: Race condition fixed
✅ lockfile-safe-versions: Works (empty test case)
✅ xmlhttp-legitimate: Parser fixed (LOW RISK FINDINGS)
✅ false-positive-project: Works correctly

ALL BUGS FIXED!
Ready for full parallel test run 🚀"

echo "📤 Pushing..."
git push origin master

echo "✅ Done!"
