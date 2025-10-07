#!/bin/bash
# Find EXACT difference between Bash and Rust HIGH counts

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 FINDING EXACT HIGH COUNT DIFFERENCE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Test each individual test case
for testcase in shai-hulud-detect/test-cases/*/; do
    name=$(basename "$testcase")
    
    # Skip if not a directory
    [ ! -d "$testcase" ] && continue
    
    # Bash
    bash_high=$(cd shai-hulud-detect && ./shai-hulud-detector.sh "test-cases/$name" 2>&1 | grep "High Risk Issues:" | awk '{print $NF}' | tr -d '\r')
    
    # Rust
    rust_high=$(cd dev-rust-scanner-1 && cargo run --quiet --release -- "../shai-hulud-detect/test-cases/$name" 2>&1 | grep "High Risk Issues:" | awk '{print $NF}' | tr -d '\r')
    
    # Default to 0 if empty
    bash_high=${bash_high:-0}
    rust_high=${rust_high:-0}
    
    # Compare
    if [ "$bash_high" != "$rust_high" ]; then
        echo "❌ MISMATCH: $name"
        echo "   Bash: $bash_high HIGH"
        echo "   Rust: $rust_high HIGH"
        echo "   Diff: $((rust_high - bash_high))"
        echo ""
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Calculating totals..."
echo ""

bash_total=$(grep "High Risk Issues:" shai-hulud-detect/bash-testcases.log | tail -1 | awk '{print $NF}')
rust_total=$(cd dev-rust-scanner-1 && cargo run --quiet --release -- ../shai-hulud-detect/test-cases 2>&1 | grep "High Risk Issues:" | awk '{print $NF}')

echo "TOTAL:"
echo "  Bash: $bash_total"
echo "  Rust: $rust_total"
echo "  Diff: $((rust_total - bash_total))"
