# Issue: Homoglyph Detection Fails Due to AWK Filter

## 🐛 Bug Description

The typosquatting homoglyph detection in paranoid mode **fails to detect Unicode homoglyphs** because the AWK package name extraction filter **removes non-ASCII packages before detection runs**.

## 📍 Location

**File**: `shai-hulud-detector.sh`  
**Function**: `check_typosquatting()`  
**Line**: ~943 (inside AWK script)

## 🔍 The Problem

The AWK script filters package names to ASCII-only **before** the Unicode homoglyph detection logic runs:

```bash
# Line 943 - BROKEN:
if ($0 ~ /^[a-zA-Z@][a-zA-Z0-9@\/\._-]*$/) print $0
#          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#          This regex ONLY matches ASCII characters!
```

**Result**: Packages like `reаct` (with Cyrillic 'а') are **filtered out** and never reach the Unicode detection code at line 961.

## ✅ Reproduction

**Test file** (`test-cases/comprehensive-test/package.json`):
```json
{
  "dependencies": {
    "react": "^18.0.0",
    "reаct": "^1.0.0"
  }
}
```
Note: `reаct` uses Cyrillic 'а' (U+0430) instead of Latin 'a' (U+0061)

**Current behavior**:
```bash
$ ./shai-hulud-detector.sh --paranoid test-cases/comprehensive-test
# Output: Only finds "lodsh" typo, MISSES "reаct" homoglyph
Medium Risk Issues: 6
```

**Expected behavior**:
```bash
# Should find "reаct" Unicode homoglyph
Medium Risk Issues: 7
```

## 💥 Impact

**Severity**: HIGH - Security detection bypass

- Homoglyph attacks using Unicode characters are **NOT detected**
- Attackers can use Cyrillic/Greek characters (`reаct`, `lоdash`) to bypass detection
- False sense of security - users think packages are safe when they're not
- The Unicode detection code exists (line 955-964) but **never executes**

## 🔧 Suggested Fix

Remove the ASCII-only filter from AWK and let the Unicode detection run:

```bash
# Current (line 943):
if ($0 ~ /^[a-zA-Z@][a-zA-Z0-9@\/\._-]*$/) print $0

# Fixed:
# Allow all package names (including Unicode) for homoglyph detection
print $0
```

**Alternative fix** (if you want minimal output filtering):
```bash
# Only filter obviously invalid names, but allow Unicode:
if (length($0) > 1) print $0
```

## 📊 Test Results

**Before fix**:
- `reаct` (Cyrillic): ❌ NOT detected
- `lodsh` (ASCII typo): ✅ Detected

**After fix**:
- `reаct` (Cyrillic): ✅ Detected as Unicode homoglyph  
- `lodsh` (ASCII typo): ✅ Still detected

## 🔗 Related

The Unicode detection logic at lines 955-964 is **correct** and works when tested in isolation. The issue is purely the AWK pre-filter preventing it from running.

## 📝 Additional Context

Tested on:
- ✅ Git Bash 2.x (Windows/MINGW64)
- ✅ Bash 5.x (Linux)

This was discovered while creating a Rust port that correctly detects Unicode homoglyphs. The Rust version revealed that Bash was missing detections.

---

**Note**: After fixing, Bash may count some homoglyphs twice (once as Unicode, once as character difference). Consider deduplicating warnings for the same package.
