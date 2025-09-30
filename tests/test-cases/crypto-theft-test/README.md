# Crypto Theft Test

This test case contains cryptocurrency theft patterns and wallet hijacking malware.

## Test Purpose
- Tests detection of XMLHttpRequest prototype modifications for crypto theft
- Tests detection of known attacker wallet addresses
- Tests detection of phishing domains like npmjs.help
- Tests HIGH RISK classification for crypto malware

## Expected Detection
- Should be classified as HIGH RISK
- Should trigger cryptocurrency theft and wallet hijacking warnings</content>
<parameter name="filePath">c:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases\crypto-theft-test\README.md