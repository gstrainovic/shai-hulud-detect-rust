// Destructive Pattern Detection - Data destruction patterns that activate on failure
// Detects file deletion patterns used when credential theft fails
//
// Corresponds to bash function:
// - check_destructive_patterns() - Lines 459-547 in shai-hulud-detector.sh

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
        "🔍 Checking for destructive payload patterns...",
    );

    let mut findings = Vec::new();

    // Destructive patterns targeting user files (from Koi.ai report)
    // These are specific enough to avoid false positives
    let destructive_patterns = vec![
        // File deletion patterns
        r"rm -rf \$HOME",
        r"rm -rf ~",
        r"del /s /q",
        r"Remove-Item -Recurse",
        r"fs\.unlinkSync",
        r"fs\.rmSync.*recursive",
        r"rimraf",
        // Bulk file operations in home directory
        // NOTE: Bash grep -qi doesn't work with [[:space:]] POSIX classes (bash bug)
        // So we skip this pattern to maintain 100% bash compatibility:
        // r"find[[:space:]]+[^[:space:]]+.*[[:space:]]+-delete",
        r"find \$HOME.*-exec rm",
        r"find ~.*-exec rm",
        r"\$HOME/\*",
        r"~/\*",
    ];

    // Conditional destruction patterns - need context limits for JS/Python
    let conditional_patterns_js_py = [r"if.{1,200}credential.{1,50}(fail|error).{1,50}(rm -|fs\.|rimraf|exec|spawn|child_process)",
        r"if.{1,200}token.{1,50}not.{1,20}found.{1,50}(rm -|del |fs\.|rimraf|unlinkSync|rmSync)",
        r"if.{1,200}github.{1,50}auth.{1,50}fail.{1,50}(rm -|fs\.|rimraf|exec)",
        r"catch.{1,100}(rm -rf|fs\.rm|rimraf|exec.*rm)",
        r"error.{1,100}(rm -|del |fs\.|rimraf).{1,100}(\$HOME|~/|home.*(directory|folder|path))"];

    // Shell-specific patterns (broader for actual shell commands)
    let conditional_patterns_shell = [r"if.*credential.*(fail|error).*rm",
        r"if.*token.*not.*found.*(delete|rm)",
        r"if.*github.*auth.*fail.*rm",
        r"catch.*rm -rf",
        r"error.*delete.*home"];

    // Compile regex patterns
    let destructive_regexes: Vec<Regex> = destructive_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let conditional_js_py_regexes: Vec<Regex> = conditional_patterns_js_py
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let conditional_shell_regexes: Vec<Regex> = conditional_patterns_shell
        .iter()
        .filter_map(|p| Regex::new(p).ok())
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
                // Always check specific destructive patterns (low false positive risk)
                for (i, regex) in destructive_regexes.iter().enumerate() {
                    if regex.is_match(&content) {
                        findings.push(Finding::new(
                            path.to_path_buf(),
                            format!("Destructive pattern detected: {}", destructive_patterns[i]),
                            RiskLevel::High,
                            "destructive_patterns",
                        ));
                    }
                }

                // Check conditional patterns based on file type
                if path_str.ends_with(".sh")
                    || path_str.ends_with(".bat")
                    || path_str.ends_with(".ps1")
                    || path_str.ends_with(".cmd")
                {
                    // Shell scripts: Use broader patterns
                    for (i, regex) in conditional_shell_regexes.iter().enumerate() {
                        if regex.is_match(&content) {
                            findings.push(Finding::new(
                                path.to_path_buf(),
                                format!(
                                    "Conditional destruction pattern detected: {}",
                                    conditional_patterns_shell[i]
                                ),
                                RiskLevel::High,
                                "destructive_patterns",
                            ));
                        }
                    }
                } else if path_str.ends_with(".js") || path_str.ends_with(".py") {
                    // JavaScript/Python: Use limited span patterns only
                    for (i, regex) in conditional_js_py_regexes.iter().enumerate() {
                        if regex.is_match(&content) {
                            findings.push(Finding::new(
                                path.to_path_buf(),
                                format!(
                                    "Conditional destruction pattern detected: {}",
                                    conditional_patterns_js_py[i]
                                ),
                                RiskLevel::High,
                                "destructive_patterns",
                            ));
                        }
                    }
                }
            }
        }
        file_count = 0; // Reset for next extension
    }

    findings
}
