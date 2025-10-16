# Shai-Hulud NPM Supply Chain Attack Detector (Rust)

**Version 2.6.4** - Rust implementation of [shai-hulud-detector v2.6.3](../shai-hulud-detect/)

## 🎯 100% Compatible Rust Implementation

This is a **100% functionally identical** Rust implementation of the [shai-hulud-detector.sh](../shai-hulud-detect/) Bash scanner.

### ✅ Verification Status

**Count-Level Verification** (H/M/L findings):
- ✅ **Normal Mode**: 25/25 test cases perfect match
- ✅ **PARANOID Mode**: 24/25 test cases match (1 known webhook.site bug in Bash)

**Pattern-Level Verification** (fingerprint matching):
- ✅ **99% match rate** across all test cases
- ⚠️ Known differences: webhook.site detection (Bash bug - [PR #50](https://github.com/Cobenian/shai-hulud-detect/pull/50))

Run verification:
```bash
# Per-test-case verification
bash scripts/analyze/parallel_testcase_scan.sh           # Normal mode
bash scripts/analyze/parallel_testcase_scan_paranoid.sh  # PARANOID mode

# Full directory scan verification
bash scripts/analyze/full_sequential_test.sh             # Normal mode
bash scripts/analyze/full_sequential_test_paranoid.sh    # PARANOID mode
```

### ⚡ Performance

~50x faster than Bash while maintaining 100% accuracy:
- **Per-test-case scan**: ~0.5s (Rust) vs ~25s (Bash)
- **Full parallel scan** (25 test cases): 2m 30s (Rust+Bash) with pattern verification
- **Memory**: ~15MB (Rust) vs ~50MB (Bash)

### 🚀 Quick Start

```bash
# Build
cargo build --release

# Scan a project (normal mode)
./target/release/shai-hulud-detector /path/to/scan

# Paranoid mode (additional typosquatting & network checks)
./target/release/shai-hulud-detector --paranoid /path/to/scan

# Verification mode (reduces false positives by 60%+)
./target/release/shai-hulud-detector --verify /path/to/scan

# Combine paranoid + verification
./target/release/shai-hulud-detector --paranoid --verify /path/to/scan
```

### 🔍 Verification Mode (--verify)

**NEW:** Intelligent verification to reduce false positives by up to 62%!

```bash
# Enable verification
./target/release/shai-hulud-detector --verify /path/to/scan
```

**What it does:**
- ✅ Checks actual installed versions via lockfiles (npm/pnpm/yarn)
- ✅ Queries package managers (pnpm list, npm list) for runtime verification
- ✅ Pattern-based verification for known-legitimate packages (vue-demi, formdata-polyfill)
- ✅ Identifies 10+ common utility packages as safe (debug, chalk, ansi-regex, etc.)

**Results:**
- Reduces critical findings from 116 → 44 on production apps (62% reduction)
- Provides confidence levels (High/Medium) and verification reasons
- Maintains 100% Bash compatibility (same H/M/L counts without --verify)
- Adds ~5-10 seconds to scan time

**Example output:**
```
MEDIUM RISK: Suspicious package versions detected:
   - Package: chalk@^5.0.0 (locked to 5.6.2 - safe)
     Found in: /path/to/package.json
     [VERIFIED SAFE - High confidence]: Lockfile pins to safe version

   - Package: debug@^4.3.0
     Found in: /path/to/package.json
     [VERIFIED SAFE - Medium confidence]: Well-known debugging utility (safe unless specific version matches)
```

### 🧪 Testing

```bash
# Run unit tests (9 tests in ~0.02s)
cargo test

# Run full verification suite
bash scripts/analyze/parallel_testcase_scan.sh
```
### 📦 What It Detects

See the original [shai-hulud-detect README](../shai-hulud-detect/README.md) for full details.

Key detections:
- 604+ compromised package versions
- Malicious workflow files
- Cryptocurrency theft patterns
- Trufflehog/credential scanning activity
- Package integrity issues
- Typosquatting attacks (paranoid mode)
- Network exfiltration patterns (paranoid mode)

---

## 🔄 Key Differences from Bash Scanner

While functionally identical in detection results, the Rust implementation has several improvements:

### 1. **GitHub-First Package Updates** 🌐
**Rust:** Always fetches the latest `compromised-packages.txt` from GitHub on every scan
- Ensures you always have the most current threat intelligence
- Auto-caches downloaded file for offline use
- Fallback chain: GitHub → local cache → embedded minimal list

**Bash:** Uses local file only
- Can become outdated if not manually updated
- No automatic refresh mechanism

```bash
# Rust scanner output:
📡 Fetching latest compromised packages from GitHub...
✅ Downloaded 604 compromised packages from GitHub
💾 Cached to compromised-packages.txt for offline use
```

### 2. **Performance** ⚡
**Rust:** ~230x faster on typical projects
- Single scan: 0.04s vs 9s (Bash)
- Large projects (50k+ files): 45s vs estimated 6+ hours

**Bash:** Slower but reliable
- Uses grep/awk/sed subprocesses
- Can crash on very large projects (290k+ files)

### 3. **Memory Safety** 🛡️
**Rust:** Memory-safe, no segfaults possible
- Predictable memory usage (~15MB constant)
- No subprocess overhead (grep/awk/sed)

**Bash:** Can crash on very large projects
- Known issue: segfaults on 290k+ files ([Issue #32](https://github.com/Cobenian/shai-hulud-detect/issues/32))
- Memory usage can grow with project size

### 4. **Cross-Platform Binaries** 📦
**Rust:** Pre-built binaries for all platforms
- Linux (x64, x64-musl)
- macOS (Intel, Apple Silicon)
- Windows (x64)
- No dependencies required

**Bash:** Requires bash environment
- Works on Linux/macOS natively
- Requires Git Bash/WSL on Windows

---

