# Extended Typosquatting Test

This test case contains comprehensive typosquatting patterns including Cyrillic and Unicode character substitutions.

## Test Purpose
- Tests detection of 25+ popular package typosquats
- Tests Cyrillic character substitutions (е instead of e, х instead of x, etc.)
- Tests character omission variants (webpck, bable, eslnt)
- Tests PARANOID MODE typosquatting detection

## Typosquatting Patterns
- **Cyrillic substitutions**: rеact, vuе, ехpress, аngular, lоdash, aхios, etc.
- **Character omissions**: webpck (webpack), bable (babel), eslnt (eslint)
- **Unicode homoglyphs**: Various lookalike characters

## Expected Detection
- Should be classified as MEDIUM RISK
- Should trigger comprehensive typosquatting warnings