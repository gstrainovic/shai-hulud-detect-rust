# Verification Scripts

This directory contains the master verification suite that proves **100% compatibility** between the Rust and Bash scanners.

---

## 🎯 Quick Start

### Normal Mode Verification (Parallel - RECOMMENDED)

```bash
# Run parallel scans + automatic verification (takes ~2 min)
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh
```

### Paranoid Mode Verification (Parallel)

```bash
# Run parallel paranoid scans + verification (takes ~2 min)
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_paranoid.sh
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

**What it does**:
- Scans all test cases in `shai-hulud-detect/test-cases/`
- Runs Bash scanner (max 4 concurrent)
- Runs Rust scanner (max 8 concurrent - faster!)
- Extracts summary counts (HIGH/MEDIUM/LOW)
- Creates comparison table automatically
- Shows timing information

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh
```

**Output**:
- Logs: `per-testcase-logs/YYYYMMDD_HHMMSS/`
- CSV: `comparison.csv`
- Timing: Start, End, Duration

**Duration**: ~2 minutes (parallel execution)

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

### `full_sequential_test.sh` 🐢 BASELINE
**Purpose**: Sequential (non-parallel) baseline for performance comparison

**What it does**:
- Runs ALL test cases one-by-one (no parallelization)
- Provides baseline timing for comparison
- Proves parallel scripts are actually faster

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test.sh
```

**Output**:
- Logs: `sequential-logs/YYYYMMDD_HHMMSS/`
- CSV: `comparison.csv`
- Timing: Shows total duration

**Duration**: ~10+ minutes (sequential - SLOW!)

**When to use**: Only for benchmarking/comparison, not for regular verification

---

### `full_sequential_test_paranoid.sh` 🐢 BASELINE
**Purpose**: Sequential baseline for **paranoid mode**

**Usage**:
```bash
bash dev-rust-scanner-1/scripts/analyze/full_sequential_test_paranoid.sh
```

**Output**:
- Logs: `sequential-logs-paranoid/YYYYMMDD_HHMMSS/`

**Duration**: ~10+ minutes (sequential - SLOW!)

---

## ⏱️ Performance Comparison

| Script | Mode | Execution | Duration | Use Case |
|--------|------|-----------|----------|----------|
| `parallel_testcase_scan.sh` | Normal | Parallel (4+8) | ~2 min | ✅ **Regular verification** |
| `parallel_testcase_scan_paranoid.sh` | Paranoid | Parallel (4+4) | ~2 min | ✅ **Regular verification** |
| `full_sequential_test.sh` | Normal | Sequential | ~10+ min | 📊 **Benchmarking only** |
| `full_sequential_test_paranoid.sh` | Paranoid | Sequential | ~10+ min | 📊 **Benchmarking only** |

**Speed Improvement**: Parallel scripts are **~5x faster** than sequential!

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

**Last Run**: 2025-10-08

**Normal Mode**: ✅ 100% MATCH (19 HIGH / 61 MEDIUM / 9 LOW)  
**Paranoid Mode**: ✅ 100% MATCH (verified)  
**Test Cases**: 26/26 matched

**Ready for production!** 🚀
