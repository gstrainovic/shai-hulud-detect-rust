#!/bin/bash
# Run ALL documented test cases and capture EXACT counts

cd /c/Users/gstra/Code/rust-scanner/shai-hulud-detect

echo "# EXACT TEST CASE EXPECTED VALUES"
echo ""
echo "Generated: $(date)"
echo "Version: $(grep -m1 "##" CHANGELOG.md | head -1)"
echo ""
echo "---"
echo ""

test_cases=(
    "clean-project"
    "infected-project"
    "mixed-project"
    "namespace-warning"
    "semver-matching"
    "legitimate-crypto"
    "chalk-debug-attack"
    "common-crypto-libs"
    "xmlhttp-legitimate"
    "xmlhttp-malicious"
    "lockfile-false-positive"
    "lockfile-compromised"
)

for test in "${test_cases[@]}"; do
    echo "## Test: $test"
    echo ""
    
    # Run test
    ./shai-hulud-detector.sh "test-cases/$test" > "/tmp/test_${test}.log" 2>&1
    
    # Extract counts
    high=$(grep "High Risk Issues:" "/tmp/test_${test}.log" | grep -oE '[0-9]+' || echo "0")
    medium=$(grep "Medium Risk Issues:" "/tmp/test_${test}.log" | grep -oE '[0-9]+' || echo "0")
    low=$(grep "Low Risk.*informational" "/tmp/test_${test}.log" | grep -oE '[0-9]+' || echo "0")
    
    echo "**Expected**: HIGH: $high, MEDIUM: $medium, LOW: $low"
    echo ""
    
    # Show key findings
    echo "**Key Findings**:"
    if [ "$high" = "0" ] && [ "$medium" = "0" ] && [ "$low" = "0" ]; then
        echo "- ✅ Clean (no issues)"
    else
        grep -E "🚨 HIGH RISK|⚠️  MEDIUM RISK|ℹ️  LOW RISK" "/tmp/test_${test}.log" | head -5 | sed 's/^/- /'
    fi
    echo ""
    echo "---"
    echo ""
done
