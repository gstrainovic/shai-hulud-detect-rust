// GitHub Runner Detection - Self-hosted GitHub Actions runners installed by malware
// Detects runner installations used as persistent backdoors
//
// Corresponds to bash function:
// - check_github_runners() - Lines 403-457 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Function: check_github_runners
// Purpose: Detect self-hosted GitHub Actions runners installed by malware
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with paths to suspicious runner installations
pub fn check_github_runners(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for malicious GitHub Actions runners...",
    );

    let mut findings = Vec::new();

    // Runner patterns to search for
    let runner_patterns = [".dev-env", "actions-runner", ".runner", "_work"];

    for pattern in &runner_patterns {
        for entry in WalkDir::new(scan_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            // Check if directory name matches pattern
            if let Some(dirname) = path.file_name().and_then(|n| n.to_str()) {
                if dirname != *pattern {
                    continue;
                }

                // Check for runner configuration files
                let has_runner_config = path.join(".runner").exists()
                    || path.join(".credentials").exists()
                    || path.join("config.sh").exists();

                if has_runner_config {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Runner configuration files found (potential persistent backdoor)"
                            .to_string(),
                        RiskLevel::High,
                        "github_runners",
                    ));
                    // NOTE: No continue - bash checks all patterns for each directory
                }

                // Check for runner binaries
                let has_runner_binary = path.join("Runner.Worker").exists()
                    || path.join("run.sh").exists()
                    || path.join("run.cmd").exists();

                if has_runner_binary {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Runner executable files found (potential persistent backdoor)".to_string(),
                        RiskLevel::High,
                        "github_runners",
                    ));
                    // NOTE: No continue - bash checks all patterns for each directory
                }

                // Check for .dev-env specifically (from Koi.ai report)
                if dirname == ".dev-env" {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "Suspicious .dev-env directory (matches Koi.ai report IOC)".to_string(),
                        RiskLevel::High,
                        "github_runners",
                    ));
                }
            }
        }
    }

    // Also check user home directory specifically for ~/.dev-env
    if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        let home_dev_env = PathBuf::from(home).join(".dev-env");
        if home_dev_env.exists() && home_dev_env.is_dir() {
            findings.push(Finding::new(
                home_dev_env,
                "Malicious runner directory in home folder (Koi.ai IOC)".to_string(),
                RiskLevel::High,
                "github_runners",
            ));
        }
    }

    findings
}
