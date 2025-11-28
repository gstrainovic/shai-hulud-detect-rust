// Second Coming Repository Detection - Git repositories with malicious descriptions
// Detects repos with "Sha1-Hulud: The Second Coming" description pattern
//
// Corresponds to bash function:
// - check_second_coming_repos() - Lines 581-609 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use std::path::Path;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

// Function: check_second_coming_repos
// Purpose: Detect repository descriptions with "Sha1-Hulud: The Second Coming" pattern
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with git repositories matching the description pattern
pub fn check_second_coming_repos(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for 'Second Coming' repository descriptions...",
    );

    let mut findings = Vec::new();

    // Look for git repositories
    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        // Check if this is a .git directory
        if path.file_name().and_then(|n| n.to_str()) != Some(".git") {
            continue;
        }

        // Get parent directory (the actual repo root)
        if let Some(repo_dir) = path.parent() {
            // Check git config for repository description with timeout
            // Use timeout to prevent hanging on problematic repositories
            let description = get_git_description(repo_dir);

            if let Some(desc) = description {
                if desc.contains("Sha1-Hulud: The Second Coming") {
                    findings.push(Finding::new(
                        repo_dir.to_path_buf(),
                        "Malicious repository description: 'Sha1-Hulud: The Second Coming' (November 2025 attack marker)".to_string(),
                        RiskLevel::High,
                        "second_coming_repos",
                    ));
                }
            }
        }
    }

    findings
}

// Helper function to get git repository description with timeout
// Implements 5-second timeout to prevent hanging
fn get_git_description(repo_dir: &Path) -> Option<String> {
    // Use timeout command if available (Linux/macOS), otherwise set process timeout
    #[cfg(unix)]
    {
        // Try using timeout command
        let output = Command::new("timeout")
            .args(["5s", "git", "-C"])
            .arg(repo_dir)
            .args(["config", "--get", "--local", "--null", "--default", ""])
            .arg("repository.description")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let desc = String::from_utf8_lossy(&output.stdout);
                return Some(desc.trim_matches('\0').to_string());
            }
        }
    }

    // Fallback for Windows or when timeout command not available
    // Note: Rust doesn't have built-in process timeout, so we do best effort
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .args(["config", "--get", "--local", "--null", "--default", ""])
        .arg("repository.description")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let desc = String::from_utf8_lossy(&output.stdout);
            return Some(desc.trim_matches('\0').to_string());
        }
    }

    None
}
