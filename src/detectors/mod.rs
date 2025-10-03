// Detectors Module - Detection functions for various attack indicators
// Each detector corresponds to a bash function in shai-hulud-detector.sh

pub mod content;
pub mod crypto;
pub mod git;
pub mod hashes;
pub mod integrity;
pub mod network;
pub mod packages;
pub mod postinstall;
pub mod repos;
pub mod trufflehog;
pub mod typosquatting;
pub mod workflow;

use std::path::PathBuf;

/// Finding severity levels
/// Corresponds to bash risk level prefixes (HIGH RISK, MEDIUM RISK, LOW RISK)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

/// A detection finding with file path, message, and risk level
/// Corresponds to bash array entries like "file:message"
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub file_path: PathBuf,
    pub message: String,
    pub risk_level: RiskLevel,
    pub category: String,
}

impl Finding {
    pub fn new(file_path: PathBuf, message: String, risk_level: RiskLevel, category: &str) -> Self {
        Self {
            file_path,
            message,
            risk_level,
            category: category.to_string(),
        }
    }
}

/// Collection of all findings from a scan
/// Corresponds to bash global arrays: WORKFLOW_FILES, MALICIOUS_HASHES, etc.
#[derive(Debug, Default)]
pub struct ScanResults {
    pub workflow_files: Vec<Finding>,
    pub malicious_hashes: Vec<Finding>,
    pub compromised_found: Vec<Finding>,
    pub suspicious_found: Vec<Finding>,
    pub suspicious_content: Vec<Finding>,
    pub crypto_patterns: Vec<Finding>,
    pub git_branches: Vec<Finding>,
    pub postinstall_hooks: Vec<Finding>,
    pub trufflehog_activity: Vec<Finding>,
    pub shai_hulud_repos: Vec<Finding>,
    pub namespace_warnings: Vec<Finding>,
    pub integrity_issues: Vec<Finding>,
    pub typosquatting_warnings: Vec<Finding>,
    pub network_exfiltration_warnings: Vec<Finding>,
}

impl ScanResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn high_risk_count(&self) -> usize {
        let arrays = vec![
            &self.workflow_files,
            &self.malicious_hashes,
            &self.compromised_found,
            &self.postinstall_hooks,
            &self.shai_hulud_repos,
        ];

        arrays.iter().map(|arr| arr.len()).sum::<usize>()
            + self
                .crypto_patterns
                .iter()
                .filter(|f| f.risk_level == RiskLevel::High)
                .count()
            + self
                .trufflehog_activity
                .iter()
                .filter(|f| f.risk_level == RiskLevel::High)
                .count()
    }

    pub fn medium_risk_count(&self, paranoid_mode: bool) -> usize {
        let arrays = vec![
            &self.suspicious_found,
            &self.suspicious_content,
            &self.git_branches,
            &self.integrity_issues,
        ];

        // BASH EXACT LINE 1523/1545: Only count first 5 typo/network IN paranoid mode
        let typo_count = if paranoid_mode {
            self.typosquatting_warnings.len().min(5)
        } else {
            0 // Not counted in normal mode
        };
        
        let network_count = if paranoid_mode {
            self.network_exfiltration_warnings.len().min(5)
        } else {
            0 // Not counted in normal mode
        };

        arrays.iter().map(|arr| arr.len()).sum::<usize>()
            + typo_count
            + network_count
            + self
                .crypto_patterns
                .iter()
                .filter(|f| f.risk_level == RiskLevel::Medium)
                .count()
            + self
                .trufflehog_activity
                .iter()
                .filter(|f| f.risk_level == RiskLevel::Medium)
                .count()
    }

    pub fn low_risk_count(&self) -> usize {
        self.namespace_warnings.len()
            + self
                .crypto_patterns
                .iter()
                .filter(|f| f.risk_level == RiskLevel::Low)
                .count()
            + self
                .trufflehog_activity
                .iter()
                .filter(|f| f.risk_level == RiskLevel::Low)
                .count()
    }
}
