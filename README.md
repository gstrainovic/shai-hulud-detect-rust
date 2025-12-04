# Shai-Hulud NPM Supply Chain Attack Detector (Rust)

**Version 3.0.4** (synced with Bash scanner) 

## 🎯 100% Compatible Rust Implementation

This is a **100% functionally identical** Rust implementation of the [https://github.com/Cobenian/shai-hulud-detect](https://github.com/Cobenian/shai-hulud-detect) Bash scanner.

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
# Run unit tests
cargo test

# Run full verification suite
bash tests/parallel_testcase_scan.sh
```
### 📦 What It Detects

See the original [shai-hulud-detect README](../shai-hulud-detect/README.md) for full details.

Key detections:
- Constantly updated compromised package versions (auto-fetched from GitHub)
- **November 2025 "The Second Coming" Attack:**
  - Fake Bun runtime installation (setup_bun.js, bun_environment.js)
  - Malicious GitHub Actions workflows (formatter_*.yml)
  - Discussion-triggered workflows
  - Self-hosted runner backdoors
  - Destructive data deletion patterns
  - SHA1HULUD runner references
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

