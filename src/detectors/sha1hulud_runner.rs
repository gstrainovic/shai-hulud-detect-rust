// SHA1HULUD Runner Detection - GitHub Actions workflows using SHA1HULUD runners
// Detects workflows referencing malicious SHA1HULUD runners
//
// Corresponds to bash function:
// - check_github_actions_runner() - Lines 565-579 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_github_actions_runner
// Purpose: Detect SHA1HULUD GitHub Actions runners in workflow files
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with workflow files containing SHA1HULUD runner references
pub fn check_github_actions_runner(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for SHA1HULUD GitHub Actions runners...",
    );

    let mut findings = Vec::new();

    // Look for workflow files containing SHA1HULUD runner names
    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Check for YAML workflow files
        let filename = path.file_name().and_then(|n| n.to_str());
        if !filename.is_some_and(|f| {
            std::path::Path::new(f).extension().is_some_and(|ext| {
                ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml")
            })
        }) {
            continue;
        }

        // Read file content and check for SHA1HULUD runner references
        if let Ok(content) = fs::read_to_string(path) {
            if content.to_lowercase().contains("sha1hulud") {
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "GitHub Actions workflow contains SHA1HULUD runner references".to_string(),
                    RiskLevel::High,
                    "github_sha1hulud_runners",
                ));
            }
        }
    }

    findings
}
