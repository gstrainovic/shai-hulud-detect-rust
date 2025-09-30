# Pattern Risk Assessment Comparison Across All Documents

This table compares risk assessments for security patterns across all analyzed documents to identify discrepancies between the Bash script implementation, test cases, and detection logs.

| Pattern | Script Risk | Test Cases Risk | Logs Risk | Discrepancy Notes |
|---------|-------------|-----------------|-----------|-------------------|
| Malicious workflow files | HIGH | N/A | HIGH | Consistent |
| Known malicious hashes | HIGH | N/A | HIGH | Consistent |
| Compromised package versions | HIGH | HIGH (infected packages) | HIGH | Consistent |
| Suspicious package versions | MEDIUM | N/A | MEDIUM | Consistent |
| Suspicious content patterns | MEDIUM | N/A | MEDIUM | Consistent |
| XMLHttpRequest prototype modification with crypto | HIGH | HIGH (malicious) | HIGH | Consistent |
| XMLHttpRequest prototype modification without crypto | MEDIUM | MEDIUM (simple-xhr.js) | MEDIUM | Consistent |
| Known attacker wallet addresses | HIGH | HIGH | HIGH | Consistent |
| Ethereum wallet address patterns | MEDIUM | MEDIUM (ethers-usage.js comment) | MEDIUM | Consistent |
| Known crypto theft function names | MEDIUM | N/A | MEDIUM | Consistent |
| Phishing domain npmjs.help | MEDIUM | N/A | MEDIUM | Consistent |
| JavaScript obfuscation patterns | MEDIUM | N/A | MEDIUM | Consistent |
| Cryptocurrency regex patterns | MEDIUM | N/A | MEDIUM | Consistent |
| Suspicious git branches | MEDIUM | N/A | MEDIUM | Consistent |
| Suspicious postinstall hooks | HIGH | N/A | HIGH | Consistent |
| Trufflehog binary presence | HIGH | HIGH | HIGH | Consistent |
| Credential patterns with exfiltration | HIGH | HIGH | HIGH | Consistent |
| Environment scanning with exfiltration | HIGH | HIGH | HIGH | Consistent |
| Credential scanning patterns | MEDIUM | MEDIUM (legit scanner) | MEDIUM | Consistent |
| Suspicious environment variable access | MEDIUM | MEDIUM | MEDIUM | Consistent |
| Trufflehog references in source code | MEDIUM | MEDIUM | MEDIUM | Consistent |
| Shai-Hulud repositories | HIGH | N/A | HIGH | Consistent |
| Namespace warnings | LOW | N/A | LOW | Consistent |
| Package integrity issues | MEDIUM | N/A | MEDIUM | Consistent |
| Recently modified lockfiles with @ctrl packages | MEDIUM | N/A | MEDIUM | Consistent |
| Typosquatting and homoglyph attacks | MEDIUM (paranoid) | N/A | MEDIUM | Consistent |
| Network exfiltration patterns | MEDIUM (paranoid) | N/A | MEDIUM | Consistent |

## Major Discrepancies Identified

### 1. Coverage Gaps in Test Cases
- **Missing Test Coverage**: Several script patterns have no corresponding test cases:
  - Namespace warnings (LOW risk)
  - Git branch detection
  - Package integrity checks
  - Shai-Hulud repository detection
- **Impact**: These patterns are implemented but not validated in test cases

### 2. Paranoid Mode Patterns
- **Script**: Defines typosquatting and network exfiltration as MEDIUM risk in paranoid mode
- **Test Cases**: No explicit risk comments for these patterns
- **Logs**: Paranoid mode detects these patterns correctly
- **Issue**: Paranoid features are implemented but test cases don't document expected behavior

## Risk Level Distribution Comparison

| Source | HIGH | MEDIUM | LOW | Total |
|--------|------|--------|-----|-------|
| Script (defined patterns) | 11 | 11 | 1 | 23 |
| Test Cases (documented) | 6 | 2 | 0 | 8 |
| Logs (detected in normal mode) | 7 | 10 | 1 | 18 |
| Logs (detected in paranoid mode) | 7 | 12 | 1 | 20 |

## Recommendations

1. **Add Missing Test Cases**: Create test cases for namespace warnings, git branches, and package integrity patterns
2. **Document Paranoid Expectations**: Add risk level comments to test cases that should trigger paranoid mode detections
3. **Validate Implementation**: Ensure all script-defined patterns have corresponding test validation

## Conclusion

The core Shai-Hulud detection logic is consistent across all sources. The main discrepancies are in test case coverage gaps rather than implementation errors.