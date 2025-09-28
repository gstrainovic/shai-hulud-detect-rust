# Shai-Hulud Scanner (Rust Implementation)

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-Active-success)](#overview)

A fast, reliable Rust implementation of the Shai-Hulud NPM supply chain attack detector. This scanner provides comprehensive detection of the September 2025 npm supply chain attacks, including the Shai-Hulud self-replicating worm and the chalk/debug crypto theft attack.

## Quick Links

🔗 **[Original Bash Implementation](https://github.com/Cobenian/shai-hulud-detect)** - Required dependency for test cases and compromised packages database  
📚 **[Attack Documentation](https://github.com/Cobenian/shai-hulud-detect#background)** - Background on the Shai-Hulud attack  
🧪 **[Test Cases](https://github.com/Cobenian/shai-hulud-detect/tree/main/test-cases)** - Comprehensive test scenarios

## Features

- 🚀 **Fast Performance**: Written in Rust for optimal scanning speed
- 🔍 **Comprehensive Detection**: Covers 621+ compromised packages and malicious patterns
- 📊 **JSON Output by Default**: Structured results saved automatically to `scan_results.json`
- � **Clean Console Output**: Progress indicators + summary only, details available in JSON
- ⏱️ **Timing Information**: Tracks scan duration and timestamps for monitoring
- 🛡️ **Paranoid Mode**: Additional security checks for thorough analysis
- 🔧 **CI/CD Ready**: Proper exit codes and clean output for automated workflows
- 📁 **Hash Verification**: Detects known malicious files by SHA-256 hash
- 🎯 **Pattern Matching**: Advanced regex patterns for suspicious content

## Quick Start

### Installation

```bash
# Clone the Rust implementation repository
git clone https://github.com/yourusername/shai-hulud-detect-rust.git
cd shai-hulud-detect-rust

# REQUIRED: Clone the original repository for test cases and compromised packages database
git clone https://github.com/Cobenian/shai-hulud-detect.git ../shai-hulud-detect

# Build the scanner
cargo build --release

# The binary will be available at target/release/shai-hulud-scanner
```

**⚠️ Important**: You must clone the [original shai-hulud-detect repository](https://github.com/Cobenian/shai-hulud-detect) as it contains:
- **Test cases** in `test-cases/` directory
- **Compromised packages database** (`compromised-packages.txt`)
- **Reference bash implementation** for comparison

The Rust scanner expects the original repository to be available at `../shai-hulud-detect/` relative to this project.

### Basic Usage

```bash
# Scan a Node.js project (JSON output is default)
./target/release/shai-hulud-scanner /path/to/your/project

# Scan with paranoid mode (additional security checks)
./target/release/shai-hulud-scanner --paranoid /path/to/your/project

# Custom JSON output file
./target/release/shai-hulud-scanner --output custom-results.json /path/to/your/project

# Disable JSON output (console summary only)
./target/release/shai-hulud-scanner --no-json /path/to/your/project
```

### New Default Behavior

🔄 **JSON Output by Default**: Results are automatically saved to `scan_results.json`  
� **Summary Output**: Shows progress and counts only - detailed findings available in JSON  
⏱️ **Timing Information**: Start time, end time, and scan duration included in output  
� **Better CI/CD Integration**: Clean console output + structured JSON for automation

### Development Usage

```bash
# Scan a Node.js project (JSON output is default)
cargo run /path/to/your/project

# Scan with paranoid mode (additional security checks)
cargo run -- --paranoid /path/to/your/project

# Custom JSON output file
cargo run -- --output my-scan-results.json /path/to/your/project

# Disable JSON output (console summary only)
cargo run -- --no-json /path/to/your/project
```

```

### Example Output

**Standard Output:**
```
📦 Loading compromised packages database...
📦 Loaded 621 compromised packages from database
🔍 Starting Shai-Hulud detection scan...
Scanning directory: /path/to/project
📄 Results will be saved to scan_results.json
🔍 Found 1234 files to analyze
🔍 Checking 45 package.json files for compromised packages...
🔍 Checking 678 JavaScript files for known malicious content...
🔍 Checking 1234 files for suspicious content patterns...
🔍 Checking 2 pnpm-lock.yaml files...
✅ Scan completed
📄 Results saved to: scan_results.json

==============================================
      SHAI-HULUD DETECTION REPORT  
==============================================
� HIGH RISK: 2 issues detected
⚠️  MEDIUM RISK: 5 issues detected
ℹ️  LOW RISK: 3 informational warnings

==============================================
�🔍 SUMMARY:
   Scan Duration: 2.34 seconds
   High Risk Issues: 2
   Medium Risk Issues: 5
   Low Risk (informational): 3
   Total Critical Issues: 7

⚠️  IMPORTANT:
   - High risk issues likely indicate actual compromise
   - Immediate investigation and remediation required
   - Consider running additional security scans
==============================================
```

**Detailed findings are available in the generated JSON file.**

### Exit Codes

The scanner uses standard exit codes for CI/CD integration:
- `0`: Clean - no issues found
- `1`: Medium risk - issues require manual review  
- `2`: High risk - immediate action required

### JSON Output Structure

Results are automatically saved with timing information:
```json
{
  "scan_path": "/path/to/project", 
  "start_time": "2025-09-28T10:00:00.000Z",
  "end_time": "2025-09-28T10:00:02.340Z", 
  "duration_seconds": 2.34,
  "files_scanned": 1234,
  "summary": {
    "high_risk_count": 0,
    "medium_risk_count": 0, 
    "low_risk_count": 0,
    "total_issues": 0
  },
  "results": [...]
}
```

## What It Detects

### High Risk Indicators

✅ **Compromised Package Versions**: 621+ confirmed compromised packages  
✅ **Known Malicious Hashes**: Files matching 9 known malicious SHA-256 hashes  
✅ **Webhook.site Exfiltration**: Data exfiltration via webhook.site endpoints  
✅ **XMLHttpRequest Hijacking**: Crypto wallet theft patterns  
✅ **Attacker Wallet Addresses**: Known cryptocurrency theft addresses  
✅ **Npmjs.help Domain**: Phishing domain used in attacks  

### Medium Risk Indicators (Paranoid Mode)

⚠️ **Base64 Decoding**: Suspicious encoding/decoding patterns  
⚠️ **Private IP Hardcoding**: Hardcoded private IP addresses  
⚠️ **WebSocket Connections**: Connections to suspicious domains  
⚠️ **TruffleHog References**: Potential credential harvesting tools  
⚠️ **Environment Scanning**: Access to sensitive environment variables  

### Low Risk Indicators

ℹ️ **Process.env Access**: Normal environment variable usage  
ℹ️ **Ethereum Addresses**: General cryptocurrency address patterns  

## Testing

The scanner includes comprehensive test coverage using the test cases from the [original bash implementation](https://github.com/Cobenian/shai-hulud-detect):

**Prerequisites**: Make sure you have cloned the original repository:
```bash
# Clone the original repository (if not done during installation)
git clone https://github.com/Cobenian/shai-hulud-detect.git ../shai-hulud-detect
```

**Run E2E Tests:**
```bash
# Run full end-to-end test suite (18 test cases)
cargo run -- --run-e2e-tests

# Test individual scenarios:
# Test on clean project (should exit 0)
cargo run -- ../shai-hulud-detect/test-cases/clean-project

# Test on infected project (should exit 2)  
cargo run -- ../shai-hulud-detect/test-cases/infected-project

# Test comprehensive patterns
cargo run -- ../shai-hulud-detect/test-cases/comprehensive-test

# Test JSON output
cargo run -- --json --output test_results.json ../shai-hulud-detect/test-cases/infected-project
```

### Available Test Cases (from [shai-hulud-detect](https://github.com/Cobenian/shai-hulud-detect))

- `clean-project`: Baseline clean project (✅ 0 exit code)
- `infected-project`: Multiple high-risk indicators (🚨 2 exit code)
- `chalk-debug-attack`: Crypto theft patterns (🚨 2 exit code)
- `comprehensive-test`: Combined malicious patterns (🚨 2 exit code)
- `legitimate-crypto`: False positive testing (✅ 0 exit code)
- `typosquatting-project`: Package name attacks (⚠️ 1 exit code)
- `network-exfiltration-project`: Data exfiltration patterns (🚨 2 exit code)
- `legitimate-security-project`: Security tool false positives (ℹ️ LOW risk)

**Current E2E Test Status**: ✅ **100% (18/18 tests passing)**

## Architecture

The scanner is built with a modular architecture:

- **`scanner.rs`**: Main scanning orchestration and file discovery
- **`patterns.rs`**: Regex pattern matching for suspicious content  
- **`hash_checker.rs`**: SHA-256 hash verification against known malicious files
- **`output.rs`**: Result formatting and JSON serialization
- **`main.rs`**: CLI interface and program entry point

## Performance

The Rust implementation provides significant performance improvements over the bash version:

- 🚀 **Parallel Processing**: Multi-threaded file scanning
- ⚡ **Memory Efficient**: Optimized memory usage for large codebases
- 🔍 **Fast Pattern Matching**: Compiled regex patterns for speed
- 📊 **Scalable**: Handles large repositories efficiently

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.