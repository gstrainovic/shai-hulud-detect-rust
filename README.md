# Shai-Hulud NPM Supply Chain Attack Detector (Rust)

## 🎯 100% Compatible Rust Implementation

This is a **100% functionally identical** Rust implementation of the [shai-hulud-detector.sh](../shai-hulud-detect/) Bash scanner.

### ✅ Verification

- **HIGH RISK**: 18/18 ✅
- **MEDIUM RISK**: 58/58 ✅  
- **LOW RISK**: 9/9 ✅

Run `bash scripts/analyze/verify_100_percent.sh` for proof.

### ⚡ Performance

~50x faster than Bash while maintaining 100% accuracy:
- Full scan: ~0.9s (Rust) vs ~45s (Bash)
- Memory: ~15MB (Rust) vs ~50MB (Bash)

### 🚀 Quick Start

```bash
# Build
cargo build --release

# Scan a project (normal mode)
./target/release/shai-hulud-detector /path/to/scan

# Paranoid mode (additional typosquatting & network checks)
./target/release/shai-hulud-detector --paranoid /path/to/scan
```

### 📚 Documentation

- [VERIFICATION_GUIDE.md](VERIFICATION_GUIDE.md) - Comprehensive verification proof

### 🧪 Testing

```bash
## 🧪 Testing & Verification

### Unit Tests (Fast)
```bash
cargo test
# 12 unit tests in ~1 second
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

