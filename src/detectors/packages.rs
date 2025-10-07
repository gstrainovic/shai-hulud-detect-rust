// Package Detector
// Rust port of: check_packages()

use crate::data::CompromisedPackage;
use crate::detectors::{Finding, RiskLevel};
use crate::semver;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_packages
// Purpose: Scan package.json files for compromised packages and suspicious namespaces
// Args: $1 = scan_dir (directory to scan), compromised_packages - set of known bad packages
// Modifies: COMPROMISED_FOUND, SUSPICIOUS_FOUND, NAMESPACE_WARNINGS (global arrays)
// Returns: Populates arrays with matches using exact and semver pattern matching
pub fn check_packages<P: AsRef<Path>>(
    scan_dir: P,
    compromised_packages: &HashSet<CompromisedPackage>,
) -> (Vec<Finding>, Vec<Finding>, Vec<Finding>) {
    let scan_dir = scan_dir.as_ref();
    let files_count = crate::utils::count_files_by_name(scan_dir, "package.json");

    crate::colors::print_status(
        crate::colors::Color::Blue,
        &format!(
            "🔍 Checking {} package.json files for compromised packages...",
            files_count
        ),
    );

    let mut compromised_found = Vec::new();
    let mut suspicious_found = Vec::new();
    let mut namespace_warnings = Vec::new();

    let mut processed = 0;

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "package.json")
    {
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
                        for (package_name, package_version) in deps {
                            let version_str = package_version.as_str().unwrap_or("");

                            // Check against compromised packages
                            for comp_pkg in compromised_packages {
                                if package_name != &comp_pkg.name {
                                    continue;
                                }

                                // Exact match
                                if version_str == comp_pkg.version {
                                    compromised_found.push(Finding::new(
                                        entry.path().to_path_buf(),
                                        format!("{}@{}", package_name, version_str),
                                        RiskLevel::High,
                                        "compromised_package",
                                    ));
                                }
                                // Semver pattern match - check lockfile for actual version
                                else if semver::semver_match(&comp_pkg.version, version_str) {
                                    // BASH LINE 447-461: Check lockfile for exact installed version
                                    let package_dir = entry.path().parent().unwrap();
                                    if let Some(actual_version) = crate::detectors::integrity::get_lockfile_version(package_name, package_dir) {
                                        if actual_version == comp_pkg.version {
                                            // Lockfile has exact compromised version!
                                            compromised_found.push(Finding::new(
                                                entry.path().to_path_buf(),
                                                format!("{}@{}", package_name, actual_version),
                                                RiskLevel::High,
                                                "compromised_package",
                                            ));
                                        } else {
                                            // Lockfile has safe version - informational only
                                            // This goes to LOCKFILE_SAFE_VERSIONS in Bash (LOW risk)
                                            // We don't track this in Rust currently
                                        }
                                    } else {
                                        // No lockfile - suspicious (could install compromised on npm install)
                                        suspicious_found.push(Finding::new(
                                            entry.path().to_path_buf(),
                                            format!("{}@{}", package_name, version_str),
                                            RiskLevel::Medium,
                                            "suspicious_package",
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                // Check for suspicious namespaces - BASH EXACT: warn for EACH namespace found
                // Bash script line 453-457: warns once per namespace, no break
                // If file has @ctrl AND @nativescript, it generates 2 warnings
                let package_str = serde_json::to_string(&json).unwrap_or_default();
                for namespace in crate::data::COMPROMISED_NAMESPACES {
                    if package_str.contains(&format!("\"{}/", namespace)) {
                        namespace_warnings.push(Finding::new(
                            entry.path().to_path_buf(),
                            format!(
                                "Contains packages from compromised namespace: {}",
                                namespace
                            ),
                            RiskLevel::Low,
                            "namespace_warning",
                        ));
                        // BASH EXACT: No break! Check ALL namespaces
                    }
                }
            }
        }

        processed += 1;
        crate::utils::show_progress(processed, files_count);
    }

    crate::utils::clear_progress();

    (compromised_found, suspicious_found, namespace_warnings)
}
