# Shai-Hulud Scanner (Rust Implementation)

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-Active-success)](#overview)

A fast, reliable Rust implementation of the Shai-Hulud NPM supply chain attack detector. This scanner provides comprehensive detection of the September 2025 npm supply chain attacks, including the Shai-Hulud self-replicating worm and the chalk/debug crypto theft attack.

## Features

- 🚀 **Fast Performance**: Written in Rust for optimal scanning speed
- 🔍 **Comprehensive Detection**: Covers 621+ compromised packages and malicious patterns
- 📊 **JSON Output**: Structured results for CI/CD integration
- 🛡️ **Paranoid Mode**: Additional security checks for thorough analysis
- 🔧 **CI/CD Ready**: Proper exit codes for automated workflows
- 📁 **Hash Verification**: Detects known malicious files by SHA-256 hash
- 🎯 **Pattern Matching**: Advanced regex patterns for suspicious content

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/shai-hulud-detect-rust.git
cd shai-hulud-detect-rust

# Build the scanner
cargo build --release

# The binary will be available at target/release/shai-hulud-scanner
```

### Basic Usage

```bash
# Scan a Node.js project
./target/release/shai-hulud-scanner /path/to/your/project

# Scan with paranoid mode (additional security checks)
./target/release/shai-hulud-scanner --paranoid /path/to/your/project

# Output results in JSON format
./target/release/shai-hulud-scanner --json --output results.json /path/to/your/project

# Quiet mode (only show summary)
./target/release/shai-hulud-scanner --quiet /path/to/your/project
```

### Exit Codes

The scanner uses standard exit codes for CI/CD integration:
- `0`: Clean - no issues found
- `1`: Medium risk - issues require manual review  
- `2`: High risk - immediate action required

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

The scanner includes comprehensive test coverage using the test cases from the bash implementation:

```bash
# Test on clean project (should exit 0)
cargo run -- ../shai-hulud-detect/test-cases/clean-project

# Test on infected project (should exit 2)  
cargo run -- ../shai-hulud-detect/test-cases/infected-project

# Test paranoid mode
cargo run -- --paranoid ../shai-hulud-detect/test-cases/comprehensive-test

# Test JSON output
cargo run -- --json --output test_results.json ../shai-hulud-detect/test-cases/infected-project
```

### Available Test Cases

- `clean-project`: Baseline clean project
- `infected-project`: Multiple high-risk indicators
- `chalk-debug-attack`: Crypto theft patterns
- `comprehensive-test`: Combined malicious patterns
- `legitimate-crypto`: False positive testing
- `typosquatting-project`: Package name attacks
- `network-exfiltration-project`: Data exfiltration patterns

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