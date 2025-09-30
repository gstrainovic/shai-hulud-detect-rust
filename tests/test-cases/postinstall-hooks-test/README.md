# Postinstall Hooks Test

This test case contains suspicious postinstall hooks that download and execute remote malware.

## Test Purpose
- Tests detection of malicious npm lifecycle scripts
- Tests detection of wget/curl commands downloading malware
- Tests detection of remote code execution during package installation
- Tests HIGH RISK classification for postinstall compromises

## Expected Detection
- Should be classified as HIGH RISK
- Should trigger postinstall hook and remote execution warnings</content>
<parameter name="filePath">c:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases\postinstall-hooks-test\README.md