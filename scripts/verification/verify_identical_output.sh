#!/bin/bash
# Simple diff verification - Bash vs Rust output
# Ignores header differences (paths, timestamps), only compares FINDINGS

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 BASH vs RUST OUTPUT COMPARISON"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run both scanners
echo "⚡ Running Bash scanner..."
cd shai-hulud-detect
./shai-hulud-detector.sh test-cases > /tmp/bash_full.log 2>&1
cd ..

echo "⚡ Running Rust scanner..."
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases > /tmp/rust_full.log 2>&1
cd ..

# Extract just the REPORT sections (skip header)
echo ""
echo "📊 Extracting REPORT sections..."

# Bash: from "SHAI-HULUD DETECTION REPORT" onwards
sed -n '/SHAI-HULUD DETECTION REPORT/,$p' /tmp/bash_full.log > /tmp/bash_report.txt

# Rust: from "SHAI-HULUD DETECTION REPORT" onwards  
sed -n '/SHAI-HULUD DETECTION REPORT/,$p' /tmp/rust_full.log > /tmp/rust_report.txt

# Compare reports
echo ""
echo "🔎 Comparing REPORT sections..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if diff -q /tmp/bash_report.txt /tmp/rust_report.txt > /dev/null; then
    echo "✅ REPORTS ARE IDENTICAL!"
    echo ""
    echo "   Both scanners produce exactly the same output."
    echo "   100% verification successful! 🎉"
    exit 0
else
    echo "⚠️  REPORTS DIFFER"
    echo ""
    echo "Showing first 50 lines of differences:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    diff -u /tmp/bash_report.txt /tmp/rust_report.txt | head -50
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Full diffs saved to:"
    echo "  - /tmp/bash_report.txt"
    echo "  - /tmp/rust_report.txt"
    echo ""
    echo "Run: diff -u /tmp/bash_report.txt /tmp/rust_report.txt | less"
    exit 1
fi
