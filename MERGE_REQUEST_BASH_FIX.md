# Fix: Network Exfiltration Detector - Hostname Pattern Support

## Summary

The network exfiltration detector's regex pattern was too strict and failed to detect suspicious domains when they appeared as property values (e.g., `hostname: 'webhook.site'`). This fix broadens the pattern to catch structured code patterns in addition to simple URL strings.

## Problem

The current regex pattern:
```bash
[[:space:]]$domain[[:space:]/\"\']
```

**Only matches:**
- Domains preceded by whitespace
- Examples: `https://webhook.site/...` ✅, ` webhook.site/` ✅

**Misses:**
- Property-value patterns: `hostname: 'webhook.site'` ❌
- Object properties: `{ host: "webhook.site" }` ❌
- Comma-separated lists: `domains: ["webhook.site", ...]` ❌

**Real-world impact:**
In `actual-credential-harvester.js` (test case), the malicious code uses:
```javascript
const options = {
    hostname: 'webhook.site',
    port: 443,
    path: '/bb8ca5f6-4175-45d2-b042-fc9ebb8170b7',
```

This pattern can now be detected more reliably with the enhanced regex.

## Solution

Updated regex pattern to include common JavaScript/JSON property syntax:
```bash
[[:space:]:,\"\']$domain[[:space:]/\"\',;]
```

**Now matches:**
- ✅ `hostname: 'webhook.site'` (property value)
- ✅ `"host": "webhook.site"` (JSON property)
- ✅ `domains: ["webhook.site"]` (array element)
- ✅ `https://webhook.site/` (URL string) - still works!
- ✅ ` webhook.site/` (whitespace-delimited) - still works!

## Changes

**File:** `shai-hulud-detector.sh`

**Lines changed:** ~1267-1268

### Line 1267-1268 (Pattern check and line info):
```diff
- if grep -qE "https?://[^[:space:]]*$domain|[[:space:]]$domain[[:space:]/\"\']" "$file" 2>/dev/null; then
+ if grep -qE "https?://[^[:space:]]*$domain|[[:space:]:,\"\']$domain[[:space:]/\"\',;]" "$file" 2>/dev/null; then
```

And the corresponding line_info extraction uses the same pattern.

## Testing

### Before Fix (main branch):
```bash
$ shai-hulud-detector.sh --paranoid test-cases/infected-project
Medium Risk Issues: 18
```

### After Fix (fix/network-exfiltration-hostname-pattern branch):
```bash
$ shai-hulud-detector.sh --paranoid test-cases/infected-project
Medium Risk Issues: 19  # +1 additional detection!
```

**Improvement:**
- ✅ +1 MEDIUM risk finding detected
- ✅ Better coverage for structured hostname patterns
- ✅ More reliable detection of property-based domain references

## Impact

- **Security:** Closes detection gap for sophisticated malware using structured code patterns
- **Backward Compatible:** All previous detections still work
- **Test Coverage:** Improved detection in infected-project test case
- **Improvement:** +1 MEDIUM finding detected (18 → 19)

## Verification

To verify this fix:
1. Checkout main branch: `git checkout main`
2. Run paranoid scan: `./shai-hulud-detector.sh --paranoid test-cases/infected-project`
3. Note the count: **18 MEDIUM**
4. Checkout fix branch: `git checkout fix/network-exfiltration-hostname-pattern`
5. Run paranoid scan again
6. Note the improved count: **19 MEDIUM** (+1 detection)

```bash
# Main branch
./shai-hulud-detector.sh --paranoid test-cases/infected-project 2>&1 | grep "Medium Risk Issues:"
# Expected: Medium Risk Issues: 18

# Fix branch
./shai-hulud-detector.sh --paranoid test-cases/infected-project 2>&1 | grep "Medium Risk Issues:"
# Expected: Medium Risk Issues: 19
```

## Related

- Issue: Network exfiltration detector misses hostname property patterns
- Test case: `test-cases/infected-project/actual-credential-harvester.js`
- Comparison tool: Rust scanner correctly detected this pattern

---

**Branch:** `fix/network-exfiltration-hostname-pattern`  
**Fixes:** Detection gap in network exfiltration patterns  
**Type:** Bug fix / Security improvement  
**Risk:** Low (pattern extension, backward compatible)
