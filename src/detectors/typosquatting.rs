// Typosquatting Detector
// Rust port of: check_typosquatting()

use crate::detectors::{Finding, RiskLevel};
use serde_json::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Popular packages commonly targeted for typosquatting
const POPULAR_PACKAGES: &[&str] = &[
    "react",
    "vue",
    "angular",
    "express",
    "lodash",
    "axios",
    "typescript",
    "webpack",
    "babel",
    "eslint",
    "jest",
    "mocha",
    "chalk",
    "debug",
    "commander",
    "inquirer",
    "yargs",
    "request",
    "moment",
    "underscore",
    "jquery",
    "bootstrap",
    "socket.io",
    "redis",
    "mongoose",
    "passport",
];

// Function: check_typosquatting
// Purpose: Detect typosquatting and homoglyph attacks in package dependencies
// Args: $1 = scan_dir (directory to scan)
// Modifies: TYPOSQUATTING_WARNINGS (global array)
// Returns: Populates TYPOSQUATTING_WARNINGS with Unicode chars, confusables, and similar names
pub fn check_typosquatting<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    let mut findings = Vec::new();

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "package.json")
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                // Extract package names from dependencies sections
                for section in &[
                    "dependencies",
                    "devDependencies",
                    "peerDependencies",
                    "optionalDependencies",
                ] {
                    if let Some(deps) = json.get(section).and_then(|v| v.as_object()) {
                        for package_name in deps.keys() {
                            // Skip if too short or no alpha chars
                            if package_name.len() < 2
                                || !package_name.chars().any(|c| c.is_alphabetic())
                            {
                                continue;
                            }

                            // Check for non-ASCII characters (Unicode/homoglyph)
                            let has_unicode = !package_name.chars().all(|c| {
                                c.is_ascii_alphanumeric()
                                    || c == '@'
                                    || c == '/'
                                    || c == '.'
                                    || c == '_'
                                    || c == '-'
                            });

                            if has_unicode {
                                findings.push(Finding::new(
                                    entry.path().to_path_buf(),
                                    format!(
                                        "Potential Unicode/homoglyph characters in package: {}",
                                        package_name
                                    ),
                                    RiskLevel::Medium,
                                    "typosquatting",
                                ));
                            }

                            // Check for confusable characters (common typosquatting patterns)
                            let confusables = [
                                ("rn", "m"),
                                ("vv", "w"),
                                ("cl", "d"),
                                ("ii", "i"),
                                ("nn", "n"),
                                ("oo", "o"),
                            ];
                            for (pattern, _target) in &confusables {
                                if package_name.contains(pattern) {
                                    findings.push(Finding::new(
                                        entry.path().to_path_buf(),
                                        format!(
                                            "Potential typosquatting pattern '{}' in package: {}",
                                            pattern, package_name
                                        ),
                                        RiskLevel::Medium,
                                        "typosquatting",
                                    ));
                                    break;
                                }
                            }

                            // Check similarity to popular packages
                            for popular in POPULAR_PACKAGES {
                                if package_name == popular {
                                    continue; // Exact match is OK
                                }

                                // Skip common legitimate variations
                                if matches!(
                                    package_name.as_str(),
                                    "test"
                                        | "tests"
                                        | "testing"
                                        | "types"
                                        | "util"
                                        | "utils"
                                        | "core"
                                        | "lib"
                                        | "libs"
                                        | "common"
                                        | "shared"
                                ) {
                                    continue;
                                }

                                // Check for single character differences (Levenshtein distance = 1)
                                if package_name.len() == popular.len() && package_name.len() > 4 {
                                    let diff_count = package_name
                                        .chars()
                                        .zip(popular.chars())
                                        .filter(|(a, b)| a != b)
                                        .count();

                                    if diff_count == 1
                                        && !package_name.contains('-')
                                        && !popular.contains('-')
                                    {
                                        findings.push(Finding::new(
                                            entry.path().to_path_buf(),
                                            format!("Potential typosquatting of '{}': {} (1 character difference)", popular, package_name),
                                            RiskLevel::Medium,
                                            "typosquatting",
                                        ));
                                    }
                                }

                                // Check for missing character
                                if package_name.len() == popular.len() - 1 {
                                    for i in 0..=popular.len() {
                                        let test_name = format!(
                                            "{}{}",
                                            &popular[..i],
                                            &popular[i.min(popular.len())..].get(1..).unwrap_or("")
                                        );
                                        if *package_name == test_name {
                                            findings.push(Finding::new(
                                                entry.path().to_path_buf(),
                                                format!("Potential typosquatting of '{}': {} (missing character)", popular, package_name),
                                                RiskLevel::Medium,
                                                "typosquatting",
                                            ));
                                            break;
                                        }
                                    }
                                }

                                // Check for extra character - UNICODE SAFE
                                if package_name.chars().count() == popular.chars().count() + 1 {
                                    let pkg_chars: Vec<char> = package_name.chars().collect();
                                    for i in 0..=pkg_chars.len() {
                                        let mut test_chars = pkg_chars.clone();
                                        if i < test_chars.len() {
                                            test_chars.remove(i);
                                        }
                                        let test_name: String = test_chars.iter().collect();
                                        if test_name == *popular {
                                            findings.push(Finding::new(
                                                entry.path().to_path_buf(),
                                                format!("Potential typosquatting of '{}': {} (extra character)", popular, package_name),
                                                RiskLevel::Medium,
                                                "typosquatting",
                                            ));
                                            break;
                                        }
                                    }
                                }
                            }

                            // Check for namespace confusion
                            if package_name.starts_with('@') {
                                let suspicious_namespaces = [
                                    "@types",
                                    "@angular",
                                    "@typescript",
                                    "@react",
                                    "@vue",
                                    "@babel",
                                ];
                                if let Some(slash_pos) = package_name.find('/') {
                                    let namespace = &package_name[..slash_pos];

                                    for suspicious in &suspicious_namespaces {
                                        if namespace != *suspicious
                                            && namespace.contains(&suspicious[1..])
                                        {
                                            // Check similarity
                                            let ns_clean = &namespace[1..];
                                            let sus_clean = &suspicious[1..];

                                            if ns_clean.len() == sus_clean.len() {
                                                let diff = ns_clean
                                                    .chars()
                                                    .zip(sus_clean.chars())
                                                    .filter(|(a, b)| a != b)
                                                    .count();
                                                if diff >= 1 && diff <= 2 {
                                                    findings.push(Finding::new(
                                                        entry.path().to_path_buf(),
                                                        format!("Suspicious namespace variation: {} (similar to {})", namespace, suspicious),
                                                        RiskLevel::Medium,
                                                        "typosquatting",
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    findings
}
