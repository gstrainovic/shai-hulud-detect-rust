#!/bin/bash
# FINAL CLEANUP - Remove obsolete files

set -e

cd /c/Users/gstra/Code/rust-scanner

echo "🧹 FINAL CLEANUP STARTING"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Phase 1: Cleanup analyze/ folder
echo "## Phase 1: Cleaning analyze/ folder"
echo ""

cd analyze

echo "Creating archive for obsolete docs..."
mkdir -p archive/obsolete-docs-20251007

echo "Moving old docs to archive..."
mv CLEANUP_SUMMARY.md archive/obsolete-docs-20251007/ 2>/dev/null || true
mv EXACT_TEST_EXPECTATIONS.md archive/obsolete-docs-20251007/ 2>/dev/null || true
mv FINAL_CLEANUP_REPORT.md archive/obsolete-docs-20251007/ 2>/dev/null || true
mv FINAL_SUMMARY.md archive/obsolete-docs-20251007/ 2>/dev/null || true
mv MIGRATION_PLAN.md archive/obsolete-docs-20251007/ 2>/dev/null || true

echo "Deleting temp files (31 files)..."
rm -f temp_*.sh

echo "Deleting obsolete test scripts..."
rm -f accurate_network_test.sh
rm -f test_bash_fix.sh
rm -f test_bash_unicode.sh
rm -f test_homoglyph_fix.sh
rm -f test_all_fixes.sh
rm -f compare_fixed_paranoid.sh
rm -f rescan_paranoid_fixed.sh
rm -f manual_comparison.sh
rm -f find_extra_medium.sh
rm -f analyze_custom_tests.sh
rm -f check_gold_tests.sh
rm -f examine_archive_tests.sh
rm -f search_archive_tests.sh
rm -f cleanup_test_files.sh
rm -f verify_issues_fixed.sh
rm -f check_namespace_test.sh
rm -f check_paranoid_status.sh
rm -f generate_test_expectations.sh
rm -f quick_exact_tests.sh
rm -f verify_individual_vs_all.sh
rm -f verify_network_test_count.sh

echo "Deleting old log files..."
rm -f bash_fresh.txt
rm -f normal_mode_rust.txt
rm -f rust_paranoid_NEW.txt
rm -f paranoid_comparison.txt

echo "✅ analyze/ cleaned!"
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Phase 2: Cleaning dev-rust-scanner-1/ folder"
echo ""

cd dev-rust-scanner-1

echo "Deleting obsolete docs..."
rm -f PARANOID_ROOT_CAUSE.md
rm -f BASH_FIX_RESULTS.md
rm -f ARCHIVE_TEST_ANALYSIS.md
rm -f FINAL_CLEANUP_REPORT.md
rm -f PERFECT_MATCH_ACHIEVEMENT.md
rm -f GITHUB_ISSUE_43_CORRECTED.md
rm -f HOMOGLYPH_BUG_ISSUE.md

echo "✅ dev-rust-scanner-1/ cleaned!"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## Phase 3: Moving essential scripts"
echo ""

echo "Moving verify_normal_mode.sh to scripts/..."
cp ../analyze/verify_normal_mode.sh scripts/ 2>/dev/null || echo "Already exists or not found"

echo "✅ Scripts organized!"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "## CLEANUP SUMMARY"
echo ""

echo "Files deleted from analyze/:"
echo "  - 31 temp_*.sh files"
echo "  - 21 obsolete test scripts"
echo "  - 4 old log files"
echo "  - 5 docs moved to archive"
echo "  Total: ~61 files cleaned"

echo ""
echo "Files deleted from dev-rust-scanner-1/:"
echo "  - 7 obsolete documentation files"

echo ""
echo "Remaining in analyze/:"
ls -1 ../analyze/*.sh 2>/dev/null | wc -l | xargs echo "  - Shell scripts:"
ls -1 ../analyze/*.md 2>/dev/null | wc -l | xargs echo "  - Markdown docs:"

echo ""
echo "Remaining in dev-rust-scanner-1/:"
ls -1 *.md 2>/dev/null | wc -l | xargs echo "  - Markdown docs:"

echo ""
echo "🎉 CLEANUP COMPLETE!"
echo ""
echo "Next steps:"
echo "1. Review remaining files"
echo "2. Update documentation with new counts (18/66/9)"
echo "3. Commit cleanup changes"
