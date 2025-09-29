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

        // Categorize results similar to Bash implementation
        self.format_categorized_results(&mut output);

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

    /// Format results categorized similar to Bash implementation
    fn format_categorized_results(&self, output: &mut String) {
        use std::collections::HashMap;
        
        // Define categories similar to Bash implementation
        let mut categories: HashMap<&str, Vec<&FileResult>> = HashMap::new();
        
        for result in &self.results {
            let category = self.categorize_result(result);
            categories.entry(category).or_insert_with(Vec::new).push(result);
        }

        // Format each category in the order they appear in Bash
        let category_order = vec![
            ("malicious_workflow", "🚨 HIGH RISK: Malicious workflow files detected:"),
            ("suspicious_packages", "⚠️  MEDIUM RISK: Suspicious package versions detected:"),
            ("suspicious_content", "⚠️  MEDIUM RISK: Suspicious content patterns:"),
            ("crypto_theft", "🚨 HIGH RISK: Cryptocurrency theft patterns detected:"),
            ("crypto_manipulation", "⚠️  MEDIUM RISK: Potential cryptocurrency manipulation patterns:"),
            ("trufflehog_high", "🚨 HIGH RISK: Trufflehog/secret scanning activity detected:"),
            ("trufflehog_medium", "⚠️  MEDIUM RISK: Potentially suspicious secret scanning patterns:"),
            ("package_integrity", "⚠️  MEDIUM RISK: Package integrity issues detected:"),
            ("postinstall_hooks", "⚠️  MEDIUM RISK: Suspicious postinstall hooks detected:"),
            ("other_high", "🚨 HIGH RISK: Other issues detected:"),
            ("other_medium", "⚠️  MEDIUM RISK: Other issues detected:"),
            ("other_low", "ℹ️  LOW RISK: Other informational warnings:"),
        ];

        for (category_key, category_title) in category_order {
            if let Some(results) = categories.get(category_key) {
                if !results.is_empty() {
                    output.push_str(category_title);
                    output.push_str("\n");
                    
                    for result in results {
                        output.push_str(&format!("   - {}\n", result.file));
                        
                        // Show context with ASCII box for high-risk items
                        if result.risk_level == RiskLevel::High {
                            self.format_high_risk_context(output, result);
                        } else {
                            self.format_standard_context(output, result);
                        }
                    }
                    output.push_str("\n");
                }
            }
        }
    }

    /// Categorize a result based on patterns detected
    fn categorize_result(&self, result: &FileResult) -> &'static str {
        // Check for workflow files
        if result.file.contains(".github/workflows/") && result.risk_level == RiskLevel::High {
            return "malicious_workflow";
        }

        // Check for package-related issues
        if result.patterns_detected.iter().any(|p| 
            p.contains("suspicious_package") || 
            p.contains("typosquatting") ||
            p.contains("debug_package_risk") ||
            p.contains("crypto_libraries")
        ) {
            return "suspicious_packages";
        }

        // Check for cryptocurrency theft patterns
        if result.patterns_detected.iter().any(|p|
            p.contains("xhr_prototype_modification") ||
            p.contains("known_attacker_wallet") ||
            result.comment.contains("XMLHttpRequest prototype modification") ||
            result.comment.contains("Known attacker wallet address")
        ) && result.risk_level == RiskLevel::High {
            return "crypto_theft";
        }

        // Check for crypto manipulation patterns
        if result.patterns_detected.iter().any(|p|
            p.contains("ethereum_addresses") ||
            p.contains("phishing_domain") ||
            result.comment.contains("Ethereum wallet address") ||
            result.comment.contains("JavaScript obfuscation")
        ) && result.risk_level == RiskLevel::Medium {
            return "crypto_manipulation";
        }

        // Check for high-risk Trufflehog activity
        if (result.patterns_detected.iter().any(|p|
            p.contains("trufflehog") ||
            p.contains("credential_scanning")
        ) && result.comment.contains("HIGH RISK")) || 
        result.comment.contains("Trufflehog binary found") {
            return "trufflehog_high";
        }

        // Check for medium-risk Trufflehog patterns
        if result.patterns_detected.iter().any(|p|
            p.contains("trufflehog") ||
            p.contains("credential_scanning") ||
            p.contains("environment_variable")
        ) && result.risk_level == RiskLevel::Medium {
            return "trufflehog_medium";
        }

        // Check for package integrity issues
        if result.patterns_detected.iter().any(|p|
            p.contains("lockfile_integrity") ||
            p.contains("compromised_packages") ||
            result.comment.contains("lockfile contains compromised packages")
        ) {
            return "package_integrity";
        }

        // Check for postinstall hooks
        if result.comment.contains("postinstall hook") {
            return "postinstall_hooks";
        }

        // Check for suspicious content patterns
        if result.patterns_detected.iter().any(|p|
            p.contains("webhook_site") ||
            p.contains("malicious_webhook") ||
            p.contains("network_exfiltration") ||
            p.contains("pastebin") ||
            p.contains("websocket")
        ) {
            return "suspicious_content";
        }

        // Default categorization by risk level
        match result.risk_level {
            RiskLevel::High => "other_high",
            RiskLevel::Medium => "other_medium",
            RiskLevel::Low => "other_low",
            _ => "other_low",
        }
    }

    /// Set the number of files scanned
    #[allow(dead_code)]
    pub fn set_files_scanned(&mut self, count: usize) {
        self.files_scanned = count;
    }

    /// Format high-risk context with ASCII box similar to Bash implementation
    fn format_high_risk_context(&self, output: &mut String, result: &FileResult) {
        output.push_str("   ┌─ File: ");
        output.push_str(&result.file);
        output.push_str("\n");
        output.push_str("   │  Context: HIGH RISK: ");
        let comment_lines: Vec<&str> = result.comment.split('\n').collect();
        output.push_str(comment_lines[0]);
        output.push_str("\n");
        output.push_str("   └─\n");
        
        // Add notes after the box
        for line in &comment_lines[1..] {
            if !line.trim().is_empty() {
                output.push_str(&format!("NOTE: {}\n", line));
            }
        }
    }

    /// Format standard context for medium and low risk items
    fn format_standard_context(&self, output: &mut String, result: &FileResult) {
        let comment_lines: Vec<&str> = result.comment.split('\n').collect();
        output.push_str(&format!("     └─ {}\n", comment_lines[0]));
        
        for line in &comment_lines[1..] {
            if !line.trim().is_empty() {
                output.push_str(&format!("NOTE: {}\n", line));
            }
        }
    }
}
