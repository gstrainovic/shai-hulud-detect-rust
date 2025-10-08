# 📚 SCRIPT ORGANIZATION

All scripts organized into logical categories for easy navigation and maintenance.

---

## 📁 FOLDER STRUCTURE

```
scripts/
├── verification/     # Production verification scripts (run these!)
├── analysis/         # Analysis & comparison tools
├── debug/           # Debugging & investigation tools
├── archive/         # Old/deprecated scripts
└── cleanup_all.sh   # Main cleanup script
```

---

## 🔍 ANALYSIS (Understanding Differences)

**Location**: `scripts/analysis/`

### exact_bash_high_count.sh
**Purpose**: Detailed breakdown of Bash HIGH count methodology  
**When to use**: Understanding how Bash counts HIGH findings  
**What it does**:
- Counts each HIGH category separately
- Shows compromised packages, crypto, workflows, trufflehog
- Explains Bash counting logic
**Usage**: `bash scripts/analysis/exact_bash_high_count.sh`

### exact_rust_high_breakdown.sh
**Purpose**: Detailed breakdown of Rust HIGH count methodology  
**When to use**: Understanding how Rust counts HIGH findings  
**What it does**:
- Shows Rust's internal counting logic
- Displays debug output
- Compares with expected values
**Usage**: `bash scripts/analysis/exact_rust_high_breakdown.sh`

### compare_high_counts.sh
**Purpose**: Side-by-side comparison of HIGH counting methods  
**When to use**: Finding discrepancies between scanners  
**What it does**:
- Compares Bash vs Rust HIGH categories
- Shows detailed package counts
- Identifies counting differences
**Usage**: `bash scripts/analysis/compare_high_counts.sh`

### rust_high_breakdown.sh
**Purpose**: Rust HIGH count by category  
**When to use**: Quick Rust analysis  
**Usage**: `bash scripts/analysis/rust_high_breakdown.sh`

### analyze_bash_high.sh
**Purpose**: Analyze Bash HIGH counting methodology  
**When to use**: Investigating Bash logic  
**What it does**:
- Counts each type of HIGH finding
- Shows pattern analysis
**Usage**: `bash scripts/analysis/analyze_bash_high.sh`

### analyze_bash_counting.sh
**Purpose**: Deep dive into Bash counting logic  
**When to use**: Understanding exact Bash methodology  
**Usage**: `bash scripts/analysis/analyze_bash_counting.sh`

---

## 🐛 DEBUG (Investigation & Bug Hunting)

**Location**: `scripts/debug/`

### find_high_diff.sh
**Purpose**: Find EXACT differences in HIGH counts  
**When to use**: Rust shows different HIGH count than Bash  
**What it does**:
- Tests each test-case individually
- Shows which cases have different HIGH counts
- Identifies mismatches
**Usage**: `bash scripts/debug/find_high_diff.sh`

### debug_rust_compromised.sh
**Purpose**: Debug Rust compromised package detection  
**When to use**: Issues with package detection  
**What it does**:
- Shows full compromised packages section
- Displays Rust's internal findings
**Usage**: `bash scripts/debug/debug_rust_compromised.sh`

### debug_rust_high_formula.sh
**Purpose**: Debug Rust's high_risk_count() formula  
**When to use**: Count doesn't match expected  
**What it does**:
- Shows each component of HIGH count
- Displays workflow, packages, crypto, trufflehog counts
**Usage**: `bash scripts/debug/debug_rust_high_formula.sh`

### find_all_chalk.sh
**Purpose**: Find all chalk package occurrences  
**When to use**: Investigating chalk detection  
**What it does**:
- Searches all package.json and lockfiles
- Shows chalk versions
**Usage**: `bash scripts/debug/find_all_chalk.sh`

### find_chalk_dupe.sh
**Purpose**: Find duplicate chalk detections  
**When to use**: Investigating why chalk counted twice  
**What it does**:
- Finds all chalk@5.6.1 locations
**Usage**: `bash scripts/debug/find_chalk_dupe.sh`

### find_lockfile_compromised.sh
**Purpose**: Find compromised packages in lockfiles  
**When to use**: Investigating lockfile detection  
**What it does**:
- Scans all lockfiles for compromised versions
- Compares with Bash findings
**Usage**: `bash scripts/debug/find_lockfile_compromised.sh`

### test_chalk_lockfile.sh
**Purpose**: Test specific chalk lockfile detection  
**When to use**: Testing lockfile-comprehensive-test case  
**What it does**:
- Shows chalk in package.json vs lockfile
- Verifies version extraction
**Usage**: `bash scripts/debug/test_chalk_lockfile.sh`

---

## 🧹 CLEANUP

**Location**: `scripts/` (root)

### cleanup_all.sh
**Purpose**: Remove obsolete files from workspace  
**When to use**: After completing development phase  
**What it does**:
- Deletes temp files from analyze/
- Removes obsolete docs from dev-rust-scanner-1/
- Archives old documentation
**Usage**: `bash scripts/cleanup_all.sh`

---

## �️ ARCHIVE

**Location**: `scripts/archive/`

### generate_exact_test_expectations.sh
**Purpose**: Generate expected values for documented test-cases (Legacy)  
**When to use**: Creating documentation of expected test outcomes  
**What it does**:
- Runs predefined test-cases
- Captures exact HIGH/MEDIUM/LOW counts
- Outputs markdown format
**Status**: Legacy tool - kept for reference  
**Usage**: `bash scripts/archive/generate_exact_test_expectations.sh`

---

## �📋 QUICK REFERENCE

### Daily Development
```bash
# Quick check both modes
bash scripts/verification/final_both_modes_check.sh

# Quick paranoid check
bash scripts/verification/quick_paranoid_check.sh
```

### Before Release
```bash
# Full verification (normal + paranoid modes, all 26 test cases)
bash scripts/analyze/verify_100_percent.sh
bash scripts/analyze/verify_100_percent_paranoid.sh

# OR: Run parallel scans first, then verify
bash scripts/analyze/parallel_testcase_scan.sh
bash scripts/analyze/verify_100_percent.sh
```

### Investigating Issues
```bash
# Find which case differs
bash scripts/debug/find_high_diff.sh

# Analyze counting method
bash scripts/analysis/exact_bash_high_count.sh
bash scripts/analysis/exact_rust_high_breakdown.sh
```

### Understanding Differences
```bash
# Compare methodologies
bash scripts/analysis/compare_high_counts.sh

# Debug specific detection
bash scripts/debug/debug_rust_compromised.sh
```

---

## 🎯 RECOMMENDED WORKFLOW

1. **During Development**: Use `quick_paranoid_check.sh` for fast feedback
2. **Before Commit**: Run `final_both_modes_check.sh` 
3. **Before Release**: Run `verify_100_percent.sh` and `verify_100_percent_paranoid.sh`
4. **When Debugging**: Use debug/ and analysis/ scripts as needed

---

**Created**: 2025-10-07  
**Last Updated**: 2025-10-07
