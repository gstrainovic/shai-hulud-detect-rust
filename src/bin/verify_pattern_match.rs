// Pattern-level verification tool
// Compares Bash and Rust scanner JSON outputs to verify exact finding matches

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
enum RiskLevel {
    High,
    Medium,
    Low,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::High => write!(f, "HIGH"),
            RiskLevel::Medium => write!(f, "MEDIUM"),
            RiskLevel::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Finding {
    file_path: PathBuf,
    message: String,
    risk_level: RiskLevel,
    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanResults {
    workflow_files: Vec<Finding>,
    malicious_hashes: Vec<Finding>,
    compromised_found: Vec<Finding>,
    suspicious_found: Vec<Finding>,
    lockfile_safe_versions: Vec<Finding>,
    suspicious_content: Vec<Finding>,
    crypto_patterns: Vec<Finding>,
    git_branches: Vec<Finding>,
    postinstall_hooks: Vec<Finding>,
    trufflehog_activity: Vec<Finding>,
    shai_hulud_repos: Vec<Finding>,
    namespace_warnings: Vec<Finding>,
    integrity_issues: Vec<Finding>,
    typosquatting_warnings: Vec<Finding>,
    network_exfiltration_warnings: Vec<Finding>,
}

impl ScanResults {
    fn all_findings(&self) -> Vec<&Finding> {
        let mut findings = Vec::new();
        findings.extend(&self.workflow_files);
        findings.extend(&self.malicious_hashes);
        findings.extend(&self.compromised_found);
        findings.extend(&self.suspicious_found);
        findings.extend(&self.lockfile_safe_versions);
        findings.extend(&self.suspicious_content);
        findings.extend(&self.crypto_patterns);
        findings.extend(&self.git_branches);
        findings.extend(&self.postinstall_hooks);
        findings.extend(&self.trufflehog_activity);
        findings.extend(&self.shai_hulud_repos);
        findings.extend(&self.namespace_warnings);
        findings.extend(&self.integrity_issues);
        findings.extend(&self.typosquatting_warnings);
        findings.extend(&self.network_exfiltration_warnings);
        findings
    }

    fn count_by_risk(&self) -> HashMap<RiskLevel, usize> {
        let mut counts = HashMap::new();
        for finding in self.all_findings() {
            *counts.entry(finding.risk_level.clone()).or_insert(0) += 1;
        }
        counts
    }
}

// Normalize file path for comparison (handles Windows vs Unix paths)
fn normalize_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace("\\\\?\\", "")
        .replace("\\", "/")
        .to_lowercase()
}

// Create a fingerprint for a finding (for comparison)
fn finding_fingerprint(finding: &Finding) -> String {
    format!(
        "{}|{}|{}|{}",
        normalize_path(&finding.file_path),
        finding.message.to_lowercase().trim(),
        format!("{:?}", finding.risk_level),
        finding.category.to_lowercase()
    )
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <bash_results.json> <rust_results.json>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} bash_scan.json rust_scan.json", args[0]);
        std::process::exit(1);
    }

    let bash_json_path = &args[1];
    let rust_json_path = &args[2];

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 PATTERN-LEVEL VERIFICATION");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📄 Bash results: {}", bash_json_path);
    println!("📄 Rust results: {}", rust_json_path);
    println!();

    // Load JSON files
    let bash_content = fs::read_to_string(bash_json_path)
        .with_context(|| format!("Failed to read {}", bash_json_path))?;
    let rust_content = fs::read_to_string(rust_json_path)
        .with_context(|| format!("Failed to read {}", rust_json_path))?;

    let bash_results: ScanResults = serde_json::from_str(&bash_content)
        .with_context(|| format!("Failed to parse {}", bash_json_path))?;
    let rust_results: ScanResults = serde_json::from_str(&rust_content)
        .with_context(|| format!("Failed to parse {}", rust_json_path))?;

    // Get all findings
    let bash_findings = bash_results.all_findings();
    let rust_findings = rust_results.all_findings();

    println!("📊 Findings Summary:");
    println!("   Bash scanner: {} findings", bash_findings.len());
    println!("   Rust scanner: {} findings", rust_findings.len());
    println!();

    // Create fingerprint sets
    let bash_set: HashSet<String> = bash_findings.iter().map(|f| finding_fingerprint(f)).collect();
    let rust_set: HashSet<String> = rust_findings.iter().map(|f| finding_fingerprint(f)).collect();

    // Find differences
    let missing_in_rust: Vec<_> = bash_set.difference(&rust_set).collect();
    let extra_in_rust: Vec<_> = rust_set.difference(&bash_set).collect();

    let perfect_match = missing_in_rust.is_empty() && extra_in_rust.is_empty();

    if perfect_match {
        // SUCCESS CASE
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("✅ PERFECT MATCH!");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
        println!("   ✓ All {} findings matched exactly", bash_findings.len());
        println!("   ✓ No missing patterns");
        println!("   ✓ No extra patterns");
        println!();

        // Show breakdown by risk level
        let bash_counts = bash_results.count_by_risk();
        println!("📈 Breakdown by Risk Level:");
        for risk in [RiskLevel::High, RiskLevel::Medium, RiskLevel::Low] {
            let count = bash_counts.get(&risk).unwrap_or(&0);
            if *count > 0 {
                println!("   {}: {} findings ✅", risk, count);
            }
        }

        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        std::process::exit(0);
    } else {
        // MISMATCH CASE
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("❌ MISMATCH DETECTED");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        if !missing_in_rust.is_empty() {
            println!("🔴 Missing in Rust ({} findings):", missing_in_rust.len());
            println!("   (found in Bash but NOT in Rust)");
            println!();

            // Group by risk level
            let mut by_risk: HashMap<RiskLevel, Vec<&Finding>> = HashMap::new();
            for fingerprint in &missing_in_rust {
                if let Some(finding) = bash_findings
                    .iter()
                    .find(|f| &&finding_fingerprint(f) == fingerprint)
                {
                    by_risk
                        .entry(finding.risk_level.clone())
                        .or_insert_with(Vec::new)
                        .push(finding);
                }
            }

            for risk in [RiskLevel::High, RiskLevel::Medium, RiskLevel::Low] {
                if let Some(findings) = by_risk.get(&risk) {
                    println!("   {} RISK ({} findings):", risk, findings.len());
                    for finding in findings {
                        println!(
                            "   📄 {}: {}",
                            normalize_path(&finding.file_path),
                            finding.message
                        );
                        println!("      Category: {}", finding.category);
                        println!();
                    }
                }
            }
        }

        if !extra_in_rust.is_empty() {
            println!("🟡 Extra in Rust ({} findings):", extra_in_rust.len());
            println!("   (found in Rust but NOT in Bash)");
            println!();

            // Group by risk level
            let mut by_risk: HashMap<RiskLevel, Vec<&Finding>> = HashMap::new();
            for fingerprint in &extra_in_rust {
                if let Some(finding) = rust_findings
                    .iter()
                    .find(|f| &&finding_fingerprint(f) == fingerprint)
                {
                    by_risk
                        .entry(finding.risk_level.clone())
                        .or_insert_with(Vec::new)
                        .push(finding);
                }
            }

            for risk in [RiskLevel::High, RiskLevel::Medium, RiskLevel::Low] {
                if let Some(findings) = by_risk.get(&risk) {
                    println!("   {} RISK ({} findings):", risk, findings.len());
                    for finding in findings {
                        println!(
                            "   📄 {}: {}",
                            normalize_path(&finding.file_path),
                            finding.message
                        );
                        println!("      Category: {}", finding.category);
                        println!();
                    }
                }
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("⚠️  Scanners produced different results!");
        println!("    Review findings above for discrepancies.");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        std::process::exit(1);
    }
}
