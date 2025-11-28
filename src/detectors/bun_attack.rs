// Bun Attack Detection - November 2025 "Shai-Hulud: The Second Coming" Attack
// Detects fake Bun runtime installation files and obfuscated credential harvesting payloads
//
// Corresponds to bash functions:
// - check_bun_attack_files() - Lines 271-346 in shai-hulud-detector.sh

use super::{Finding, RiskLevel};
use crate::colors;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Known malicious file hashes from Koi.ai incident report
// https://www.koi.ai/incident/live-updates-sha1-hulud-the-second-coming
const SETUP_BUN_HASHES: &[&str] =
    &["a3894003ad1d293ba96d77881ccd2071446dc3f65f434669b49b3da92421901a"];

const BUN_ENVIRONMENT_HASHES: &[&str] = &[
    "62ee164b9b306250c1172583f138c9614139264f889fa99614903c12755468d0",
    "f099c5d9ec417d4445a0328ac0ada9cde79fc37410914103ae9c609cbc0ee068",
    "cbb9bc5a8496243e02f3cc080efbe3e4a1430ba0671f2e43a202bf45b05479cd",
];

// Function: check_bun_attack_files
// Purpose: Detect November 2025 "Shai-Hulud: The Second Coming" Bun attack files
// Args: scan_dir (directory to scan)
// Returns: Vec<Finding> with paths to suspicious Bun-related malicious files
pub fn check_bun_attack_files(scan_dir: &Path) -> Vec<Finding> {
    colors::print_status(
        colors::Color::Blue,
        "🔍 Checking for November 2025 Bun attack files...",
    );

    let mut findings = Vec::new();

    // Look for setup_bun.js files (fake Bun runtime installation)
    for entry in WalkDir::new(scan_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();

        // Check for setup_bun.js
        if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some("setup_bun.js") {
            // Verify hash if possible
            if let Ok(file_hash) = calculate_sha256(path) {
                if SETUP_BUN_HASHES.contains(&file_hash.as_str()) {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        format!("SHA256={file_hash} (CONFIRMED MALICIOUS - Koi.ai IOC)"),
                        RiskLevel::High,
                        "bun_setup_files",
                    ));
                } else {
                    // Found setup_bun.js but hash doesn't match known malicious
                    // Still suspicious due to naming convention
                    // BASH COMPATIBILITY: Match exact message from shai-hulud-detector.sh
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "setup_bun.js - Fake Bun runtime installation malware".to_string(),
                        RiskLevel::High,
                        "bun_setup_files",
                    ));
                }
            } else {
                // Hash calculation failed, report anyway
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "setup_bun.js - Fake Bun runtime installation malware".to_string(),
                    RiskLevel::High,
                    "bun_setup_files",
                ));
            }
        }

        // Check for bun_environment.js (10MB+ obfuscated payload)
        if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some("bun_environment.js")
        {
            // Verify hash if possible
            if let Ok(file_hash) = calculate_sha256(path) {
                if BUN_ENVIRONMENT_HASHES.contains(&file_hash.as_str()) {
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        format!("SHA256={file_hash} (CONFIRMED MALICIOUS - Koi.ai IOC)"),
                        RiskLevel::High,
                        "bun_environment_files",
                    ));
                } else {
                    // Found bun_environment.js but hash doesn't match known malicious
                    // Still highly suspicious due to naming and typical size
                    // BASH COMPATIBILITY: Match exact message from shai-hulud-detector.sh
                    findings.push(Finding::new(
                        path.to_path_buf(),
                        "bun_environment.js - 10MB+ obfuscated credential harvesting payload"
                            .to_string(),
                        RiskLevel::High,
                        "bun_environment_files",
                    ));
                }
            } else {
                // Hash calculation failed, report anyway
                findings.push(Finding::new(
                    path.to_path_buf(),
                    "bun_environment.js - 10MB+ obfuscated credential harvesting payload"
                        .to_string(),
                    RiskLevel::High,
                    "bun_environment_files",
                ));
            }
        }
    }

    findings
}

// Helper function to calculate SHA256 hash of a file
fn calculate_sha256(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
