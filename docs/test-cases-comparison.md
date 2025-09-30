# Test Cases Comparison: JSON vs Physical Folders

## Summary

- **Total test cases in JSON**: 22
- **Folders in Bash test-cases (shai-hulud-detect/test-cases)**: 23
- **Folders in Rust tests/test-cases (shai-hulud-detect-rust/tests/test-cases)**: 6
- **Test cases in JSON with Bash paths (test-cases/)**: 19
- **Test cases in JSON with Rust paths (tests/test-cases/)**: 3
- **Missing in JSON from Bash folders**: lockfile-compromised, lockfile-false-positive, xmlhttp-legitimate, xmlhttp-malicious
- **Missing in JSON from Rust folders**: crypto-theft-test, postinstall-hooks-test
- **Risk Levels in test-cases**: Based on README.md where available, specifying HIGH, MEDIUM, or LOW RISK expectations
- **Comments in JSON**: Detailed per-file comments explaining patterns, purposes, and risk rationales
- **Comments in test-cases**: README.md files with test purpose, patterns, and expected detection (only in some Rust test-cases)

## Risk Levels in the Two Test-Case Folders

### Bash Test-Cases (shai-hulud-detect/test-cases)
- Most folders do not have explicit README.md with risk levels
- Based on names and content, they cover various risks: HIGH (infected-project, chalk-debug-attack), MEDIUM (legitimate-crypto, typosquatting), LOW (namespace-warning, false-positive), OK (clean-project)

### Rust Test-Cases (shai-hulud-detect-rust/tests/test-cases)
- README.md specify expected risks:
  - extended-network-exfiltration: HIGH RISK
  - extended-typosquatting-test: MEDIUM RISK
  - shai-hulud-repo-detection: MEDIUM RISK (note: README says MEDIUM, but JSON expects HIGH)
- Other folders (crypto-theft-test, postinstall-hooks-test, git-branch-test) have no README, but git-branch-test is in JSON with MEDIUM

## Missing Risk Assessments
- **Folders missing in JSON**: lockfile-compromised, lockfile-false-positive, xmlhttp-legitimate, xmlhttp-malicious, crypto-theft-test, postinstall-hooks-test
- **Is the missing risk assessment correct?** Yes, because these folders are not included in the test_verification_detailed.json, meaning they are not part of the current test suite expectations. They may be legacy or additional test cases not yet integrated into the verification framework.

## Detailed Comparison Table

| Test Case | JSON Path | Folder Location | Exists | Expected Risks (JSON) | Risk Levels (Files in JSON) | JSON Comment Summary | Test-case Comment Summary | Differences/Notes |
|-----------|-----------|-----------------|--------|-----------------------|-----------------------------|----------------------|---------------------------|-------------------|
| clean-project | test-cases/clean-project | Bash | Yes | [] | OK | Clean package.json with legitimate dependencies. No suspicious packages or patterns. | No README | - |
| infected-project | test-cases/infected-project | Bash | Yes | HIGH, MEDIUM | LOW (package.json), HIGH (multiple files) | Contains compromised packages (MEDIUM), various malicious patterns (HIGH). | No README | - |
| chalk-debug-attack | test-cases/chalk-debug-attack | Bash | Yes | HIGH | MEDIUM (package.json), HIGH (js files) | Compromised chalk/debug packages (MEDIUM), actual attack implementation (HIGH). | No README | - |
| network-exfiltration-project | test-cases/network-exfiltration-project | Bash | Yes | HIGH | HIGH | Multiple exfiltration patterns: IPs, webhook.site, Base64, WebSocket. | No README | - |
| legitimate-crypto | test-cases/legitimate-crypto | Bash | Yes | MEDIUM | MEDIUM, OK | Legitimate crypto libraries triggering medium-risk warnings. | No README | - |
| common-crypto-libs | test-cases/common-crypto-libs | Bash | Yes | MEDIUM | MEDIUM, OK | Common crypto libs, should not be HIGH-risk false positives. | No README | - |
| typosquatting-project | test-cases/typosquatting-project | Bash | Yes | MEDIUM | MEDIUM | Typosquatting packages with character omissions and Unicode substitutions. | No README | - |
| comprehensive-test | test-cases/comprehensive-test | Bash | Yes | HIGH | MEDIUM (package.json), HIGH (js) | Comprehensive threats including typosquatting and high-risk exfiltration. | No README | - |
| mixed-project | test-cases/mixed-project | Bash | Yes | MEDIUM | MEDIUM | Mixed suspicious elements, ambivalent webhook usage. | No README | - |
| semver-matching | test-cases/semver-matching | Bash | Yes | MEDIUM | MEDIUM | Semver ranges that could include compromised versions. | No README | - |
| namespace-warning | test-cases/namespace-warning | Bash | Yes | LOW | LOW | Packages from affected namespaces but safe versions. | No README | - |
| debug-js | test-cases/debug-js | Bash | Yes | MEDIUM | MEDIUM | Debug package triggering crypto theft patterns. | No README | - |
| legitimate-security-project | test-cases/legitimate-security-project | Bash | Yes | MEDIUM | LOW, MEDIUM | Legitimate security tool using TruffleHog. | No README | - |
| false-positive-project | test-cases/false-positive-project | Bash | Yes | LOW | LOW, OK | Legitimate code with expected low-level pattern matches. | No README | - |
| edge-case-project | test-cases/edge-case-project | Bash | Yes | LOW | LOW | Documentation mentioning credentials legitimately. | No README | - |
| infected-lockfile | test-cases/infected-lockfile | Bash | Yes | [] | OK | Standard package-lock.json, appears clean. | No README | - |
| infected-lockfile-pnpm | test-cases/infected-lockfile-pnpm | Bash | Yes | HIGH | HIGH | pnpm-lock.yaml with potentially compromised package. | No README | - |
| git-branch-test | test-cases/git-branch-test | Bash | Yes | MEDIUM | MEDIUM (multiple) | Suspicious git branches like 'fix-prod-bug'. | No README | Also exists in Rust folder |
| multi-hash-detection | test-cases/multi-hash-detection | Bash | Yes | [] | OK | Test files with UUIDs for hash detection. | No README | - |
| shai-hulud-repo-detection | tests/test-cases/shai-hulud-repo-detection | Rust | Yes | HIGH | HIGH | Repository with Shai-Hulud naming patterns. | Tests detection of shai-hulud repos and migration patterns. Expected MEDIUM RISK (README) but HIGH in JSON. | Risk mismatch: README says MEDIUM, JSON expects HIGH |
| extended-typosquatting-test | tests/test-cases/extended-typosquatting-test | Rust | Yes | MEDIUM | MEDIUM | Comprehensive typosquatting with 25+ packages and Unicode substitutions. | Tests comprehensive typosquatting detection. Expected MEDIUM RISK. | - |
| extended-network-exfiltration | tests/test-cases/extended-network-exfiltration | Rust | Yes | HIGH | HIGH | Comprehensive network exfiltration with 15+ domains and private IPs. | Tests detection of suspicious domains and IPs. Expected HIGH RISK. | - |
| lockfile-compromised | - | Bash | No (missing in JSON) | - | - | - | No README | Missing in JSON |
| lockfile-false-positive | - | Bash | No (missing in JSON) | - | - | - | No README | Missing in JSON |
| xmlhttp-legitimate | - | Bash | No (missing in JSON) | - | - | - | No README | Missing in JSON |
| xmlhttp-malicious | - | Bash | No (missing in JSON) | - | - | - | No README | Missing in JSON |
| crypto-theft-test | - | Rust | No (missing in JSON) | - | - | - | No README | Missing in JSON |
| postinstall-hooks-test | - | Rust | No (missing in JSON) | - | - | - | No README | Missing in JSON |

## Comments in Test-Cases
- Only Rust test-cases have README.md with comments: extended-network-exfiltration, extended-typosquatting-test, shai-hulud-repo-detection
- Bash test-cases have no README.md, so no explicit comments beyond folder names
- JSON has detailed comments for each file, explaining patterns and purposes

## Comments in JSON
- Extensive per-file comments explaining what each file tests, expected patterns, and risk rationales
- Examples: "Contains compromised packages: @ctrl/deluge@1.2.0... The compromised packages are detected as separate MEDIUM RISK entries"
- Purposes: "Tests detection of webhook.site URLs as exfiltration endpoints"</content>
<parameter name="filePath">c:\Users\gstra\Code\shai-hulud-detect-rust\docs\test-cases-comparison.md