# Fix: Cross-Platform Compatibility (Git Bash + WSL/Linux)

## Branch

`fix/cross-platform-compatibility`

## Problem Description

The recent performance optimizations introduced two cross-platform compatibility issues:

### Issue 1: `comm` command fails on Git Bash/MSYS2

In `check_packages()`, the code uses `comm -12` for fast set intersection after sorting with `LC_ALL=C sort`. However, the `comm` command uses the default locale, which can differ from the sort locale on Git Bash/MSYS2.

**Error message:**
```
comm: file 1 is not in sorted order
```

**Root cause:** On Git Bash (Windows), the default locale may differ from `LC_ALL=C`, causing `comm` to interpret the sort order differently than `sort` did.

### Issue 2: `shasum` command not available on Linux/WSL

The hash computation uses `shasum -a 256`, which is available on macOS and Git Bash but **not** on native Linux/WSL systems (which have `sha256sum` instead).

**Error message:**
```
bash: shasum: command not found
```

**Root cause:** `shasum` is a Perl script that comes with macOS and Git Bash, but Linux distributions ship `sha256sum` from GNU coreutils instead.

## The Fix

### Fix 1: Consistent locale for `comm`

```diff
-    comm -12 "$TEMP_DIR/compromised_lookup.txt" "$TEMP_DIR/deps_only.txt" > "$TEMP_DIR/matched_deps.txt"
+    # FIX: Use LC_ALL=C to ensure comm uses the same sort order as sort (Git Bash compatibility)
+    LC_ALL=C comm -12 "$TEMP_DIR/compromised_lookup.txt" "$TEMP_DIR/deps_only.txt" > "$TEMP_DIR/matched_deps.txt"
```

### Fix 2: Hash command fallback

```diff
     print_status "$BLUE" "   Computing hashes in parallel..."
-    xargs -P "$PARALLELISM" shasum -a 256 < "$TEMP_DIR/priority_files.txt" 2>/dev/null | \
+    # FIX: Use sha256sum on Linux/WSL, shasum on macOS/Git Bash
+    # Check if shasum actually works (not just exists in PATH)
+    local hash_cmd="sha256sum"
+    if shasum -a 256 /dev/null &>/dev/null; then
+        hash_cmd="shasum -a 256"
+    fi
+    xargs -P "$PARALLELISM" $hash_cmd < "$TEMP_DIR/priority_files.txt" 2>/dev/null | \
```

Note: We use `shasum -a 256 /dev/null` as a functional test rather than `command -v shasum` because on WSL, the Windows `shasum` may appear in PATH but fail to execute.

## Testing

### Tested Platforms

| Platform | Before Fix | After Fix |
|----------|------------|-----------|
| Git Bash (Windows) | ❌ `comm: file is not in sorted order` | ✅ Works |
| WSL2 (Fedora 41) | ❌ `shasum: command not found` | ✅ Works |
| macOS | ✅ Works | ✅ Works (no regression) |
| Native Linux | ❌ `shasum: command not found` | ✅ Works |

### Verification

**Sorting consistency:** `LC_ALL=C sort` produces identical byte-order sorting on both Git Bash and WSL/Linux.

**Hash consistency:** Both `shasum -a 256` and `sha256sum` produce identical SHA-256 hashes. The output format differs slightly (`*filename` vs `filename`), but `awk '{print $1, $2}'` correctly parses both formats.

### Test Commands

```bash
# Git Bash
./shai-hulud-detector.sh test-cases/infected-project

# WSL (from Git Bash)
wsl bash -c "cd /mnt/c/path/to/shai-hulud-detect && ./shai-hulud-detector.sh test-cases/infected-project"
```

## Impact

- **No regressions:** Both fixes are backward compatible with macOS
- **Wider platform support:** Scanner now works on Git Bash, WSL, and native Linux
- **No performance impact:** The hash command detection runs once per scan

## Related Issues

This fixes issues introduced by the performance optimizations in recent commits that use `comm` for O(n) set intersection and batch hash computation.
