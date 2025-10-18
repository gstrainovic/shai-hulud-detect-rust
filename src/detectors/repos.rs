// Shai-Hulud Repositories Detector
// Rust port of: check_shai_hulud_repos()

use crate::detectors::{Finding, RiskLevel};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_shai_hulud_repos
// Purpose: Detect Shai-Hulud worm repositories and malicious migration patterns
// Args: $1 = scan_dir (directory to scan)
// Modifies: SHAI_HULUD_REPOS (global array)
// Returns: Populates array with repository patterns and migration indicators
pub fn check_shai_hulud_repos<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "Checking for Shai-Hulud repositories and migration patterns...",
    );

    let mut findings = Vec::new();

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_dir() && e.file_name() == ".git")
    {
        let repo_dir = entry.path().parent().unwrap_or(entry.path());
        let repo_name = repo_dir.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Check repo name
        if repo_name.to_lowercase().contains("shai-hulud") {
            findings.push(Finding::new(
                repo_dir.to_path_buf(),
                "Repository name contains 'Shai-Hulud'".to_string(),
                RiskLevel::High,
                "shai_hulud_repo",
            ));
        }

        // Check for migration pattern
        if repo_name.contains("-migration") {
            findings.push(Finding::new(
                repo_dir.to_path_buf(),
                "Repository name contains migration pattern".to_string(),
                RiskLevel::High,
                "shai_hulud_repo",
            ));
        }

        // Check git config for shai-hulud remotes
        let git_config = entry.path().join("config");
        if let Ok(config_content) = fs::read_to_string(&git_config) {
            if config_content.to_lowercase().contains("shai-hulud") {
                findings.push(Finding::new(
                    repo_dir.to_path_buf(),
                    "Git remote contains 'Shai-Hulud'".to_string(),
                    RiskLevel::High,
                    "shai_hulud_repo",
                ));
            }
        }

        // Check for double base64-encoded data.json
        let data_json = repo_dir.join("data.json");
        if let Ok(content) = fs::read_to_string(&data_json) {
            if content.contains("eyJ") && content.contains("==") {
                findings.push(Finding::new(
                    repo_dir.to_path_buf(),
                    "Contains suspicious data.json (possible base64-encoded credentials)"
                        .to_string(),
                    RiskLevel::High,
                    "shai_hulud_repo",
                ));
            }
        }
    }

    findings
}
