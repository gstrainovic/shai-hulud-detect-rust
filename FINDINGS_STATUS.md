# Findings Comparison Status

## Current Status (2025-10-14)

### Test Setup
- **Bash Scanner:** `shai-hulud-detect/` (original Cobenian repo, main branch)
- **Rust Scanner:** `dev-rust-scanner-1/` (our implementation)
- **Test Case:** `infected-project` (PARANOID mode)

### Count Comparison

| Risk Level | Bash (Original) | Rust Scanner | Status |
|------------|----------------|--------------|--------|
| HIGH       | 8              | 8            | ✅ Match |
| MEDIUM     | 18             | 19           | ⚠️ +1 in Rust |
| LOW        | 2              | 2            | ✅ Match |
| **TOTAL**  | **28**         | **29**       | **+1** |

### Difference Explanation

**+1 MEDIUM in Rust:**
- **Finding:** `actual-credential-harvester.js` - webhook.site at line 48 (hostname pattern)
- **Reason:** Rust correctly detects `hostname: 'webhook.site'` pattern
- **Bash Status:** Original bash (main) has a bug - misses this pattern
- **Fix Status:** Fixed in `shai-hulud-detect-gs/fix/network-exfiltration-hostname-pattern`
- **Resolution:** After PR merge to Cobenian/shai-hulud-detect, counts will match

### Why This Is Correct

This difference is **EXPECTED and DESIRED**:
1. ✅ Rust scanner implements the corrected detection logic
2. ✅ Original bash has known bug (already fixed in PR branch)
3. ✅ Rust finding is a TRUE POSITIVE (malicious hostname pattern)
4. ✅ After PR merge, both will find 19 MEDIUM findings

### Parser Comparison (Pattern-Level)

**Bash Findings:** 26 patterns  
**Rust Findings:** 20 patterns  
**Matches:** 16 (67%)  

**Remaining Differences (10 missing + 4 extra):**

**Missing in Rust (10):**
- Most are message format variations (same finding, different text)
- Examples:
  - "contains trufflehog references" vs "trufflehog binary found"
  - "potentially suspicious environment variable" (capitalization differences)

**Extra in Rust (4):**
- 2x LOW risk namespace warnings (Rust shows these, Bash doesn't in paranoid)
- 1x webhook.site hostname pattern (the expected +1)
- 1x webhook.site in different file

### Next Steps

1. ✅ Network message format fixed (added "at line N:")
2. ✅ Compromised packages loading fixed (nativescript now detected)
3. ⏳ Message format harmonization (make Rust messages match Bash exactly)
4. ⏳ Fix remaining 10 message variations

### Technical Notes

**Repository Setup:**
- `shai-hulud-detect/` - Original repo (Cobenian/shai-hulud-detect, main)
  - Used for: compromised-packages.txt loading
  - Status: Unmodified, contains the hostname detection bug
  
- `shai-hulud-detect-gs/` - Our fork
  - Used for: Test execution, PR preparation
  - Branch: `fix/network-exfiltration-hostname-pattern`
  - Status: Contains hostname detection fix

**Why Test Against Original:**
- Provides fair baseline comparison
- Shows Rust scanner is more accurate (finds true positives original misses)
- After PR merge, will automatically align

## Improvements Made

### 2025-10-14
- ✅ Added line numbers to network warnings ("at line N:")
- ✅ Fixed compromised packages loading (nativescript detection)
- ✅ Cloned original repo for proper testing
- ✅ Documented expected difference (+1 MEDIUM)

### Remaining Work
- Match message formats exactly (10 variations)
- Verify all 29 findings are correct
- Ensure 100% pattern-level matching
