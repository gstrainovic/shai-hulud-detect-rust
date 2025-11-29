// Trufflehog Activity Detector - BASH EXACT VERSION
// Matches bash check_trufflehog_activity() from shai-hulud-detector.sh lines 1466-1550
//
// IMPORTANT: Bash uses "skip if already flagged" logic - only ONE finding per file!

use crate::detectors::{Finding, RiskLevel};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

lazy_static! {
    // HIGH PRIORITY: Dynamic TruffleHog download patterns
    static ref DOWNLOAD_PATTERN: Regex = Regex::new(r"curl.*trufflehog|wget.*trufflehog|bunExecutable.*trufflehog|download.*trufflehog").unwrap();
    // HIGH PRIORITY: TruffleHog credential harvesting patterns
    static ref CREDENTIAL_SCAN_PATTERN: Regex = Regex::new(r"TruffleHog.*scan.*credential|trufflehog.*env|trufflehog.*AWS|trufflehog.*NPM_TOKEN").unwrap();
    // HIGH PRIORITY: Credential patterns with exfiltration
    static ref EXFIL_PATTERN: Regex = Regex::new(r"(AWS_ACCESS_KEY|GITHUB_TOKEN|NPM_TOKEN).*(webhook\.site|curl|https\.request)").unwrap();
    // MEDIUM PRIORITY: Trufflehog references
    static ref TRUFFLEHOG_REF: Regex = Regex::new(r"(?i)trufflehog").unwrap();
    // MEDIUM PRIORITY: Credential scanning patterns
    static ref CREDENTIAL_PATTERN: Regex = Regex::new(r"AWS_ACCESS_KEY|GITHUB_TOKEN|NPM_TOKEN").unwrap();
    // LOW PRIORITY: Environment variable scanning with suspicious patterns
    static ref ENV_SUSPICIOUS: Regex = Regex::new(r"(process\.env|os\.environ|getenv).*(scan|harvest|steal|exfiltrat)").unwrap();
}

/// Detect Trufflehog secret scanning activity - BASH EXACT version
/// Matches bash check_trufflehog_activity() exactly:
/// - Only ONE finding per file (skip if already flagged)
/// - Same pattern order and risk levels as bash
pub fn check_trufflehog_activity<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "Checking for Trufflehog activity and secret scanning...",
    );

    let mut findings = Vec::new();
    let mut flagged_files: HashSet<String> = HashSet::new();

    // Collect code files (matching bash: script_files + code_files)
    let extensions = &["js", "py", "sh", "json", "ts"];
    let code_files: Vec<_> = WalkDir::new(&scan_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| extensions.contains(&ext))
        })
        .collect();

    // 1. Look for trufflehog files by name (always HIGH RISK)
    // BASH: grep "trufflehog" all_files_raw.txt - matches ANY file with trufflehog in path
    for entry in WalkDir::new(&scan_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let filename = entry.file_name().to_string_lossy().to_lowercase();
        if filename.contains("trufflehog") {
            let path_key = entry.path().to_string_lossy().to_lowercase();
            findings.push(Finding::new(
                entry.path().to_path_buf(),
                "Trufflehog binary found".to_string(),
                RiskLevel::High,
                "trufflehog_binary",
            ));
            flagged_files.insert(path_key);
        }
    }

    // 2. HIGH PRIORITY: Dynamic TruffleHog download patterns (November 2025 attack)
    // BASH: NO deduplication for HIGH priority checks!
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if DOWNLOAD_PATTERN.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "November 2025 pattern - Dynamic TruffleHog download via curl/wget/Bun"
                        .to_string(),
                    RiskLevel::High,
                    "trufflehog_download",
                ));
                // Add to flagged_files so MEDIUM/LOW checks skip this file
                let path_key = entry.path().to_string_lossy().to_lowercase();
                flagged_files.insert(path_key);
            }
        }
    }

    // 3. HIGH PRIORITY: TruffleHog credential harvesting patterns
    // BASH: NO deduplication for HIGH priority checks!
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if CREDENTIAL_SCAN_PATTERN.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "TruffleHog credential scanning pattern detected".to_string(),
                    RiskLevel::High,
                    "trufflehog_credential_scan",
                ));
                let path_key = entry.path().to_string_lossy().to_lowercase();
                flagged_files.insert(path_key);
            }
        }
    }

    // 4. HIGH PRIORITY: Credential patterns with exfiltration indicators
    // BASH: NO deduplication for HIGH priority checks!
    for entry in &code_files {
        let path_str = entry.path().to_string_lossy();
        // BASH: grep -v "/node_modules/\|\.d\.ts$"
        if path_str.contains("/node_modules/")
            || path_str.contains("\\node_modules\\")
            || path_str.ends_with(".d.ts")
        {
            continue;
        }
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if EXFIL_PATTERN.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Credential patterns with potential exfiltration".to_string(),
                    RiskLevel::High,
                    "credential_exfiltration",
                ));
                let path_key = entry.path().to_string_lossy().to_lowercase();
                flagged_files.insert(path_key);
            }
        }
    }

    // 5. MEDIUM PRIORITY: Trufflehog references in source code (not node_modules/docs)
    for entry in &code_files {
        let path_key = entry.path().to_string_lossy().to_lowercase();
        if flagged_files.contains(&path_key) {
            continue;
        }
        let path_str = entry.path().to_string_lossy();
        // BASH: grep -v "/node_modules/\|\.md$\|/docs/\|\.d\.ts$"
        if path_str.contains("/node_modules/")
            || path_str.contains("\\node_modules\\")
            || path_str.ends_with(".md")
            || path_str.contains("/docs/")
            || path_str.ends_with(".d.ts")
        {
            continue;
        }
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if TRUFFLEHOG_REF.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Contains trufflehog references in source code".to_string(),
                    RiskLevel::Medium,
                    "trufflehog_reference",
                ));
                flagged_files.insert(path_key);
            }
        }
    }

    // 6. MEDIUM PRIORITY: Credential scanning patterns (not in type definitions)
    for entry in &code_files {
        let path_key = entry.path().to_string_lossy().to_lowercase();
        if flagged_files.contains(&path_key) {
            continue;
        }
        let path_str = entry.path().to_string_lossy();
        // BASH: grep -v "/node_modules/\|\.d\.ts$\|/docs/"
        if path_str.contains("/node_modules/")
            || path_str.contains("\\node_modules\\")
            || path_str.ends_with(".d.ts")
            || path_str.contains("/docs/")
        {
            continue;
        }
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if CREDENTIAL_PATTERN.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Contains credential scanning patterns".to_string(),
                    RiskLevel::Medium,
                    "credential_patterns",
                ));
                flagged_files.insert(path_key);
            }
        }
    }

    // 7. LOW PRIORITY: Environment variable scanning with suspicious patterns
    // BASH EXACT: (process\.env|os\.environ|getenv).*(scan|harvest|steal|exfiltrat)
    for entry in &code_files {
        let path_key = entry.path().to_string_lossy().to_lowercase();
        if flagged_files.contains(&path_key) {
            continue;
        }
        let path_str = entry.path().to_string_lossy();
        // BASH: grep -v "/node_modules/\|\.d\.ts$"
        if path_str.contains("/node_modules/")
            || path_str.contains("\\node_modules\\")
            || path_str.ends_with(".d.ts")
        {
            continue;
        }
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if ENV_SUSPICIOUS.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Potentially suspicious environment variable access".to_string(),
                    RiskLevel::Low, // BASH EXACT: This is LOW risk!
                    "env_suspicious",
                ));
                flagged_files.insert(path_key);
            }
        }
    }

    findings
}
