// Preinstall Bun Pattern Detection - Fake Bun runtime preinstall patterns
// Detects malicious "preinstall": "node setup_bun.js" in package.json
//
// Corresponds to bash function:
// - check_preinstall_bun_patterns() - Lines 549-563 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_preinstall_bun_patterns
// Purpose: Detect fake Bun runtime preinstall patterns in package.json files
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with files containing suspicious preinstall patterns
pub fn check_preinstall_bun_patterns(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for fake Bun preinstall patterns...",
    );

    let mut findings = Vec::new();

    // Look for package.json files with suspicious "preinstall": "node setup_bun.js" pattern
    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.file_name().and_then(|n| n.to_str()) != Some("package.json") {
            continue;
        }

        // Read and check file content
        if let Ok(content) = fs::read_to_string(path) {
            // Check for the malicious preinstall pattern
            // Pattern: "preinstall": "node setup_bun.js" (with flexible whitespace)
            if content.contains(r#""preinstall""#) && content.contains("setup_bun.js") {
                // More precise check with regex-like logic
                if content.contains(r#""preinstall":"node setup_bun.js""#)
                    || content.contains(r#""preinstall": "node setup_bun.js""#)
                    || content.contains(r#""preinstall" : "node setup_bun.js""#)
                {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Malicious preinstall script: fake Bun runtime installation (November 2025 attack)".to_string(),
                        RiskLevel::High,
                        "preinstall_bun_patterns",
                    ));
                }
            }
        }
    }

    findings
}
