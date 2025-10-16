// Lockfile Resolver - Parse package-lock.json, yarn.lock, pnpm-lock.yaml
// Purpose: Extract actual installed versions to verify against compromised packages

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LockfileType {
    Npm,  // package-lock.json
    Yarn, // yarn.lock
    Pnpm, // pnpm-lock.yaml
}

/// Main resolver - tries all lockfile formats
pub struct LockfileResolver {
    pub packages: HashMap<String, String>, // package_name -> version
    pub lockfile_type: Option<LockfileType>,
}

impl LockfileResolver {
    /// Load lockfiles from a directory
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();

        // Try package-lock.json first (most common)
        if let Ok(resolver) = Self::load_npm_lockfile(dir) {
            return Ok(resolver);
        }

        // Try pnpm-lock.yaml
        if let Ok(resolver) = Self::load_pnpm_lockfile(dir) {
            return Ok(resolver);
        }

        // Try yarn.lock
        if let Ok(resolver) = Self::load_yarn_lockfile(dir) {
            return Ok(resolver);
        }

        // No lockfile found
        Ok(Self {
            packages: HashMap::new(),
            lockfile_type: None,
        })
    }

    /// Get resolved version for a package
    pub fn get_version(&self, package_name: &str) -> Option<&str> {
        self.packages.get(package_name).map(|s| s.as_str())
    }

    /// Check if lockfile was found
    pub fn has_lockfile(&self) -> bool {
        self.lockfile_type.is_some()
    }

    // NPM package-lock.json parser
    fn load_npm_lockfile<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let lockfile_path = dir.as_ref().join("package-lock.json");
        if !lockfile_path.exists() {
            anyhow::bail!("package-lock.json not found");
        }

        let content =
            fs::read_to_string(&lockfile_path).context("Failed to read package-lock.json")?;

        let lockfile: NpmLockfile =
            serde_json::from_str(&content).context("Failed to parse package-lock.json")?;

        let mut packages = HashMap::new();

        // NPM v1-v2 format
        if let Some(deps) = lockfile.dependencies {
            extract_npm_packages(&deps, &mut packages);
        }

        // NPM v3+ format
        if let Some(pkgs) = lockfile.packages {
            for (path, pkg_info) in pkgs {
                if let Some(name) = extract_package_name(&path) {
                    if let Some(version) = pkg_info.version {
                        packages.insert(name, version);
                    }
                }
            }
        }

        Ok(Self {
            packages,
            lockfile_type: Some(LockfileType::Npm),
        })
    }

    // PNPM pnpm-lock.yaml parser
    fn load_pnpm_lockfile<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let lockfile_path = dir.as_ref().join("pnpm-lock.yaml");
        if !lockfile_path.exists() {
            anyhow::bail!("pnpm-lock.yaml not found");
        }

        let content =
            fs::read_to_string(&lockfile_path).context("Failed to read pnpm-lock.yaml")?;

        let mut packages = HashMap::new();

        // Parse YAML - two patterns:
        // 1. Dependencies section: "  debug:\n    version: 4.3.4"
        // 2. Packages section: "/debug@4.3.4:"

        let mut current_package: Option<String> = None;
        let mut in_dependencies = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Track if we're in dependencies section
            if line.starts_with("dependencies:") || line.starts_with("devDependencies:") {
                in_dependencies = true;
                continue;
            }
            if !line.starts_with(' ') && !line.is_empty() && line.contains(':') {
                in_dependencies = false;
            }

            // Pattern 1: Dependencies section
            if in_dependencies && line.starts_with("  ") && !line.starts_with("    ") {
                // Package name line: "  debug:"
                if let Some(pkg_name) = line.trim().strip_suffix(':') {
                    current_package = Some(pkg_name.to_string());
                }
            } else if in_dependencies && line.starts_with("    version: ") {
                // Version line: "    version: 4.3.4"
                if let Some(pkg_name) = &current_package {
                    let version = line
                        .trim()
                        .strip_prefix("version: ")
                        .unwrap_or("")
                        .to_string();
                    packages.insert(pkg_name.clone(), version);
                    current_package = None;
                }
            }

            // Pattern 2: Packages section
            // PNPM v6/v7: "/debug@4.3.4:"
            // PNPM v9:   "'@alloc/quick-lru@5.2.0':"
            if (trimmed.starts_with('/') || trimmed.starts_with('\''))
                && trimmed.contains('@')
                && trimmed.ends_with(':')
            {
                let pkg_line = trimmed
                    .trim_start_matches('/')
                    .trim_start_matches('\'')
                    .trim_end_matches('\'')
                    .trim_end_matches(':');

                if let Some(at_pos) = pkg_line.rfind('@') {
                    let name = pkg_line[..at_pos].to_string();
                    let version = pkg_line[at_pos + 1..].to_string();
                    packages.insert(name, version);
                }
            }
        }

        Ok(Self {
            packages,
            lockfile_type: Some(LockfileType::Pnpm),
        })
    }

    // Yarn yarn.lock parser
    fn load_yarn_lockfile<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let lockfile_path = dir.as_ref().join("yarn.lock");
        if !lockfile_path.exists() {
            anyhow::bail!("yarn.lock not found");
        }

        let content = fs::read_to_string(&lockfile_path).context("Failed to read yarn.lock")?;

        let mut packages = HashMap::new();
        let mut current_package: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();

            // Package declaration: "package@version", package@version:
            if !line.starts_with(' ') && line.contains('@') && line.ends_with(':') {
                let pkg_line = line.trim_end_matches(':');
                // Extract first package name from "pkg@^1.0.0", "@scope/pkg@^2.0.0"
                if let Some(at_pos) = pkg_line.find('@') {
                    let name = if pkg_line.starts_with('"') {
                        pkg_line[1..at_pos].to_string()
                    } else if pkg_line.starts_with('@') {
                        // @scope/package@version
                        if let Some(second_at) = pkg_line[1..].find('@') {
                            pkg_line[..=second_at].to_string()
                        } else {
                            continue;
                        }
                    } else {
                        pkg_line[..at_pos].to_string()
                    };
                    current_package = Some(name);
                }
            }
            // Version line: version "1.2.3"
            else if line.starts_with("version ") {
                if let Some(pkg_name) = &current_package {
                    let version = line
                        .trim_start_matches("version ")
                        .trim_matches('"')
                        .to_string();
                    packages.insert(pkg_name.clone(), version);
                    current_package = None;
                }
            }
        }

        Ok(Self {
            packages,
            lockfile_type: Some(LockfileType::Yarn),
        })
    }
}

// NPM lockfile JSON structures
#[derive(Debug, Deserialize)]
struct NpmLockfile {
    dependencies: Option<HashMap<String, NpmPackage>>,
    packages: Option<HashMap<String, NpmPackageInfo>>,
}

#[derive(Debug, Deserialize)]
struct NpmPackage {
    version: Option<String>,
    dependencies: Option<HashMap<String, NpmPackage>>,
}

#[derive(Debug, Deserialize)]
struct NpmPackageInfo {
    version: Option<String>,
}

// Helper: Extract package name from node_modules path
fn extract_package_name(path: &str) -> Option<String> {
    if path.is_empty() || path == "node_modules" {
        return None;
    }

    let path = path.trim_start_matches("node_modules/");

    if path.starts_with('@') {
        // @scope/package
        let parts: Vec<&str> = path.splitn(3, '/').collect();
        if parts.len() >= 2 {
            Some(format!("{}/{}", parts[0], parts[1]))
        } else {
            None
        }
    } else {
        // package
        path.split('/').next().map(|s| s.to_string())
    }
}

// Helper: Recursively extract packages from NPM v1-v2 format
fn extract_npm_packages(
    deps: &HashMap<String, NpmPackage>,
    packages: &mut HashMap<String, String>,
) {
    for (name, pkg) in deps {
        if let Some(version) = &pkg.version {
            packages.insert(name.clone(), version.clone());
        }
        if let Some(subdeps) = &pkg.dependencies {
            extract_npm_packages(subdeps, packages);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_npm_lockfile_v3() {
        let temp_dir = TempDir::new().unwrap();
        let lockfile_content = r#"{
            "name": "test-project",
            "lockfileVersion": 3,
            "packages": {
                "": {},
                "node_modules/debug": {
                    "version": "4.3.4"
                },
                "node_modules/@scope/package": {
                    "version": "1.2.3"
                }
            }
        }"#;

        fs::write(temp_dir.path().join("package-lock.json"), lockfile_content).unwrap();

        let resolver = LockfileResolver::load_from_dir(temp_dir.path()).unwrap();
        assert_eq!(resolver.lockfile_type, Some(LockfileType::Npm));
        assert_eq!(resolver.get_version("debug"), Some("4.3.4"));
        assert_eq!(resolver.get_version("@scope/package"), Some("1.2.3"));
    }

    #[test]
    fn test_pnpm_lockfile() {
        let temp_dir = TempDir::new().unwrap();
        let lockfile_content = r#"
lockfileVersion: '6.0'
dependencies:
  debug:
    specifier: ^4.3.4
    version: 4.3.4

packages:
  /debug@4.3.4:
    resolution: {integrity: sha512-xxx}
  /ansi-regex@6.1.0:
    resolution: {integrity: sha512-yyy}
"#;

        fs::write(temp_dir.path().join("pnpm-lock.yaml"), lockfile_content).unwrap();

        let resolver = LockfileResolver::load_from_dir(temp_dir.path()).unwrap();
        assert_eq!(resolver.lockfile_type, Some(LockfileType::Pnpm));
        assert_eq!(resolver.get_version("debug"), Some("4.3.4"));
        assert_eq!(resolver.get_version("ansi-regex"), Some("6.1.0"));
    }

    #[test]
    fn test_extract_package_name() {
        assert_eq!(
            extract_package_name("node_modules/debug"),
            Some("debug".to_string())
        );
        assert_eq!(
            extract_package_name("node_modules/@scope/package"),
            Some("@scope/package".to_string())
        );
        assert_eq!(extract_package_name(""), None);
    }

    #[test]
    #[ignore] // Manual only - requires barcode-scanner-v2
    fn test_real_barcode_scanner_lockfile() {
        let path = "../../barcode-scanner-v2";

        if let Ok(resolver) = LockfileResolver::load_from_dir(path) {
            if resolver.has_lockfile() {
                println!("\n✅ Lockfile type: {:?}", resolver.lockfile_type);
                println!("✅ Total packages: {}", resolver.packages.len());

                // Test the 4 "NEEDS REVIEW" packages
                if let Some(v) = resolver.get_version("ansi-regex") {
                    println!("✅ ansi-regex: {} (compromised: 6.2.1)", v);
                }
                if let Some(v) = resolver.get_version("error-ex") {
                    println!("✅ error-ex: {} (compromised: 1.3.3)", v);
                }
                if let Some(v) = resolver.get_version("is-arrayish") {
                    println!("✅ is-arrayish: {} (compromised: 0.3.3)", v);
                }
                if let Some(v) = resolver.get_version("vue-demi") {
                    println!("✅ vue-demi: {}", v);
                }
            }
        }
    }
}
