// New Workflow Pattern Detection - November 2025 Attack Patterns
// Detects formatter_*.yml workflows and actionsSecrets.json files
//
// Corresponds to bash function:
// - check_new_workflow_patterns() - Lines 348-371 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_new_workflow_patterns
// Purpose: Detect November 2025 new workflow file patterns and actionsSecrets.json
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with paths to new attack pattern files
pub fn check_new_workflow_patterns(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for new workflow patterns...",
    );

    let mut findings = Vec::new();

    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Look for formatter_123456789.yml workflow files in .github/workflows/
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("formatter_") && filename.ends_with(".yml") {
                // Check if in .github/workflows/ directory
                if let Some(parent) = path.parent() {
                    if parent.ends_with(".github/workflows") {
                        findings.push(Finding::new(
                            path.to_path_buf(),
                            "Malicious formatter workflow pattern (November 2025 attack)".to_string(),
                            RiskLevel::High,
                            "new_workflow_files",
                        ));
                    }
                }
            }

            // Look for actionsSecrets.json files (double Base64 encoded secrets)
            if filename == "actionsSecrets.json" {
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "Suspicious GitHub Actions secrets file (credential exfiltration)".to_string(),
                    RiskLevel::High,
                    "actions_secrets_files",
                ));
            }
        }
    }

    findings
}
