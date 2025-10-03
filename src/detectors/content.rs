// Content Detector
// Rust port of: check_content()

use crate::detectors::{Finding, RiskLevel};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_content
// Purpose: Search for suspicious content patterns like webhook.site and malicious endpoints
// Args: $1 = scan_dir (directory to scan)
// Modifies: SUSPICIOUS_CONTENT (global array)
// Returns: Populates SUSPICIOUS_CONTENT array with files containing suspicious patterns
pub fn check_content<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking for suspicious content patterns...",
    );

    let mut findings = Vec::new();
    let extensions = &["js", "ts", "json", "yml", "yaml"];

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        })
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            // Search for webhook.site references
            if content.contains("webhook.site") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "webhook.site reference".to_string(),
                    RiskLevel::Medium,
                    "suspicious_content",
                ));
            }

            // Search for malicious webhook endpoint
            if content.contains("bb8ca5f6-4175-45d2-b042-fc9ebb8170b7") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "malicious webhook endpoint".to_string(),
                    RiskLevel::Medium,
                    "suspicious_content",
                ));
            }
        }
    }

    findings
}
