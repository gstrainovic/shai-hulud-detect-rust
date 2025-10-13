# Repository Instructions for GitHub Copilot

## Project Overview
This is a Rust-based scanner for detecting npm supply chain attacks (Shai-Hulud detection). It works alongside a bash scanner and includes comprehensive test infrastructure for validation.

## Development Workflow

### Code Execution Rules
- **Always use bash scripts** - Never send commands directly to bash terminal
- **Temporary scripts** (one-time use) go in `/tmp/` directory
- **Reusable scripts** go in `scripts/` or appropriate subdirectories
- **Commit measurable progress** with temporary scripts included
- **Clean up** - Remove unnecessary docs, scripts, and debug files before final commits

### Script Conventions
```bash
#!/bin/bash
# All scripts must:
# 1. Have descriptive header comments
# 2. Use absolute paths
# 3. Handle errors gracefully
# 4. Print clear progress messages
```

### Testing Standards
- Run tests via `scripts/analyze/` test scripts
- Normal mode: `parallel_testcase_scan.sh`
- Paranoid mode: `parallel_testcase_scan_paranoid.sh`
- Count-based verification (H/M/L findings must match)
- Test results go in `scripts/analyze/per-testcase-logs/` or `per-testcase-logs-paranoid/`

### Commit Guidelines
- Commit after each measurable success
- Include temporary test scripts if they demonstrate the fix
- Use emoji prefixes: 🐛 (bug), ✨ (feature), 📝 (docs), 🧪 (tests)
- Keep commits focused and atomic

## Project Structure

### Core Components
- `src/` - Rust scanner source code
  - `detectors/` - Individual detection modules
  - `main.rs` - Entry point
  - `cli.rs` - Command-line interface
  - `report.rs` - Output formatting

### Test Infrastructure
- `scripts/analyze/` - Test execution scripts
  - `parallel_testcase_scan.sh` - Normal mode tests
  - `parallel_testcase_scan_paranoid.sh` - Paranoid mode tests
  - `per-testcase-logs/` - Test results (normal)
  - `per-testcase-logs-paranoid/` - Test results (paranoid)

### External Dependencies
- `shai-hulud-detect/` - Original bash scanner (external repo)
- `shai-hulud-detect/test-cases/` - Shared test cases

## Key Tasks & Context

### Current Status
- Rust scanner is functionally complete
- Count-based validation works (H/M/L findings match)
- Pattern-level verification was removed (too buggy/complex)

### Known Issues
- Bash scanner has network exfiltration detection bug (hostname patterns)
- Some test cases have artifacts (scan_results.json) that need cleanup

### TODO Tracking
- `TODO.md` - Main task list
- Tasks are marked with: ✅ (done), ⏳ (in progress), ❌ (blocked)
- Always check TODO.md before starting work

## Testing Workflow

### Before Making Changes
1. Check current TODO.md status
2. Clean up any scan_results.json artifacts: `find shai-hulud-detect/test-cases -name "scan_results.json" -delete`

### After Making Changes
1. Run appropriate test script (normal or paranoid)
2. Verify counts match (H/M/L)
3. Commit if successful
4. Update TODO.md

### Test Execution Example
```bash
# Always via script, never direct commands!
bash scripts/analyze/parallel_testcase_scan.sh
# Check results in output
```

## Common Operations

### Running Tests
```bash
# Create test script
cat > /tmp/run_tests.sh << 'EOF'
#!/bin/bash
cd /c/Users/gstra/Code/rust-scanner
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh
EOF

bash /tmp/run_tests.sh
```

### Cleanup Artifacts
```bash
# Create cleanup script
cat > /tmp/cleanup.sh << 'EOF'
#!/bin/bash
find /c/Users/gstra/Code/rust-scanner/shai-hulud-detect/test-cases -name "scan_results.json" -delete
echo "✅ Cleaned up scan_results.json artifacts"
EOF

bash /tmp/cleanup.sh
```

### Quick Verification
```bash
# Create verification script
cat > /tmp/verify_counts.sh << 'EOF'
#!/bin/bash
cd /c/Users/gstra/Code/rust-scanner
timeout 300 shai-hulud-detect/shai-hulud-detector.sh shai-hulud-detect/test-cases/infected-project | grep -E "High Risk|Medium Risk|Low Risk"
EOF

bash /tmp/verify_counts.sh
```

## Debugging Guidelines

### When Tests Fail
1. Check count mismatch (expected vs actual)
2. Run single test case in isolation
3. Compare bash vs rust outputs manually
4. Look for scan_results.json artifacts
5. Check if bash scanner changed (git pull)

### Debug Script Template
```bash
cat > /tmp/debug_test.sh << 'EOF'
#!/bin/bash
TESTCASE="$1"
echo "🔍 Testing: $TESTCASE"
# Bash scan
shai-hulud-detect/shai-hulud-detector.sh "shai-hulud-detect/test-cases/$TESTCASE" > /tmp/bash_result.log 2>&1
# Rust scan
cd dev-rust-scanner-1
cargo run --quiet --release -- "../shai-hulud-detect/test-cases/$TESTCASE" > /tmp/rust_result.log 2>&1
# Compare
echo "Bash counts:"
grep -E "High Risk|Medium Risk|Low Risk" /tmp/bash_result.log
echo "Rust counts:"
grep -E "High Risk|Medium Risk|Low Risk" /tmp/rust_result.log
EOF

bash /tmp/debug_test.sh infected-project
```

## Important Notes

### Do NOT
- ❌ Send raw commands to bash terminal
- ❌ Create permanent debug scripts in project root
- ❌ Commit unnecessary documentation
- ❌ Leave scan_results.json artifacts in test cases
- ❌ Run tests without scripts

### Always DO
- ✅ Use bash scripts for all operations
- ✅ Put temporary scripts in /tmp/
- ✅ Clean up before committing
- ✅ Update TODO.md with progress
- ✅ Commit after measurable success
- ✅ Check for artifacts before running tests

## External Repositories

### shai-hulud-detect (Bash Scanner)
- Owner: Cobenian
- Current branch: main
- Our changes: Network exfiltration fix (pending PR)
- Update frequency: Check occasionally for upstream changes

## Performance Targets
- Normal mode tests: ~2 minutes for 26 test cases
- Paranoid mode tests: ~3 minutes for 26 test cases
- Single test case: <5 seconds

## Success Metrics
- ✅ All 26 test cases pass count verification
- ✅ No mismatch between bash and rust counts (H/M/L)
- ✅ Clean test runs (no artifacts, no errors)
- ✅ TODO.md up to date
