// Package Integrity Detector
// Rust port of: check_package_integrity()

use crate::data::CompromisedPackage;
use crate::detectors::{Finding, RiskLevel};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Verify package lock files for compromised packages and version integrity
/// Rust port of: check_package_integrity()
pub fn check_package_integrity<P: AsRef<Path>>(
    scan_dir: P,
    compromised_packages: &HashSet<CompromisedPackage>,
) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking package lock files for integrity issues...",
    );

    let mut findings = Vec::new();

    // Check package-lock.json, yarn.lock, pnpm-lock.yaml
    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let filename = e.file_name().to_string_lossy();
            filename == "package-lock.json"
                || filename == "yarn.lock"
                || filename == "pnpm-lock.yaml"
        })
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            // For JSON lockfiles
            if entry.file_name() == "package-lock.json" {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    check_json_lockfile(entry.path(), &json, compromised_packages, &mut findings);
                }
            }

            // Check for @ctrl packages (potential worm activity)
            if content.contains("@ctrl") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Recently modified lockfile contains @ctrl packages (potential worm activity)"
                        .to_string(),
                    RiskLevel::Medium,
                    "integrity",
                ));
            }
        }
    }

    findings
}

fn check_json_lockfile(
    path: &Path,
    json: &Value,
    compromised_packages: &HashSet<CompromisedPackage>,
    findings: &mut Vec<Finding>,
) {
    let mut found_packages = std::collections::HashSet::new(); // BASH: prevent duplicates per lockfile

    // Check "packages" section (npm lockfile v2+)
    if let Some(packages) = json.get("packages").and_then(|p| p.as_object()) {
        for (pkg_path, pkg_data) in packages {
            // Extract package name from node_modules path
            if let Some(pkg_name) = pkg_path.strip_prefix("node_modules/") {
                if let Some(version) = pkg_data.get("version").and_then(|v| v.as_str()) {
                    // Check against compromised packages - BASH: exact logic
                    for comp_pkg in compromised_packages {
                        if comp_pkg.name == pkg_name && comp_pkg.version == version {
                            let package_key = format!("{}@{}", pkg_name, version);
                            if !found_packages.contains(&package_key) {
                                found_packages.insert(package_key.clone());
                                findings.push(Finding::new(
                                    path.to_path_buf(),
                                    format!("Compromised package in lockfile: {}", package_key),
                                    RiskLevel::Medium,
                                    "integrity",
                                ));
                                break; // BASH: only one finding per package per lockfile
                            }
                        }
                    }
                }
            }
        }
    }

    // Also check "dependencies" section (npm lockfile v1 and v2 flat format)
    if let Some(dependencies) = json.get("dependencies").and_then(|d| d.as_object()) {
        for (pkg_name, pkg_data) in dependencies {
            if let Some(version) = pkg_data.get("version").and_then(|v| v.as_str()) {
                // Check against compromised packages - BASH: prevent duplicates
                for comp_pkg in compromised_packages {
                    if &comp_pkg.name == pkg_name && comp_pkg.version == version {
                        let package_key = format!("{}@{}", pkg_name, version);
                        if !found_packages.contains(&package_key) {
                            found_packages.insert(package_key.clone());
                            findings.push(Finding::new(
                                path.to_path_buf(),
                                format!("Compromised package in lockfile: {}", package_key),
                                RiskLevel::Medium,
                                "integrity",
                            ));
                            break; // BASH: only one finding per package per lockfile
                        }
                    }
                }
            }
        }
    }
}
