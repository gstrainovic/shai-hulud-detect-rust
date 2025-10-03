// Git Branches Detector
// Rust port of: check_git_branches()

use crate::detectors::{Finding, RiskLevel};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_git_branches
// Purpose: Search for suspicious git branches containing "shai-hulud" in their names
// Args: $1 = scan_dir (directory to scan)
// Modifies: GIT_BRANCHES (global array)
// Returns: Populates GIT_BRANCHES array with branch names and commit hashes
pub fn check_git_branches<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking for suspicious git branches...",
    );

    let mut findings = Vec::new();

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir() && e.file_name() == ".git")
    {
        let repo_dir = entry.path().parent().unwrap_or(entry.path());
        let refs_heads = entry.path().join("refs/heads");

        if refs_heads.exists() {
            for branch_entry in WalkDir::new(&refs_heads)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let branch_name = branch_entry.file_name().to_string_lossy();
                if branch_name.to_lowercase().contains("shai-hulud") {
                    let commit_hash = fs::read_to_string(branch_entry.path())
                        .unwrap_or_default()
                        .trim()
                        .to_string();

                    findings.push(Finding::new(
                        repo_dir.to_path_buf(),
                        format!(
                            "Branch '{}' (commit: {}...)",
                            branch_name,
                            &commit_hash.chars().take(8).collect::<String>()
                        ),
                        RiskLevel::Medium,
                        "git_branch",
                    ));
                }
            }
        }
    }

    findings
}
