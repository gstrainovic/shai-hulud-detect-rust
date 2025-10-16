// Detectors Module - Detection functions for various attack indicators
// Each detector corresponds to a bash function in shai-hulud-detector.sh

pub mod content;
pub mod crypto;
pub mod git;
pub mod hashes;
pub mod integrity;
pub mod lockfile_resolver;
pub mod network;
pub mod packages;
pub mod postinstall;
pub mod repos;
pub mod trufflehog;
pub mod typosquatting;
pub mod verification;
pub mod workflow;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Finding severity levels
/// Corresponds to bash risk level prefixes (HIGH RISK, MEDIUM RISK, LOW RISK)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

/// A detection finding with file path, message, and risk level
/// Corresponds to bash array entries like "file:message"
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Finding {
    pub file_path: PathBuf,
    pub message: String,
    pub risk_level: RiskLevel,
    pub category: String,
}

// Custom serialization to normalize Windows UNC paths (\\?\C:\...)
impl Serialize for Finding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        // Normalize file_path: remove \\?\ prefix and convert to forward slashes
        let path_str = self.file_path.to_string_lossy();
        let normalized = path_str
            .strip_prefix(r"\\?\")
            .unwrap_or(&path_str)
            .replace('\\', "/");

        let mut state = serializer.serialize_struct("Finding", 4)?;
        state.serialize_field("file_path", &normalized)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("risk_level", &self.risk_level)?;
        state.serialize_field("category", &self.category)?;
        state.end()
    }
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
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub workflow_files: Vec<Finding>,
    pub malicious_hashes: Vec<Finding>,
    pub compromised_found: Vec<Finding>,
    pub suspicious_found: Vec<Finding>,
    pub lockfile_safe_versions: Vec<Finding>, // NEW: Packages safe due to lockfile
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

    // BASH COMPATIBILITY: Track counts for suppressed low risk findings
    #[serde(skip)] // Don't include in JSON
    pub suppressed_namespace_count: usize,
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
        // NOTE: lockfile_safe_versions are NOT counted in low_risk (they're informational only)
        // Include both actual namespace warnings AND suppressed ones (for bash compatibility)
        self.namespace_warnings.len()
            + self.suppressed_namespace_count
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
