use crate::patterns::RiskLevel;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Complete scan results
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResults {
    pub scan_path: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<f64>,
    pub timestamp: DateTime<Utc>, // Kept for backwards compatibility
    pub files_scanned: usize,
    pub results: Vec<FileResult>,
    pub summary: ScanSummary,
}

/// Results for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub file: String,
    pub risk_level: RiskLevel,
    pub comment: String,
    pub patterns_detected: Vec<String>,
    pub details: Option<Vec<String>>,
}

/// Summary of scan results
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanSummary {
    pub high_risk_count: usize,
    pub medium_risk_count: usize,
    pub low_risk_count: usize,
    pub total_issues: usize,
}

/// Test results structure similar to test_verification_detailed.json
#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub path: String,
    pub expected_risks: Vec<String>,
    pub description: String,
    pub actual_results: Vec<FileResult>,
    pub status: TestStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestStatus {
    Pass,
    Fail,
    Warning,
}

impl ScanResults {
    /// Create new scan results
    pub fn new(scan_path: &Path) -> Self {
        let now = Utc::now();
        ScanResults {
            scan_path: scan_path.to_string_lossy().to_string(),
            start_time: now,
            end_time: None,
            duration_seconds: None,
            timestamp: now, // For backwards compatibility
            files_scanned: 0,
            results: Vec::new(),
            summary: ScanSummary {
                high_risk_count: 0,
                medium_risk_count: 0,
                low_risk_count: 0,
                total_issues: 0,
            },
        }
    }

    /// Add a file result to the scan results with consolidation
    /// Add a file result to the scan results with pattern-level counting (Bash-compatible)
    pub fn add_file_result(&mut self, result: FileResult) {
        // Pattern-level counting: Each finding is a separate issue (like Bash scanner)
        self.update_summary_counts(&result.risk_level);
        self.results.push(result);
    }

    /// Update summary counts for a given risk level
    fn update_summary_counts(&mut self, risk_level: &RiskLevel) {
        match risk_level {
            RiskLevel::High => {
                self.summary.high_risk_count += 1;
                self.summary.total_issues += 1;
            }
            RiskLevel::Medium => {
                self.summary.medium_risk_count += 1;
                self.summary.total_issues += 1;
            }
            RiskLevel::Low => {
                self.summary.low_risk_count += 1;
                self.summary.total_issues += 1;
            }
            RiskLevel::Ok => {} // OK level doesn't count as an issue
        }
    }

    /// Get count of high risk issues
    pub fn high_risk_count(&self) -> usize {
        self.summary.high_risk_count
    }

    /// Get count of medium risk issues
    pub fn medium_risk_count(&self) -> usize {
        self.summary.medium_risk_count
    }

    /// Get count of low risk issues
    pub fn low_risk_count(&self) -> usize {
        self.summary.low_risk_count
    }

    /// Finalize scan results with end time
    pub fn finalize(&mut self) {
        self.end_time = Some(Utc::now());
        if let Some(end_time) = self.end_time {
            self.duration_seconds =
                Some((end_time - self.start_time).num_milliseconds() as f64 / 1000.0);
        }
    }

    /// Save results to JSON file
    pub fn save_json(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Print summary to console and optionally to a file
    pub fn print_summary_to_file(&self, log_file: Option<&Path>) {
        let output = self.format_summary();
        print!("{}", output);
        if let Some(file) = log_file {
            if let Some(parent) = file.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!(
                        "Warning: Failed to create log directory {}: {}",
                        parent.display(),
                        e
                    );
                    return;
                }
            }
            if let Ok(_) = std::fs::write(file, &output) {
                println!("📄 Console log saved to: {}", file.display());
            } else {
                eprintln!("Warning: Failed to write log file: {}", file.display());
            }
        }
    }

    /// Format summary as string
    fn format_summary(&self) -> String {
        let mut output = String::new();

        output.push_str("==============================================\n");
        output.push_str("      SHAI-HULUD DETECTION REPORT\n");
        output.push_str("==============================================\n");
        output.push_str("\n");

        if self.summary.total_issues == 0 {
            output.push_str("✅ No indicators of Shai-Hulud compromise detected.\n");
            output.push_str("Your system appears clean from this specific attack.\n");
            output.push_str("\n");

            // Show summary even for clean scans
            output.push_str("==============================================\n");
            output.push_str("🔍 SUMMARY:\n");
            if let Some(duration) = self.duration_seconds {
                output.push_str(&format!("   Scan Duration: {:.2} seconds\n", duration));
            }
            output.push_str("   High Risk Issues: 0\n");
            output.push_str("   Medium Risk Issues: 0\n");
            output.push_str("   Low Risk (informational): 0\n");
            output.push_str("   Total Critical Issues: 0\n");
            output.push_str("==============================================\n");
            return output;
        }

        // Group results by risk level
        let high_risk: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.risk_level == RiskLevel::High)
            .collect();
        let medium_risk: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.risk_level == RiskLevel::Medium)
            .collect();
        let low_risk: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.risk_level == RiskLevel::Low)
            .collect();

        // Show detailed findings with context (like Bash scanner)
        if !high_risk.is_empty() {
            output.push_str(&format!(
                "🚨 HIGH RISK: {} issues detected\n",
                high_risk.len()
            ));
            for result in high_risk {
                output.push_str(&format!("   • {}\n", result.file));
                let comment_lines: Vec<&str> = result.comment.split('\n').collect();
                output.push_str(&format!("     └─ {}\n", comment_lines[0]));
                for line in &comment_lines[1..] {
                    output.push_str(&format!("         {}\n", line));
                }
            }
            output.push_str("\n");
        }

        if !medium_risk.is_empty() {
            output.push_str(&format!(
                "⚠️  MEDIUM RISK: {} issues detected\n",
                medium_risk.len()
            ));
            for result in medium_risk {
                output.push_str(&format!("   • {}\n", result.file));
                let comment_lines: Vec<&str> = result.comment.split('\n').collect();
                output.push_str(&format!("     └─ {}\n", comment_lines[0]));
                for line in &comment_lines[1..] {
                    output.push_str(&format!("         {}\n", line));
                }
            }
            output.push_str("\n");
        }

        if !low_risk.is_empty() {
            output.push_str(&format!(
                "ℹ️  LOW RISK: {} informational warnings\n",
                low_risk.len()
            ));
            for result in low_risk {
                output.push_str(&format!("   • {}\n", result.file));
                let comment_lines: Vec<&str> = result.comment.split('\n').collect();
                output.push_str(&format!("     └─ {}\n", comment_lines[0]));
                for line in &comment_lines[1..] {
                    output.push_str(&format!("         {}\n", line));
                }
            }
            output.push_str("\n");
        }

        output.push_str("\n");
        output.push_str("==============================================\n");
        output.push_str("🔍 SUMMARY:\n");
        if let Some(duration) = self.duration_seconds {
            output.push_str(&format!("   Scan Duration: {:.2} seconds\n", duration));
        }
        output.push_str(&format!(
            "   High Risk Issues: {}\n",
            self.summary.high_risk_count
        ));
        output.push_str(&format!(
            "   Medium Risk Issues: {}\n",
            self.summary.medium_risk_count
        ));
        output.push_str(&format!(
            "   Low Risk (informational): {}\n",
            self.summary.low_risk_count
        ));
        output.push_str(&format!(
            "   Total Critical Issues: {}\n",
            self.summary.total_issues
        ));
        output.push_str("\n");

        if self.summary.high_risk_count > 0 {
            output.push_str("⚠️  IMPORTANT:\n");
            output.push_str("   - High risk issues likely indicate actual compromise\n");
            output.push_str("   - Immediate investigation and remediation required\n");
            output.push_str("   - Consider running additional security scans\n");
            output.push_str("   - Review your npm audit logs and package history\n");
        } else if self.summary.medium_risk_count > 0 {
            output.push_str("⚠️  IMPORTANT:\n");
            output.push_str("   - Medium risk issues require manual investigation\n");
            output.push_str("   - Verify if detected patterns are legitimate\n");
            output.push_str("   - Review your npm audit logs and package history\n");
        }

        output.push_str("==============================================\n");
        output
    }

    /// Set the number of files scanned
    #[allow(dead_code)]
    pub fn set_files_scanned(&mut self, count: usize) {
        self.files_scanned = count;
    }
}
