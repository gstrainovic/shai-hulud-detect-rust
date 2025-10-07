#!/bin/bash
# Test normal mode to ensure it's still 100% match

cd /c/Users/gstra/Code/rust-scanner

echo "🧪 Testing NORMAL mode (no --paranoid flag)..."
echo ""

# Run Rust normal mode
echo "⚡ Running Rust scanner (normal mode)..."
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases > ../analyze/normal_mode_rust.txt 2>&1
cd ..

# Extract numbers
rust_h=$(grep "High Risk Issues:" analyze/normal_mode_rust.txt | awk '{print $NF}')
rust_m=$(grep "Medium Risk Issues:" analyze/normal_mode_rust.txt | awk '{print $NF}')
rust_l=$(grep "Low Risk" analyze/normal_mode_rust.txt | grep "informational" | awk '{print $NF}')

echo ""
echo "📊 NORMAL MODE RESULTS:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Expected (from v1.0.0-perfect-match tag):"
echo "  HIGH:   18"
echo "  MEDIUM: 58"
echo "  LOW:    9"
echo ""
echo "Current Rust Normal Mode:"
echo "  HIGH:   $rust_h"
echo "  MEDIUM: $rust_m"
echo "  LOW:    $rust_l"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ "$rust_h" = "18" ] && [ "$rust_m" = "58" ] && [ "$rust_l" = "9" ]; then
    echo "✅ NORMAL MODE: STILL PERFECT 100% MATCH!"
    echo "   Paranoid changes did NOT break normal mode ✅"
else
    echo "❌ NORMAL MODE: BROKEN!"
    echo "   Paranoid changes broke normal mode!"
    echo "   Differences:"
    echo "     HIGH:   $((rust_h - 18))"
    echo "     MEDIUM: $((rust_m - 58))"  
    echo "     LOW:    $((rust_l - 9))"
fi
