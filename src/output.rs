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
    pub timestamp: DateTime<Utc>,
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
        ScanResults {
            scan_path: scan_path.to_string_lossy().to_string(),
            timestamp: Utc::now(),
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

    /// Add a file result to the scan results
    pub fn add_file_result(&mut self, result: FileResult) {
        // Update summary counts
        match result.risk_level {
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

        self.results.push(result);
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

    /// Save results to JSON file
    pub fn save_json(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Print summary to console
    pub fn print_summary(&self) {
        println!();
        println!("==============================================");
        println!("      SHAI-HULUD DETECTION REPORT");
        println!("==============================================");
        println!();

        if self.summary.total_issues == 0 {
            println!("✅ No indicators of Shai-Hulud compromise detected.");
            println!("Your system appears clean from this specific attack.");
            return;
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

        // Print high risk issues
        if !high_risk.is_empty() {
            println!("🚨 HIGH RISK: {} issues detected", high_risk.len());
            for result in high_risk {
                println!("   - {}", result.file);
                println!("     {}", result.comment);
                if let Some(details) = &result.details {
                    for detail in details.iter().take(3) {
                        // Limit output
                        println!("     • {}", detail);
                    }
                }
                println!();
            }
        }

        // Print medium risk issues
        if !medium_risk.is_empty() {
            println!("⚠️  MEDIUM RISK: {} issues detected", medium_risk.len());
            for result in medium_risk {
                println!("   - {}", result.file);
                println!("     {}", result.comment);
                println!();
            }
        }

        // Print low risk issues
        if !low_risk.is_empty() {
            println!("ℹ️  LOW RISK: {} informational warnings", low_risk.len());
            for result in low_risk {
                println!("   - {}", result.file);
                println!("     {}", result.comment);
                println!();
            }
        }

        println!("==============================================");
        println!("🔍 SUMMARY:");
        println!("   High Risk Issues: {}", self.summary.high_risk_count);
        println!("   Medium Risk Issues: {}", self.summary.medium_risk_count);
        println!(
            "   Low Risk (informational): {}",
            self.summary.low_risk_count
        );
        println!("   Total Critical Issues: {}", self.summary.total_issues);
        println!();

        if self.summary.high_risk_count > 0 {
            println!("⚠️  IMPORTANT:");
            println!("   - High risk issues likely indicate actual compromise");
            println!("   - Immediate investigation and remediation required");
            println!("   - Consider running additional security scans");
        } else if self.summary.medium_risk_count > 0 {
            println!("⚠️  IMPORTANT:");
            println!("   - Medium risk issues require manual investigation");
            println!("   - Verify if detected patterns are legitimate");
        }

        println!("==============================================");
    }

    /// Set the number of files scanned
    pub fn set_files_scanned(&mut self, count: usize) {
        self.files_scanned = count;
    }
}
