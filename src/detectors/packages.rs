// Package Detector
// Rust port of: check_packages()
// Updated to match PR #84 changes: only exact matches, no semver matching for package.json

use crate::data::CompromisedPackage;
use crate::detectors::{lockfile_resolver::LockfileResolver, Finding, RiskLevel};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_packages
// Purpose: Scan package.json files for compromised packages and suspicious namespaces
// Args: $1 = scan_dir (directory to scan), compromised_packages - set of known bad packages
//       lockfile_resolver - optional lockfile for verification (unused after PR #84)
// Modifies: COMPROMISED_FOUND, NAMESPACE_WARNINGS (global arrays)
// Returns: Populates arrays with exact matches only (no semver matching per PR #84)
//
// PR #84 CHANGE: The bash scanner now uses comm -12 for O(n) set intersection
// This means ONLY exact "package_name:version" matches are found - no semver matching.
// The old semver matching logic was removed for performance.
#[allow(unused_variables)]
pub fn check_packages<P: AsRef<Path>>(
    scan_dir: P,
    compromised_packages: &HashSet<CompromisedPackage>,
    lockfile_resolver: Option<&LockfileResolver>,
    runtime_resolver: Option<&mut crate::detectors::runtime_resolver::RuntimeResolver>,
) -> (Vec<Finding>, Vec<Finding>, Vec<Finding>, Vec<Finding>) {
    let scan_dir = scan_dir.as_ref();
    let files_count = crate::utils::count_files_by_name(scan_dir, "package.json");

    crate::colors::print_status(
        crate::colors::Color::Blue,
        &format!("Checking {files_count} package.json files for compromised packages..."),
    );

    let mut compromised_found = Vec::new();
    let suspicious_found = Vec::new(); // No longer used after PR #84
    let lockfile_safe_versions = Vec::new(); // No longer used after PR #84
    let mut namespace_warnings = Vec::new();

    let mut processed = 0;

    // Collect and sort package.json files for consistent order
    let mut package_files: Vec<_> = WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file() && e.file_name() == "package.json")
        .collect();

    // Sort by path for deterministic order matching Bash's find
    package_files.sort_by(|a, b| a.path().cmp(b.path()));

    for entry in package_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                // Check dependencies sections
                for section in &[
                    "dependencies",
                    "devDependencies",
                    "peerDependencies",
                    "optionalDependencies",
                ] {
                    if let Some(deps) = json.get(section).and_then(|v| v.as_object()) {
                        // PR #84: Iterate through dependencies and check for EXACT matches only
                        // No semver matching - just check if "package_name:version" exists in compromised set
                        for (package_name, package_version) in deps {
                            let version_str = package_version.as_str().unwrap_or("");

                            // PR #84: Exact match only - no semver
                            // Check if this exact package:version is in the compromised list
                            let lookup_key = CompromisedPackage {
                                name: package_name.clone(),
                                version: version_str.to_string(),
                            };

                            if compromised_packages.contains(&lookup_key) {
                                compromised_found.push(Finding::new(
                                    entry.path().to_path_buf(),
                                    format!("{package_name}@{version_str}"),
                                    RiskLevel::High,
                                    "compromised_package",
                                ));
                            }
                        }
                    }
                }

                // Check for suspicious namespaces - BASH EXACT: warn for EACH namespace found
                // Bash script: warns once per namespace per file
                let package_str = serde_json::to_string(&json).unwrap_or_default();
                for namespace in crate::data::COMPROMISED_NAMESPACES {
                    if package_str.contains(&format!("\"{namespace}/")) {
                        namespace_warnings.push(Finding::new(
                            // BASH EXACT: Use "Namespace warning" as file_path for compatibility
                            std::path::PathBuf::from("Namespace warning"),
                            format!(
                                "Contains packages from compromised namespace: {} (found in {})",
                                namespace,
                                entry
                                    .path()
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                            ),
                            RiskLevel::Low, // BASH EXACT: namespace warnings are LOW risk
                            "namespace_warning",
                        ));
                    }
                }
            }
        }

        processed += 1;
        crate::utils::show_progress(processed, files_count);
    }

    crate::utils::clear_progress();

    (
        compromised_found,
        suspicious_found,
        lockfile_safe_versions,
        namespace_warnings,
    )
}
