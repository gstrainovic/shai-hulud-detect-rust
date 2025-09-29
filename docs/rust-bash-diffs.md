# Rust-Bash Differences Analysis

This document tracks intentional and unintentional differences between the Rust implementation and the original Bash shai-hulud-detector.sh script.

## Purpose
- Document legitimate differences between implementations
- Track areas where alignment is needed
- Provide rationale for divergences
- Help maintainers understand implementation choices

## Risk Level Classifications

### Compromised Packages
**Status**: ✅ **ALIGNED**

- **Bash Implementation**: Classifies compromised packages as `MEDIUM RISK`
- **Rust Implementation**: Classifies compromised packages as `MEDIUM RISK`
- **Rationale**: Despite being confirmed compromised packages from compromised-packages.txt, both implementations treat them as MEDIUM RISK requiring manual review, not automatic HIGH RISK

**Evidence from Bash log:**
```bash
⚠️  MEDIUM RISK: Suspicious package versions detected:
   - Package: @ctrl/deluge@1.2.0
     Found in: /c/Users/gstra/Code/shai-hulud-detect/test-cases/infected-project/package.json
   - Package: @nativescript-community/ui-material-core@7.2.49
     Found in: /c/Users/gstra/Code/shai-hulud-detect/test-cases/infected-project/package.json
```

**Possible Rationale:**
- Avoids false positives in legitimate projects that may have outdated but not necessarily malicious packages
- Encourages manual verification rather than automatic blocking
- Aligns with principle of "trust but verify" for package management
Manual review required to determine if these are malicious."

Bash analyse:
Das erklärt die Designentscheidung! Die Bash-Version fügt explizit hinzu: "Manual review required to determine if these are malicious."
Das zeigt, dass selbst bekannte kompromittierte Pakete als MEDIUM RISK eingestuft werden, um manuelle Überprüfung zu fördern, statt automatische Blockierung.

## Output Format Differences

### Category Order
**Status**: ✅ **ALIGNED**

Both implementations now show HIGH RISK categories first, then MEDIUM RISK, then LOW RISK.

### Package Formatting  
**Status**: ✅ **ALIGNED**

Both implementations use the format:
```
- Package: name@version
  Found in: /path/to/package.json
```

## Areas Requiring Further Investigation

### TBD: Other Risk Level Mismatches
- Several test cases still show different risk levels between Bash and Rust
- Need to analyze each case to determine if alignment is needed

### TBD: Pattern Detection Differences
- Some patterns may be detected differently between implementations
- Requires detailed analysis of both codebases

## Methodology
1. Compare outputs from both implementations on identical test cases
2. Analyze bash-testcase.log vs Rust E2E test results
3. Review source code for intentional design decisions
4. Document rationale for differences
5. Align implementations where appropriate

## Status Legend
- ✅ **ALIGNED**: Implementations match expected behavior
- ⚠️ **INVESTIGATION NEEDED**: Differences require analysis
- ❌ **MISALIGNED**: Implementations should match but don't
- 📝 **INTENTIONAL**: Documented intentional difference