// Discussion Workflow Detection - Malicious GitHub Actions with discussion triggers
// Detects workflows using discussion events for arbitrary command execution
//
// Corresponds to bash function:
// - check_discussion_workflows() - Lines 373-401 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_discussion_workflows
// Purpose: Detect malicious GitHub Actions workflows with discussion triggers
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with paths to suspicious discussion-triggered workflows
pub fn check_discussion_workflows(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for malicious discussion workflows...",
    );

    let mut findings = Vec::new();

    // Regex patterns for detection
    // NOTE: Bash grep doesn't match across newlines, so we need to be specific
    // Pattern matches: "on: discussion" or "on:.*discussion" (on same line only)
    // Use [ \t]* instead of \s* because \s includes newlines in Rust regex
    let discussion_trigger = Regex::new(r"on:[ \t]*discussion").unwrap();
    let self_hosted_runner = Regex::new(r"runs-on:.*self-hosted").unwrap();
    let dynamic_payload = Regex::new(r"\$\{\{ github\.event\..*\.body \}\}").unwrap();

    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();

        // Look for .yml/.yaml files in .github/workflows/ directories
        if !path.is_file() {
            continue;
        }

        let is_workflow = path
            .parent()
            .is_some_and(|p| p.ends_with(".github/workflows"));

        if !is_workflow {
            continue;
        }

        let filename = path.file_name().and_then(|n| n.to_str());
        if !filename
            .is_some_and(|f| f.ends_with(".yml") || f.ends_with(".yaml"))
        {
            continue;
        }

        // Read file content for pattern matching
        if let Ok(content) = fs::read_to_string(path) {
            // Check for discussion-based triggers
            if discussion_trigger.is_match(&content) {
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "Discussion trigger detected (enables arbitrary command execution)".to_string(),
                    RiskLevel::High,
                    "discussion_workflows",
                ));
            }

            // Check for self-hosted runners combined with dynamic payload execution
            if self_hosted_runner.is_match(&content) && dynamic_payload.is_match(&content) {
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "Self-hosted runner with dynamic payload execution (high risk)".to_string(),
                    RiskLevel::High,
                    "discussion_workflows",
                ));
            }

            // Check for specific discussion.yaml filename (exact match from Koi.ai report)
            if filename == Some("discussion.yaml") || filename == Some("discussion.yml") {
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "Suspicious discussion workflow filename (matches Koi.ai IOC)".to_string(),
                    RiskLevel::High,
                    "discussion_workflows",
                ));
            }
        }
    }

    findings
}
