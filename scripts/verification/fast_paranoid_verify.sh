#!/bin/bash
# Fast paranoid verification using pre-built binary

cd /c/Users/gstra/Code/rust-scanner

echo "🔍 PARANOID MODE - FAST VERIFICATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Key test cases
CASES=("comprehensive-test" "infected-project" "typosquatting-project")

for name in "${CASES[@]}"; do
    echo "═══ $name ═══"
    
    # Bash
    echo "Bash:"
    cd shai-hulud-detect
    ./shai-hulud-detector.sh --paranoid "test-cases/$name" 2>&1 | grep -E "High Risk|Medium Risk|Low Risk" | sed 's/^/  /'
    cd ..
    
    # Rust  
    echo "Rust:"
    cd dev-rust-scanner-1
    ./target/release/shai-hulud-detector.exe "../shai-hulud-detect/test-cases/$name" --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk" | sed 's/^/  /'
    cd ..
    
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TOTALS:"
echo ""
echo "Bash All:"
tail -5 shai-hulud-detect/paranoid-bash-testcases.log | grep -E "High Risk|Medium Risk|Low Risk"

echo ""
echo "Rust All:"
cd dev-rust-scanner-1
./target/release/shai-hulud-detector.exe ../shai-hulud-detect/test-cases --paranoid 2>&1 | grep -E "High Risk|Medium Risk|Low Risk"
