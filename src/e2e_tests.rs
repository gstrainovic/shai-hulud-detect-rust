use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::output::ScanResults;
use crate::scanner::Scanner;

/// Test case from test_verification_detailed.json
#[derive(Debug, Deserialize, Serialize)]
pub struct TestCase {
    pub name: String,
    pub path: String,
    pub expected_risks: Vec<String>,
    pub description: String,
    pub files: Vec<ExpectedFile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpectedFile {
    pub file: String,
    pub risk_level: String,
    pub comment: String,
    pub patterns_detected: Vec<String>,
    pub purpose: String,
}

/// Root structure of test_verification_detailed.json
#[derive(Debug, Deserialize, Serialize)]
pub struct TestVerification {
    pub test_cases: Vec<TestCase>,
}

/// Comparison result for a single test case
#[derive(Debug)]
pub struct TestComparison {
    pub test_case_name: String,
    pub passed: bool,
    pub issues: Vec<String>,
    pub expected_risk_levels: Vec<String>,
    pub actual_risk_levels: Vec<String>,
    pub missing_patterns: Vec<String>,
    pub unexpected_patterns: Vec<String>,
}

/// End-to-end test runner that compares scanner output with test verification JSON
pub struct E2ETestRunner {
    test_verification: TestVerification,
    base_path: String,
}

impl E2ETestRunner {
    /// Load test verification from JSON file
    pub async fn new(verification_path: &str, base_test_path: &str) -> Result<Self> {
        let content = fs::read_to_string(verification_path).await?;
        let test_verification: TestVerification = serde_json::from_str(&content)?;

        Ok(E2ETestRunner {
            test_verification,
            base_path: base_test_path.to_string(),
        })
    }

    /// Run all test cases and compare with expectations
    pub async fn run_all_tests(&self) -> Result<Vec<TestComparison>> {
        let mut results = Vec::new();

        for test_case in &self.test_verification.test_cases {
            let comparison = self.run_single_test(test_case).await?;
            results.push(comparison);
        }

        Ok(results)
    }

    /// Run a single test case
    async fn run_single_test(&self, test_case: &TestCase) -> Result<TestComparison> {
        let test_path = Path::new(&self.base_path).join(&test_case.path);

        // Skip if test path doesn't exist
        if !test_path.exists() {
            return Ok(TestComparison {
                test_case_name: test_case.name.clone(),
                passed: false,
                issues: vec![format!("Test path does not exist: {}", test_path.display())],
                expected_risk_levels: test_case.expected_risks.clone(),
                actual_risk_levels: vec!["SKIP".to_string()],
                missing_patterns: vec![],
                unexpected_patterns: vec![],
            });
        }

        // Run scanner
        let scanner = Scanner::new(&test_path, false).await?;
        let scan_results = scanner.scan().await?;

        // Compare results
        self.compare_results(test_case, &scan_results)
    }

    /// Compare scanner results with expected results
    fn compare_results(
        &self,
        test_case: &TestCase,
        scan_results: &ScanResults,
    ) -> Result<TestComparison> {
        let mut issues = Vec::new();
        let mut missing_patterns = Vec::new();
        let mut unexpected_patterns = Vec::new();

        // Map scan results by filename for easier comparison
        let actual_results: HashMap<String, _> = scan_results
            .results
            .iter()
            .map(|r| {
                // Extract just the filename from the full path
                let filename = Path::new(&r.file)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&r.file);
                (filename.to_string(), r)
            })
            .collect();

        // Get actual risk levels
        let actual_risk_levels = self.get_actual_risk_levels(scan_results);

        // Check overall risk level expectations
        if !self.risk_levels_match(&test_case.expected_risks, &actual_risk_levels) {
            issues.push(format!(
                "Risk levels don't match. Expected: {:?}, Actual: {:?}",
                test_case.expected_risks, actual_risk_levels
            ));
        }

        // Check individual file expectations
        for expected_file in &test_case.files {
            let expected_filename = Path::new(&expected_file.file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&expected_file.file);

            if let Some(actual_result) = actual_results.get(expected_filename) {
                // Check risk level match
                let expected_risk = self.normalize_risk_level(&expected_file.risk_level);
                let actual_risk =
                    self.normalize_risk_level(&format!("{:?}", actual_result.risk_level));

                if expected_risk != "OK" && actual_risk != expected_risk {
                    issues.push(format!(
                        "File {}: Expected risk {}, got {}",
                        expected_filename, expected_risk, actual_risk
                    ));
                }

                // Check patterns (lenient matching - we don't expect exact pattern name matches)
                if !expected_file.patterns_detected.is_empty()
                    && actual_result.patterns_detected.is_empty()
                {
                    missing_patterns.push(format!(
                        "File {}: Expected patterns but found none",
                        expected_filename
                    ));
                }
            } else if expected_file.risk_level != "OK" {
                // Only report missing files if they were expected to have issues
                issues.push(format!(
                    "Expected file not found in results: {}",
                    expected_filename
                ));
            }
        }

        let passed = issues.is_empty();

        Ok(TestComparison {
            test_case_name: test_case.name.clone(),
            passed,
            issues,
            expected_risk_levels: test_case.expected_risks.clone(),
            actual_risk_levels,
            missing_patterns,
            unexpected_patterns,
        })
    }

    /// Extract risk levels from scan results
    fn get_actual_risk_levels(&self, scan_results: &ScanResults) -> Vec<String> {
        let mut risk_levels = Vec::new();

        if scan_results.high_risk_count() > 0 {
            risk_levels.push("HIGH".to_string());
        }
        if scan_results.medium_risk_count() > 0 {
            risk_levels.push("MEDIUM".to_string());
        }
        if scan_results.low_risk_count() > 0 {
            risk_levels.push("LOW".to_string());
        }

        if risk_levels.is_empty() {
            risk_levels.push("OK".to_string());
        }

        risk_levels
    }

    /// Check if risk levels match (order doesn't matter)
    fn risk_levels_match(&self, expected: &[String], actual: &[String]) -> bool {
        if expected.is_empty() && (actual.is_empty() || actual == ["OK"]) {
            return true;
        }

        // Convert to sets for comparison
        let expected_set: std::collections::HashSet<_> = expected.iter().collect();
        let actual_set: std::collections::HashSet<_> = actual.iter().collect();

        expected_set == actual_set
    }

    /// Normalize risk level names for comparison
    fn normalize_risk_level(&self, risk_level: &str) -> String {
        match risk_level.to_uppercase().as_str() {
            "HIGH" => "HIGH".to_string(),
            "MEDIUM" => "MEDIUM".to_string(),
            "LOW" => "LOW".to_string(),
            "OK" => "OK".to_string(),
            _ => risk_level.to_uppercase(),
        }
    }

    /// Print test results summary
    pub fn print_test_summary(&self, results: &[TestComparison]) {
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();

        println!("==============================================");
        println!("           E2E TEST RESULTS");
        println!("==============================================");
        println!();

        if passed == total {
            println!("✅ ALL TESTS PASSED ({}/{})", passed, total);
        } else {
            println!("❌ SOME TESTS FAILED ({}/{} passed)", passed, total);
        }

        println!();

        for result in results {
            if result.passed {
                println!("✅ {}: PASSED", result.test_case_name);
            } else {
                println!("❌ {}: FAILED", result.test_case_name);
                for issue in &result.issues {
                    println!("   • {}", issue);
                }

                if !result.missing_patterns.is_empty() {
                    println!("   Missing patterns:");
                    for pattern in &result.missing_patterns {
                        println!("     - {}", pattern);
                    }
                }
            }
        }
    }
}
