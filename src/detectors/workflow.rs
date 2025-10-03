// Workflow Files Detector
// Rust port of: check_workflow_files()

use crate::detectors::{Finding, RiskLevel};
use std::path::Path;
use walkdir::WalkDir;

// Function: check_workflow_files
// Purpose: Detect malicious shai-hulud-workflow.yml files in project directories
// Args: $1 = scan_dir (directory to scan)
// Modifies: WORKFLOW_FILES (global array)
// Returns: Populates WORKFLOW_FILES array with paths to suspicious workflow files
pub fn check_workflow_files<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking for malicious workflow files...",
    );

    let mut findings = Vec::new();

    // Look specifically for shai-hulud-workflow.yml files
    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if entry.file_name() == "shai-hulud-workflow.yml" {
            findings.push(Finding::new(
                entry.path().to_path_buf(),
                "Known malicious workflow filename".to_string(),
                RiskLevel::High,
                "workflow",
            ));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_malicious_workflow() {
        let temp = TempDir::new().unwrap();
        let workflows_dir = temp.path().join(".github/workflows");
        fs::create_dir_all(&workflows_dir).unwrap();

        let malicious_file = workflows_dir.join("shai-hulud-workflow.yml");
        fs::write(&malicious_file, "malicious content").unwrap();

        let findings = check_workflow_files(temp.path());

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].risk_level, RiskLevel::High);
    }

    #[test]
    fn test_clean_project() {
        let temp = TempDir::new().unwrap();
        let workflows_dir = temp.path().join(".github/workflows");
        fs::create_dir_all(&workflows_dir).unwrap();

        let clean_file = workflows_dir.join("ci.yml");
        fs::write(&clean_file, "clean content").unwrap();

        let findings = check_workflow_files(temp.path());

        assert_eq!(findings.len(), 0);
    }
}
