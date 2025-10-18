// Verification Module - Verify findings to reduce false positives
// Purpose: Check if findings are legitimate patterns vs actual threats

use crate::data::{CompromisedPackage, VERIFIED_FILES};
use crate::detectors::lockfile_resolver::LockfileResolver;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Verified {
        reason: String,
        confidence: Confidence,
        method: VerificationMethod,
    },
    Compromised {
        reason: String,
    },
    Suspicious {
        reason: String,
    },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,   // 95%+ sure (lockfile match, code analysis)
    Medium, // 70-95% (pattern matching)
    Low,    // 50-70% (heuristics)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMethod {
    LockfileMatch,
    CodePatternAnalysis,
    Combined,
}

/// Verify if a package is safe via lockfile or runtime resolution
pub fn verify_via_lockfile(
    package_name: &str,
    lockfile_resolver: Option<&LockfileResolver>,
    runtime_resolver: Option<&mut crate::detectors::runtime_resolver::RuntimeResolver>,
    compromised_packages: &HashSet<CompromisedPackage>,
) -> VerificationStatus {
    // Try runtime resolver first (most accurate - actual installed version)
    if let Some(runtime) = runtime_resolver {
        if let Some(installed_version) = runtime.get_version(package_name) {
            let is_compromised = compromised_packages
                .iter()
                .any(|cp| cp.name == package_name && cp.version == installed_version);

            if is_compromised {
                return VerificationStatus::Compromised {
                    reason: format!("Installed version {} is COMPROMISED", installed_version),
                };
            } else {
                return VerificationStatus::Verified {
                    reason: format!("Installed version {} is safe", installed_version),
                    confidence: Confidence::High,
                    method: VerificationMethod::LockfileMatch,
                };
            }
        }
    }

    // Fallback to lockfile
    if let Some(lockfile) = lockfile_resolver {
        if let Some(locked_version) = lockfile.get_version(package_name) {
            let is_compromised = compromised_packages
                .iter()
                .any(|cp| cp.name == package_name && cp.version == locked_version);

            if is_compromised {
                return VerificationStatus::Compromised {
                    reason: format!("Lockfile pins to COMPROMISED version {}", locked_version),
                };
            } else {
                return VerificationStatus::Verified {
                    reason: format!("Lockfile pins to safe version {}", locked_version),
                    confidence: Confidence::High,
                    method: VerificationMethod::LockfileMatch,
                };
            }
        }
    }

    VerificationStatus::Unknown
}

/// Verify file by SHA-256 hash against AI-reviewed whitelist
pub fn verify_file_by_hash(file_path: &Path) -> VerificationStatus {
    // Calculate SHA-256 hash of file
    let hash = match calculate_file_hash(file_path) {
        Ok(h) => h,
        Err(_) => return VerificationStatus::Unknown,
    };

    // Check against verified files list
    for verified in VERIFIED_FILES {
        if verified.hash == hash {
            return VerificationStatus::Verified {
                reason: format!(
                    "{} (reviewed by {} on {})",
                    verified.reason, verified.reviewed_by, verified.reviewed_date
                ),
                confidence: Confidence::High,
                method: VerificationMethod::CodePatternAnalysis,
            };
        }
    }

    VerificationStatus::Unknown
}

/// Calculate SHA-256 hash of a file
fn calculate_file_hash(file_path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read(file_path)?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_verify_safe_via_lockfile() {
        let mut packages = HashMap::new();
        packages.insert("ansi-regex".to_string(), "6.1.0".to_string());

        let resolver = LockfileResolver {
            packages,
            lockfile_type: Some(crate::detectors::lockfile_resolver::LockfileType::Pnpm),
        };

        let mut compromised = HashSet::new();
        compromised.insert(CompromisedPackage {
            name: "ansi-regex".to_string(),
            version: "6.2.1".to_string(),
        });

        let result = verify_via_lockfile("ansi-regex", Some(&resolver), None, &compromised);

        match result {
            VerificationStatus::Verified {
                reason,
                confidence,
                method,
            } => {
                assert!(reason.contains("6.1.0"));
                assert_eq!(confidence, Confidence::High);
                assert_eq!(method, VerificationMethod::LockfileMatch);
            }
            _ => panic!("Expected Verified status"),
        }
    }

    #[test]
    fn test_verify_compromised_via_lockfile() {
        let mut packages = HashMap::new();
        packages.insert("ansi-regex".to_string(), "6.2.1".to_string());

        let resolver = LockfileResolver {
            packages,
            lockfile_type: Some(crate::detectors::lockfile_resolver::LockfileType::Pnpm),
        };

        let mut compromised = HashSet::new();
        compromised.insert(CompromisedPackage {
            name: "ansi-regex".to_string(),
            version: "6.2.1".to_string(),
        });

        let result = verify_via_lockfile("ansi-regex", Some(&resolver), None, &compromised);

        match result {
            VerificationStatus::Compromised { reason } => {
                assert!(reason.contains("COMPROMISED"));
                assert!(reason.contains("6.2.1"));
            }
            _ => panic!("Expected Compromised status"),
        }
    }

    #[test]
    fn test_verify_unknown_no_lockfile() {
        let resolver = LockfileResolver {
            packages: HashMap::new(),
            lockfile_type: None,
        };

        let compromised = HashSet::new();
        let result = verify_via_lockfile("some-package", Some(&resolver), None, &compromised);

        assert!(matches!(result, VerificationStatus::Unknown));
    }
}
