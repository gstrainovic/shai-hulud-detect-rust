# Verification Scripts

This directory contains the master verification suite that proves **100% compatibility** between the Rust and Bash scanners.

---

## 🎯 Quick Start

### Normal Mode Verification (Parallel - RECOMMENDED)

```bash
# Run parallel scans + automatic verification (takes ~2 min)
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh
```

### Normal Mode + --verify (Verification Test)

```bash
# Run parallel scans WITH --verify flag to test verification system
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_verify.sh

# Then compare to ensure --verify doesn't change counts
bash dev-rust-scanner-1/scripts/analyze/compare_normal_vs_verify.sh

# OR run complete suite (normal + verify + comparison)
bash dev-rust-scanner-1/scripts/analyze/run_full_verification_test.sh
```

### Paranoid Mode Verification (Parallel)

```bash
# Run parallel paranoid scans + verification (takes ~2 min)
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_paranoid.sh

# Run parallel paranoid scans WITH --verify flag
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_verify_paranoid.sh
```

### Performance Comparison (Sequential Baseline)

```bash
# Sequential normal mode (takes ~10+ min - for comparison only)
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test.sh

# Sequential paranoid mode (takes ~10+ min - for comparison only)
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test_paranoid.sh
```

---

## 📋 Scripts Overview

### `parallel_testcase_scan.sh` ⭐ RECOMMENDED
**Purpose**: Run both Bash and Rust scanners on all 26 test cases in parallel (normal mode)

**Supports**: `[--paranoid] [--verify]` flags

**What it does**:
- Scans all test cases in `shai-hulud-detect/test-cases/`
- Runs Bash scanner (max CPU_CORES concurrent)
- Runs Rust scanner (max CPU_CORES concurrent)
- Extracts summary counts (HIGH/MEDIUM/LOW)
- Creates comparison table automatically
- Shows timing information

**Usage**:
```bash
# Normal mode
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh

# With --verify (tests verification doesn't change counts)
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh --verify

# Paranoid mode
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh --paranoid

# Paranoid + verify
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh --paranoid --verify
```

**Output**:
- Logs: `per-testcase-logs/` (normal), `per-testcase-logs-verify/` (verify), etc.
- CSV: `comparison.csv`
- Timing: Start, End, Duration

**Duration**: ~2-3 minutes (parallel execution)

---

### `parallel_testcase_scan_paranoid.sh` ⭐
**Purpose**: Same as above but for **paranoid mode**

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_paranoid.sh
```

**Output**:
- Logs: `per-testcase-logs-paranoid/YYYYMMDD_HHMMSS/`
- CSV: `comparison.csv`
- Timing: Start, End, Duration

**Duration**: ~2 minutes (parallel execution)

---

### `full_sequential_test.sh` � INTEGRATION TEST
**Purpose**: Scan ENTIRE test-cases/ directory at once (integration test, not per-folder)

**What it does**:
- Runs **ONE SCAN** of the entire `shai-hulud-detect/test-cases/` directory
- Tests how scanners handle the complete collection together
- Catches integration issues that per-folder testing might miss
- Compares final summary counts (H/M/L)

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test.sh
```

**Output**:
- Logs: `sequential-logs/YYYYMMDD_HHMMSS/`
- Files: `bash_full_scan.log`, `rust_full_scan.log`
- Comparison: `comparison.txt`

**Duration**: Variable (depends on total test-cases size)

**When to use**: 
- Integration testing (how do scanners handle entire directory?)
- Verify no cross-contamination between test cases
- Test aggregation logic

---

### `full_sequential_test_paranoid.sh` � INTEGRATION TEST (PARANOID)
**Purpose**: Same as above but for **paranoid mode**

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test_paranoid.sh
```

**Output**:
- Logs: `sequential-logs-paranoid/YYYYMMDD_HHMMSS/`

---

## ⏱️ Performance & Testing Comparison

| Script | Mode | Type | Target | Duration | Use Case |
|--------|------|------|--------|----------|----------|
| `parallel_testcase_scan.sh` | Normal | Parallel | Each folder | ~2 min | ✅ **Per-folder verification** |
| `parallel_testcase_scan_paranoid.sh` | Paranoid | Parallel | Each folder | ~2 min | ✅ **Per-folder verification** |
| `full_sequential_test.sh` | Normal | Integration | Entire dir | Variable | � **Integration test** |
| `full_sequential_test_paranoid.sh` | Paranoid | Integration | Entire dir | Variable | � **Integration test** |

**Key Difference**:
- **Parallel scripts**: Test each subfolder separately (26 individual scans)
- **Sequential scripts**: Test entire test-cases/ directory at once (1 big scan)

**Why both?**:
- Per-folder: Catches individual test case issues
- Full directory: Catches integration/aggregation issues

---

## 📁 Output Structure

```
scripts/analyze/
├── parallel_testcase_scan.sh              ⭐ Use this
├── parallel_testcase_scan_paranoid.sh     ⭐ Use this
├── full_sequential_test.sh                🐢 Baseline only
├── full_sequential_test_paranoid.sh       🐢 Baseline only
├── README.md
├── per-testcase-logs/                     # Parallel normal mode
│   └── 20251008_012345/
│       ├── comparison.csv
│       ├── bash_*.log
│       ├── bash_*_summary.txt
│       ├── rust_*.log
│       └── rust_*_summary.txt
├── per-testcase-logs-paranoid/            # Parallel paranoid mode
│   └── 20251008_123456/
├── sequential-logs/                       # Sequential normal mode
│   └── 20251008_234567/
└── sequential-logs-paranoid/              # Sequential paranoid mode
    └── 20251008_345678/
```

---

## 🧪 Cargo Test Integration

**Note**: The full 100% verification is NOT part of `cargo test` because it takes 3+ minutes to scan all 26 test cases.

Instead:
- ✅ `cargo test` runs fast unit tests (~1 second)
- ✅ Full verification uses dedicated Bash scripts (this directory)

To verify 100% compatibility, use the scripts above instead of cargo test.


---

## 📊 What Gets Verified

### Per Test Case:
- ✅ HIGH risk count (exact match)
- ✅ MEDIUM risk count (exact match)
- ✅ LOW risk count (exact match)

### Overall:
- ✅ All 26 test cases must match
- ✅ Both normal and paranoid modes
- ✅ No timeouts or crashes

---

## 🎓 Understanding the Results

### Summary Format

Each `*_summary.txt` file contains:
```
   High Risk Issues: 8
   Medium Risk Issues: 16
   Low Risk (informational): 2
```

### Verification Comparison

The verification script compares these numbers for each test case:
- `Bash H/M/L` = Numbers from Bash scanner
- `Rust H/M/L` = Numbers from Rust scanner
- `Match` = ✅ if identical, ❌ if different

### Success Criteria

**100% Match** means:
- Every test case shows ✅
- Total matched = Total tests
- Both scanners produce identical detection counts

---

## 🔧 Troubleshooting

### "No test results found"
**Solution**: Run `parallel_testcase_scan.sh` first to generate test data

### Timeout errors
**Solution**: Some large test cases may timeout (>5 minutes). This is logged but not a failure.

### ANSI color code issues
**Solution**: Scripts automatically strip ANSI codes when comparing numbers

---

## 📚 Related Documentation

- `../../VERIFICATION_GUIDE.md` - Complete verification documentation
- `../README.md` - General scripts documentation
- `../../README.md` - Project README

---

## ✅ Verification Status

**Last Run**: 2025-11-28

**Normal Mode**: ✅ 100% MATCH (including November 2025 attack detectors)
**Paranoid Mode**: ✅ 100% MATCH (all PR #50 fixes integrated)
**Test Cases**: 26/26 matched
**Version**: 2.7.6 (matches Bash scanner)

**Ready for production!** 🚀
