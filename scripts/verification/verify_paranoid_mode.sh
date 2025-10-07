#!/bin/bash
# Dynamic verification - Paranoid Mode (Bash vs Rust)
# No hardcoded numbers - uses live Bash scanner as ground truth

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 PARANOID MODE - DYNAMIC VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run Bash (ground truth)
echo "⚡ Running Bash scanner (paranoid mode)..."
cd shai-hulud-detect
./shai-hulud-detector.sh --paranoid test-cases/ > /tmp/bash_paranoid_verify.log 2>&1
cd ..

bash_h=$(grep "High Risk Issues:" /tmp/bash_paranoid_verify.log | tail -1 | awk '{print $NF}')
bash_m=$(grep "Medium Risk Issues:" /tmp/bash_paranoid_verify.log | tail -1 | awk '{print $NF}')
bash_l=$(grep "Low Risk" /tmp/bash_paranoid_verify.log | grep "informational" | tail -1 | awk '{print $NF}')

# Run Rust
echo "⚡ Running Rust scanner (paranoid mode)..."
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases --paranoid > /tmp/rust_paranoid_verify.log 2>&1
cd ..

rust_h=$(grep "High Risk Issues:" /tmp/rust_paranoid_verify.log | tail -1 | awk '{print $NF}')
rust_m=$(grep "Medium Risk Issues:" /tmp/rust_paranoid_verify.log | tail -1 | awk '{print $NF}')
rust_l=$(grep "Low Risk" /tmp/rust_paranoid_verify.log | grep "informational" | tail -1 | awk '{print $NF}')

# Default to 0
bash_h=${bash_h:-0}
bash_m=${bash_m:-0}
bash_l=${bash_l:-0}
rust_h=${rust_h:-0}
rust_m=${rust_m:-0}
rust_l=${rust_l:-0}

echo ""
echo "📊 PARANOID MODE RESULTS:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Bash (Ground Truth):"
echo "  HIGH:   $bash_h"
echo "  MEDIUM: $bash_m"
echo "  LOW:    $bash_l"
echo ""
echo "Rust:"
echo "  HIGH:   $rust_h"
echo "  MEDIUM: $rust_m"
echo "  LOW:    $rust_l"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Compare
if [ "$rust_h" = "$bash_h" ] && [ "$rust_m" = "$bash_m" ] && [ "$rust_l" = "$bash_l" ]; then
    echo "✅ PARANOID MODE: 100% MATCH!"
    echo "   Rust matches Bash exactly: $rust_h/$rust_m/$rust_l"
    exit 0
else
    echo "❌ PARANOID MODE: MISMATCH!"
    echo ""
    echo "   Differences:"
    [ "$rust_h" != "$bash_h" ] && echo "     HIGH:   Bash=$bash_h, Rust=$rust_h (diff: $((rust_h - bash_h)))"
    [ "$rust_m" != "$bash_m" ] && echo "     MEDIUM: Bash=$bash_m, Rust=$rust_m (diff: $((rust_m - bash_m)))"
    [ "$rust_l" != "$bash_l" ] && echo "     LOW:    Bash=$bash_l, Rust=$rust_l (diff: $((rust_l - bash_l)))"
    exit 1
fi
