#!/bin/bash
# Dynamic verification - Normal Mode (Bash vs Rust)
# No hardcoded numbers - uses live Bash scanner as ground truth

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 NORMAL MODE - DYNAMIC VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run Bash (ground truth)
echo "⚡ Running Bash scanner (normal mode)..."
cd shai-hulud-detect
./shai-hulud-detector.sh test-cases/ > /tmp/bash_normal_verify.log 2>&1
cd ..

bash_h=$(grep "High Risk Issues:" /tmp/bash_normal_verify.log | tail -1 | awk '{print $NF}')
bash_m=$(grep "Medium Risk Issues:" /tmp/bash_normal_verify.log | tail -1 | awk '{print $NF}')
bash_l=$(grep "Low Risk" /tmp/bash_normal_verify.log | grep "informational" | tail -1 | awk '{print $NF}')

# Run Rust
echo "⚡ Running Rust scanner (normal mode)..."
cd dev-rust-scanner-1
cargo run --quiet --release -- ../shai-hulud-detect/test-cases > /tmp/rust_normal_verify.log 2>&1
cd ..

rust_h=$(grep "High Risk Issues:" /tmp/rust_normal_verify.log | tail -1 | awk '{print $NF}')
rust_m=$(grep "Medium Risk Issues:" /tmp/rust_normal_verify.log | tail -1 | awk '{print $NF}')
rust_l=$(grep "Low Risk" /tmp/rust_normal_verify.log | grep "informational" | tail -1 | awk '{print $NF}')

# Default to 0
bash_h=${bash_h:-0}
bash_m=${bash_m:-0}
bash_l=${bash_l:-0}
rust_h=${rust_h:-0}
rust_m=${rust_m:-0}
rust_l=${rust_l:-0}

echo ""
echo "📊 NORMAL MODE RESULTS:"
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

# Compare (strip all whitespace and control chars)
bash_h_clean=$(echo "$bash_h" | tr -d '[:space:][:cntrl:]')
bash_m_clean=$(echo "$bash_m" | tr -d '[:space:][:cntrl:]')
bash_l_clean=$(echo "$bash_l" | tr -d '[:space:][:cntrl:]')
rust_h_clean=$(echo "$rust_h" | tr -d '[:space:][:cntrl:]')
rust_m_clean=$(echo "$rust_m" | tr -d '[:space:][:cntrl:]')
rust_l_clean=$(echo "$rust_l" | tr -d '[:space:][:cntrl:]')

if [ "$rust_h_clean" = "$bash_h_clean" ] && [ "$rust_m_clean" = "$bash_m_clean" ] && [ "$rust_l_clean" = "$bash_l_clean" ]; then
    echo "✅ NORMAL MODE: 100% MATCH!"
    echo "   Rust matches Bash exactly: $rust_h/$rust_m/$rust_l"
    exit 0
else
    echo "❌ NORMAL MODE: MISMATCH!"
    echo ""
    echo "   Differences:"
    if [ "$rust_h_clean" != "$bash_h_clean" ]; then
        echo "     HIGH:   Bash=$bash_h, Rust=$rust_h"
    fi
    if [ "$rust_m_clean" != "$bash_m_clean" ]; then
        echo "     MEDIUM: Bash=$bash_m, Rust=$rust_m"
    fi
    if [ "$rust_l_clean" != "$bash_l_clean" ]; then
        echo "     LOW:    Bash=$bash_l, Rust=$rust_l"
    fi
    exit 1
fi
