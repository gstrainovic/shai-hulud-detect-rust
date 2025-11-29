// Destructive Pattern Detection - Data destruction patterns that activate on failure
// Detects file deletion patterns used when credential theft fails
//
// Corresponds to bash function:
// - check_destructive_patterns() - Lines 686-740 in shai-hulud-detector.sh
//
// Updated to match PRs #85 and #86 changes:
// - Standalone rimraf, fs.unlinkSync, fs.rmSync removed to reduce false positives (Issue #74)
// - Only flags deletion commands that target user directories ($HOME, ~, /home/)
// - ~[^a-zA-Z0-9_/] excludes Vue.js import aliases like ~/path
// - exec.{1,30}rm limits span to avoid matching minified code

use super::{Finding, RiskLevel};
use crate::colors;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_destructive_patterns
// Purpose: Detect destructive patterns that can cause data loss when credential theft fails
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with paths to files containing destructive patterns
pub fn check_destructive_patterns(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "   Checking for destructive payload patterns...",
    );

    let mut findings = Vec::new();

    // PR #85/#86: Basic destructive patterns - ONLY flag when targeting user directories ($HOME, ~, /home/)
    // Standalone rimraf/unlinkSync/rmSync removed to reduce false positives (GitHub issue #74)
    let basic_destructive_patterns = vec![
        // Pattern, Description
        (r"rm -rf\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/)", "rm -rf targeting user directory"),
        (r"del /s /q\s+(%USERPROFILE%|\$HOME)", "del /s /q targeting user directory"),
        (r"Remove-Item -Recurse\s+(\$HOME|~[^a-zA-Z0-9_/])", "Remove-Item -Recurse targeting user directory"),
        (r"find\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/).*-exec rm", "find -exec rm targeting user directory"),
        (r"find\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/).*-delete", "find -delete targeting user directory"),
        (r"\$HOME/\*", "$HOME/* wildcard deletion"),
        (r"~/\*", "~/* wildcard deletion (literal asterisk)"),
        (r"/home/[^/]+/\*", "/home/user/* wildcard deletion"),
    ];

    // Conditional destruction patterns - need context limits for JS/Python
    // Note: exec.{1,30}rm limits span to avoid matching minified code where "exec" and "rm" are far apart
    let conditional_patterns_js_py = [
        (r"if.{1,200}credential.{1,50}(fail|error).{1,50}(rm -|fs\.|rimraf|exec|spawn|child_process)", "credential failure triggers deletion"),
        (r"if.{1,200}token.{1,50}not.{1,20}found.{1,50}(rm -|del |fs\.|rimraf|unlinkSync|rmSync)", "token not found triggers deletion"),
        (r"if.{1,200}github.{1,50}auth.{1,50}fail.{1,50}(rm -|fs\.|rimraf|exec)", "github auth failure triggers deletion"),
        (r"catch.{1,100}(rm -rf|fs\.rm|rimraf|exec.{1,30}rm)", "catch block with deletion"),
        (r"error.{1,100}(rm -|del |fs\.|rimraf).{1,100}(\$HOME|~/|home.*(directory|folder|path))", "error handler with home directory deletion"),
    ];

    // Shell-specific patterns (broader for actual shell commands)
    let conditional_patterns_shell = [
        (r"if.*credential.*(fail|error).*rm", "credential failure triggers rm"),
        (r"if.*token.*not.*found.*(delete|rm)", "token not found triggers deletion"),
        (r"if.*github.*auth.*fail.*rm", "github auth failure triggers rm"),
        (r"catch.*rm -rf", "catch block with rm -rf"),
        (r"error.*delete.*home", "error handler with home deletion"),
    ];

    // Compile regex patterns
    let basic_regexes: Vec<(Regex, &str)> = basic_destructive_patterns
        .iter()
        .filter_map(|(p, desc)| Regex::new(p).ok().map(|r| (r, *desc)))
        .collect();

    let conditional_js_py_regexes: Vec<(Regex, &str)> = conditional_patterns_js_py
        .iter()
        .filter_map(|(p, desc)| Regex::new(p).ok().map(|r| (r, *desc)))
        .collect();

    let conditional_shell_regexes: Vec<(Regex, &str)> = conditional_patterns_shell
        .iter()
        .filter_map(|(p, desc)| Regex::new(p).ok().map(|r| (r, *desc)))
        .collect();

    // Search for destructive patterns in common script files
    let file_extensions = ["*.js", "*.sh", "*.ps1", "*.py", "*.bat", "*.cmd"];

    let mut file_count = 0;
    for ext in &file_extensions {
        for entry in WalkDir::new(scan_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // Match file extension
            let path_str = path.to_string_lossy();
            let ext_pattern = ext.trim_start_matches('*');
            if !path_str.ends_with(ext_pattern) {
                continue;
            }

            // Limit to 100 files per extension for performance
            file_count += 1;
            if file_count > 100 {
                break;
            }

            // Read file content
            if let Ok(content) = fs::read_to_string(path) {
                // BASH EXACT: Only report ONCE per file per category (basic or conditional)
                // Bash uses `grep -l` which outputs filename once per match, not per pattern
                
                // Check basic destructive patterns (targeting user directories only)
                let mut found_basic = false;
                for (regex, _desc) in &basic_regexes {
                    if regex.is_match(&content) {
                        found_basic = true;
                        break; // Stop after first match
                    }
                }
                if found_basic {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Basic destructive pattern detected".to_string(),
                        RiskLevel::High,
                        "destructive_patterns",
                    ));
                }

                // Check conditional patterns based on file type
                let mut found_conditional = false;
                if path_str.ends_with(".sh")
                    || path_str.ends_with(".bat")
                    || path_str.ends_with(".ps1")
                    || path_str.ends_with(".cmd")
                {
                    // Shell scripts: Use broader patterns
                    for (regex, _desc) in &conditional_shell_regexes {
                        if regex.is_match(&content) {
                            found_conditional = true;
                            break;
                        }
                    }
                } else if path_str.ends_with(".js") || path_str.ends_with(".py") {
                    // JavaScript/Python: Use limited span patterns only
                    for (regex, _desc) in &conditional_js_py_regexes {
                        if regex.is_match(&content) {
                            found_conditional = true;
                            break;
                        }
                    }
                }
                if found_conditional {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Conditional destruction pattern detected".to_string(),
                        RiskLevel::High,
                        "destructive_patterns",
                    ));
                }
            }
        }
        file_count = 0; // Reset for next extension
    }

    findings
}
