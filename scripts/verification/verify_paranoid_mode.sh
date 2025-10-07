#!/bin/bash
# Verify PARANOID mode - test-case by test-case comparison

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 PARANOID MODE - PER TEST-CASE VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Pre-build Rust once (faster than cargo run each time)
echo "Building Rust scanner..."
cd dev-rust-scanner-1
cargo build --release --quiet 2>/dev/null
RUST_BIN="target/release/shai-hulud-detector.exe"
cd ..
echo ""

MISMATCHES=0
MATCHES=0

for testcase in shai-hulud-detect/test-cases/*/; do
    name=$(basename "$testcase")
    
    # Skip if not a directory
    [ ! -d "$testcase" ] && continue
    
    echo "Testing: $name"
    
    # Bash paranoid
    bash_output=$(cd shai-hulud-detect && timeout 30 ./shai-hulud-detector.sh --paranoid "test-cases/$name" 2>&1)
    bash_high=$(echo "$bash_output" | grep "High Risk Issues:" | awk '{print $NF}' | tr -d '\r' || echo "0")
    bash_medium=$(echo "$bash_output" | grep "Medium Risk Issues:" | awk '{print $NF}' | tr -d '\r' || echo "0")
    bash_low=$(echo "$bash_output" | grep "Low Risk" | grep "informational" | awk '{print $NF}' | tr -d '\r' || echo "0")
    
    # Rust paranoid (using pre-built binary)
    rust_output=$(dev-rust-scanner-1/$RUST_BIN "$testcase" --paranoid 2>&1)
    rust_high=$(echo "$rust_output" | grep "High Risk Issues:" | awk '{print $NF}' | tr -d '\r' || echo "0")
    rust_medium=$(echo "$rust_output" | grep "Medium Risk Issues:" | awk '{print $NF}' | tr -d '\r' || echo "0")
    rust_low=$(echo "$rust_output" | grep "Low Risk" | grep "informational" | awk '{print $NF}' | tr -d '\r' || echo "0")
    
    # Default to 0 if empty
    bash_high=${bash_high:-0}
    bash_medium=${bash_medium:-0}
    bash_low=${bash_low:-0}
    rust_high=${rust_high:-0}
    rust_medium=${rust_medium:-0}
    rust_low=${rust_low:-0}
    
    # Compare
    if [ "$bash_high" != "$rust_high" ] || [ "$bash_medium" != "$rust_medium" ] || [ "$bash_low" != "$rust_low" ]; then
        echo "  ❌ MISMATCH!"
        echo "     Bash:  $bash_high/$bash_medium/$bash_low"
        echo "     Rust:  $rust_high/$rust_medium/$rust_low"
        MISMATCHES=$((MISMATCHES + 1))
    else
        echo "  ✅ Match: $bash_high/$bash_medium/$bash_low"
        MATCHES=$((MATCHES + 1))
    fi
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "RESULTS:"
echo "  ✅ Matches: $MATCHES"
echo "  ❌ Mismatches: $MISMATCHES"
echo ""

if [ $MISMATCHES -eq 0 ]; then
    echo "🎉 PARANOID MODE: 100% MATCH!"
else
    echo "⚠️  PARANOID MODE: NOT 100% - $MISMATCHES test-cases differ!"
fi
