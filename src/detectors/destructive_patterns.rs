// Destructive Pattern Detection - Data destruction patterns that activate on failure
// Detects file deletion patterns used when credential theft fails
//
// Corresponds to bash function:
// - check_destructive_patterns() - Lines 760-830 in shai-hulud-detector.sh
//
// Updated for v3.0.2-3.0.4:
// - Replaced overly broad conditional patterns with tight Shai-Hulud 2.0 wiper signatures
// - Removed standalone glob patterns ($HOME/*, ~/*) that matched path examples in comments
// - Single-pass search to avoid catastrophic backtracking on minified files

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

    // v3.0.2: Basic destructive patterns - command-context patterns only
    // Removed context-free glob patterns ($HOME/*, ~/*) that caused false positives
    let basic_destructive_patterns = vec![
        // Pattern, Description
        (
            r"rm -rf\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/)",
            "rm -rf targeting user directory",
        ),
        (
            r"del /s /q\s+(%USERPROFILE%|\$HOME)",
            "del /s /q targeting user directory",
        ),
        (
            r"Remove-Item -Recurse\s+(\$HOME|~[^a-zA-Z0-9_/])",
            "Remove-Item -Recurse targeting user directory",
        ),
        (
            r"find\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/).*-exec rm",
            "find -exec rm targeting user directory",
        ),
        (
            r"find\s+(\$HOME|~[^a-zA-Z0-9_/]|/home/).*-delete",
            "find -delete targeting user directory",
        ),
        // NOTE: Removed $HOME/*, ~/* standalone patterns (v3.0.2) - caused false positives in comments
    ];

    // v3.0.2: Shai-Hulud 2.0 wiper signatures (replaces overly broad conditional patterns)
    // Based on actual Koi Security malware disclosure
    let shai_hulud_wiper_patterns = [
        // Bun.spawnSync with cmd.exe/bash and destructive commands
        (
            r"Bun\.spawnSync.{1,50}(cmd\.exe|bash).{1,100}(del /F|shred|cipher /W)",
            "Bun.spawnSync wiper pattern",
        ),
        // shred with secure delete flags targeting $HOME
        (
            r"shred.{1,30}-[nuvz].{1,50}(\$HOME|~/)",
            "shred targeting home directory",
        ),
        // cipher /W secure wipe on Windows
        (
            r"cipher\s*/W:.{0,30}USERPROFILE",
            "cipher /W Windows secure wipe",
        ),
        // del /F /Q /S with USERPROFILE
        (
            r"del\s*/F\s*/Q\s*/S.{1,30}USERPROFILE",
            "del /F /Q /S Windows wiper",
        ),
        // find $HOME ... shred pipeline
        (
            r"find.{1,30}\$HOME.{1,50}shred",
            "find + shred wiper pattern",
        ),
        // rd /S /Q recursive delete
        (r"rd\s*/S\s*/Q.{1,30}USERPROFILE", "rd /S /Q Windows wiper"),
    ];

    // Shell-specific patterns (broader for actual shell scripts)
    let conditional_patterns_shell = [
        (
            r"if.*credential.*(fail|error).*rm",
            "credential failure triggers rm",
        ),
        (
            r"if.*token.*not.*found.*(delete|rm)",
            "token not found triggers deletion",
        ),
        (
            r"if.*github.*auth.*fail.*rm",
            "github auth failure triggers rm",
        ),
        (r"catch.*rm -rf", "catch block with rm -rf"),
        (r"error.*delete.*home", "error handler with home deletion"),
    ];

    // Compile regex patterns
    let basic_regexes: Vec<(Regex, &str)> = basic_destructive_patterns
        .iter()
        .filter_map(|(p, desc)| Regex::new(p).ok().map(|r| (r, *desc)))
        .collect();

    let wiper_regexes: Vec<(Regex, &str)> = shai_hulud_wiper_patterns
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
                    // v3.0.2: JavaScript/Python - Use Shai-Hulud 2.0 wiper signatures
                    for (regex, _desc) in &wiper_regexes {
                        if regex.is_match(&content) {
                            found_conditional = true;
                            break;
                        }
                    }
                }
                if found_conditional {
                    // BASH COMPATIBILITY: Include context suffix for JS/Python files
                    let message = if path_str.ends_with(".js") || path_str.ends_with(".py") {
                        "Shai-Hulud wiper pattern detected (JS/Python context)".to_string()
                    } else {
                        "Shai-Hulud wiper pattern detected".to_string()
                    };
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        message,
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
