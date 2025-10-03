# EXACT TEST CASE EXPECTED VALUES

**Generated from 100% verified Rust scanner**
**Date**: 2025-10-04

These are the EXACT counts each test case should produce.
Derived from the Rust scanner which has mathematical 100% match with Bash.

---

## `clean-project`

**Description**: Clean system (no threats)

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 0
Low Risk (informational): 0
```

**Status**: ✅ **CLEAN** - No issues detected

---

## `infected-project`

**Description**: Multiple Shai-Hulud indicators

**Expected Output**:
```
High Risk Issues: 8
Medium Risk Issues: 16
Low Risk (informational): 2
```

**Status**: 🚨 **COMPROMISED** - Immediate action required

---

## `mixed-project`

**Description**: Suspicious patterns (not definitive)

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 1
Low Risk (informational): 1
```

**Status**: ⚠️ **SUSPICIOUS** - Manual review needed

---

## `namespace-warning`

**Description**: Safe packages from affected namespaces

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 0
Low Risk (informational): 0
```

**Status**: ✅ **CLEAN** - No issues detected

---

## `semver-matching`

**Description**: Packages that could match on update

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 19
Low Risk (informational): 2
```

**Status**: ⚠️ **SUSPICIOUS** - Manual review needed

---

## `legitimate-crypto`

**Description**: Legitimate crypto library usage

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 1
Low Risk (informational): 0
```

**Status**: ⚠️ **SUSPICIOUS** - Manual review needed

---

## `chalk-debug-attack`

**Description**: Chalk/Debug crypto theft attack

**Expected Output**:
```
High Risk Issues: 6
Medium Risk Issues: 7
Low Risk (informational): 0
```

**Status**: 🚨 **COMPROMISED** - Immediate action required

---

## `common-crypto-libs`

**Description**: Common crypto libraries (safe)

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 1
Low Risk (informational): 0
```

**Status**: ⚠️ **SUSPICIOUS** - Manual review needed

---

## `xmlhttp-legitimate`

**Description**: Legitimate XMLHttpRequest (React Native/Next.js)

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 0
Low Risk (informational): 0
```

**Status**: ✅ **CLEAN** - No issues detected

---

## `xmlhttp-malicious`

**Description**: Malicious XMLHttpRequest with crypto patterns

**Expected Output**:
```
High Risk Issues: 2
Medium Risk Issues: 3
Low Risk (informational): 0
```

**Status**: 🚨 **COMPROMISED** - Immediate action required

---

## `lockfile-false-positive`

**Description**: Safe despite name similarity

**Expected Output**:
```
High Risk Issues: 0
Medium Risk Issues: 0
Low Risk (informational): 0
```

**Status**: ✅ **CLEAN** - No issues detected

---

## `lockfile-compromised`

**Description**: Actual compromised package in lockfile

**Expected Output**:
```
High Risk Issues: 1
Medium Risk Issues: 1
Low Risk (informational): 0
```

**Status**: 🚨 **COMPROMISED** - Immediate action required

---


## Usage in Testing

When testing the scanner, verify that each test case produces these EXACT numbers.
Any deviation indicates a regression or change in detection logic.

Example test script:
```bash
# Test clean-project
./shai-hulud-detector.sh test-cases/clean-project
# Should show: HIGH: 0, MEDIUM: 0, LOW: 0

# Test infected-project
./shai-hulud-detector.sh test-cases/infected-project
# Should show: HIGH: 8, MEDIUM: 8, LOW: 2
```
