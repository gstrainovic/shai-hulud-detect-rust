// Postinstall Hooks Detector
// Rust port of: check_postinstall_hooks()

use crate::detectors::{verification, Finding, RiskLevel};
use serde_json::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_postinstall_hooks
// Purpose: Detect suspicious postinstall scripts that may execute malicious code
// Args: $1 = scan_dir (directory to scan)
// Modifies: POSTINSTALL_HOOKS (global array)
// Returns: Populates POSTINSTALL_HOOKS array with package.json files containing hooks
pub fn check_postinstall_hooks<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "Checking for suspicious postinstall hooks...",
    );

    let mut findings = Vec::new();

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "package.json")
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                // Look for postinstall scripts
                if let Some(scripts) = json.get("scripts").and_then(|v| v.as_object()) {
                    if let Some(postinstall) = scripts.get("postinstall").and_then(|v| v.as_str()) {
                        // Check for suspicious patterns in postinstall commands
                        let suspicious_patterns = ["curl", "wget", "node -e", "eval"];

                        if suspicious_patterns.iter().any(|p| postinstall.contains(p)) {
                            let mut finding = Finding::new(
                                entry.path().to_path_buf(),
                                format!("Suspicious postinstall: {}", postinstall),
                                RiskLevel::High,
                                "postinstall_hook",
                            );

                            // Try to verify via file hash (AI-reviewed files)
                            let hash_verification = verification::verify_file_by_hash(entry.path());
                            if let verification::VerificationStatus::Verified { .. } =
                                hash_verification
                            {
                                finding.verification = Some(hash_verification);
                            }

                            findings.push(finding);
                        }
                    }
                }
            }
        }
    }

    findings
}
