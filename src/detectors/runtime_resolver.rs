// Runtime Package Resolver - Get actual installed versions via package manager
// Purpose: Supplement lockfile parsing by querying actual installed packages
// Usage: ONLY when --verify flag is enabled

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct PnpmListOutput {
    #[serde(default)]
    dependencies: HashMap<String, PnpmPackageInfo>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, PnpmPackageInfo>,
}

#[derive(Debug, Deserialize)]
struct PnpmPackageInfo {
    version: String,
    #[serde(default)]
    dependencies: HashMap<String, PnpmPackageInfo>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, PnpmPackageInfo>,
}

#[derive(Debug, Deserialize)]
struct NpmListOutput {
    #[serde(default)]
    dependencies: HashMap<String, NpmPackage>,
}

#[derive(Debug, Deserialize)]
struct NpmPackage {
    version: String,
    #[serde(default)]
    dependencies: HashMap<String, NpmPackage>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, NpmPackage>,
}

/// Runtime resolver - queries package manager for actual installed versions
pub struct RuntimeResolver {
    pub packages: HashMap<String, String>, // package_name -> version
    base_dir: std::path::PathBuf,          // Store base directory for fallback queries
}

impl RuntimeResolver {
    /// Try to resolve packages using runtime package manager query
    pub fn from_runtime<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();
        let base_dir = dir.to_path_buf();

        // Try pnpm first
        if let Ok(packages) = Self::query_pnpm(dir) {
            return Ok(Self { packages, base_dir });
        }

        // Try npm fallback
        if let Ok(packages) = Self::query_npm(dir) {
            return Ok(Self { packages, base_dir });
        }

        // No runtime resolution available
        Ok(Self {
            packages: HashMap::new(),
            base_dir,
        })
    }
    /// Query pnpm for installed packages
    fn query_pnpm<P: AsRef<Path>>(dir: P) -> Result<HashMap<String, String>> {
        let output = Command::new("pnpm")
            .arg("list")
            .arg("--json")
            .arg("--depth=Infinity")
            .current_dir(dir.as_ref())
            .output()
            .context("Failed to execute pnpm list")?;

        if !output.status.success() {
            anyhow::bail!("pnpm list failed");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // pnpm returns array of workspace results
        let results: Vec<PnpmListOutput> =
            serde_json::from_str(&stdout).context("Failed to parse pnpm list output")?;

        let mut all_packages = HashMap::new();

        for result in results {
            Self::flatten_pnpm_deps(&result.dependencies, &mut all_packages);
            Self::flatten_pnpm_deps(&result.dev_dependencies, &mut all_packages);
        }

        // If no packages found, consider it a failure
        if all_packages.is_empty() {
            anyhow::bail!("pnpm list returned no packages");
        }

        // If we didn't find many packages, try scanning node_modules directly
        // This handles cases where pnpm list doesn't return all transitive deps
        if all_packages.len() < 100 {
            Self::scan_node_modules_fallback(dir.as_ref(), &mut all_packages)?;
        }

        Ok(all_packages)
    }

    /// Query npm for installed packages
    fn query_npm<P: AsRef<Path>>(dir: P) -> Result<HashMap<String, String>> {
        let output = Command::new("npm")
            .arg("list")
            .arg("--json")
            .arg("--depth=999")
            .arg("--all")
            .current_dir(dir.as_ref())
            .output()
            .context("Failed to execute npm list")?;

        if !output.status.success() {
            anyhow::bail!("npm list failed");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: NpmListOutput =
            serde_json::from_str(&stdout).context("Failed to parse npm list output")?;

        let mut all_packages = HashMap::new();
        Self::flatten_npm_deps(&result.dependencies, &mut all_packages);

        // If no packages found, consider it a failure
        if all_packages.is_empty() {
            anyhow::bail!("npm list returned no packages");
        }

        Ok(all_packages)
    }

    /// Recursively flatten pnpm dependencies
    fn flatten_pnpm_deps(
        deps: &HashMap<String, PnpmPackageInfo>,
        output: &mut HashMap<String, String>,
    ) {
        for (name, pkg) in deps {
            // Store first version found (most likely to be used)
            output.entry(name.clone()).or_insert(pkg.version.clone());

            // Recurse into nested dependencies AND devDependencies
            Self::flatten_pnpm_deps(&pkg.dependencies, output);
            Self::flatten_pnpm_deps(&pkg.dev_dependencies, output);
        }
    }

    /// Recursively flatten npm dependencies
    fn flatten_npm_deps(deps: &HashMap<String, NpmPackage>, output: &mut HashMap<String, String>) {
        for (name, pkg) in deps {
            output.entry(name.clone()).or_insert(pkg.version.clone());
            // Recurse into nested dependencies AND devDependencies
            Self::flatten_npm_deps(&pkg.dependencies, output);
            Self::flatten_npm_deps(&pkg.dev_dependencies, output);
        }
    }

    /// Fallback: Scan node_modules directory for package.json files
    /// This handles cases where pnpm list doesn't return all packages
    fn scan_node_modules_fallback<P: AsRef<Path>>(
        dir: P,
        output: &mut HashMap<String, String>,
    ) -> Result<()> {
        use std::fs;
        use walkdir::WalkDir;

        let node_modules = dir.as_ref().join("node_modules");
        if !node_modules.exists() {
            return Ok(());
        }

        // Scan all package.json files in node_modules
        for entry in WalkDir::new(&node_modules)
            .max_depth(3) // Limit depth for performance
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "package.json")
        {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let (Some(name), Some(version)) = (
                        json.get("name").and_then(|n| n.as_str()),
                        json.get("version").and_then(|v| v.as_str()),
                    ) {
                        output
                            .entry(name.to_string())
                            .or_insert(version.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Get version for a package, with fallback to specific package query
    pub fn get_version(&mut self, package_name: &str) -> Option<String> {
        // Try cache first
        if let Some(version) = self.packages.get(package_name) {
            return Some(version.clone());
        }

        // Fallback: Query specific package
        if let Ok(version) = Self::query_specific_package(package_name, &self.base_dir) {
            // Cache it for future lookups
            self.packages
                .insert(package_name.to_string(), version.clone());
            return Some(version);
        }

        None
    }

    /// Query a specific package with pnpm list <package>
    fn query_specific_package(package_name: &str, dir: &Path) -> Result<String> {
        // Try pnpm first
        if let Ok(version) = Self::query_pnpm_specific(package_name, dir) {
            return Ok(version);
        }

        // Fallback to npm
        if let Ok(version) = Self::query_npm_specific(package_name, dir) {
            return Ok(version);
        }

        // Final fallback: Search in node_modules directory
        // This handles transitive dependencies that pnpm/npm list don't return
        if let Ok(version) = Self::find_package_in_node_modules(package_name, dir) {
            return Ok(version);
        }

        anyhow::bail!("Package {} not found", package_name)
    }

    /// Find package version by searching node_modules directory
    /// This is a targeted search for a specific package, NOT a full scan
    fn find_package_in_node_modules(package_name: &str, dir: &Path) -> Result<String> {
        use std::fs;

        let node_modules = dir.join("node_modules");
        if !node_modules.exists() {
            anyhow::bail!("node_modules not found");
        }

        // Handle scoped packages (e.g., @isaacs/cliui)
        let package_path = if package_name.starts_with('@') {
            // Scoped: node_modules/@scope/package/package.json
            node_modules.join(package_name).join("package.json")
        } else {
            // Normal: node_modules/package/package.json
            node_modules.join(package_name).join("package.json")
        };

        // Try direct path first
        if package_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(version) = json.get("version").and_then(|v| v.as_str()) {
                        return Ok(version.to_string());
                    }
                }
            }
        }

        // Fallback: Search in .pnpm directory (pnpm-specific structure)
        let pnpm_dir = node_modules.join(".pnpm");
        if pnpm_dir.exists() {
            // Search for package_name@version pattern
            if let Ok(entries) = fs::read_dir(&pnpm_dir) {
                for entry in entries.flatten() {
                    let dir_name = entry.file_name().to_string_lossy().to_string();

                    // Match: strip-ansi@7.1.2 or @isaacs+cliui@8.0.2
                    let normalized_package = package_name.replace('/', "+");
                    if dir_name.starts_with(&format!("{}@", normalized_package)) {
                        let package_json = entry
                            .path()
                            .join("node_modules")
                            .join(package_name)
                            .join("package.json");

                        if package_json.exists() {
                            if let Ok(content) = fs::read_to_string(&package_json) {
                                if let Ok(json) =
                                    serde_json::from_str::<serde_json::Value>(&content)
                                {
                                    if let Some(version) =
                                        json.get("version").and_then(|v| v.as_str())
                                    {
                                        return Ok(version.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        anyhow::bail!("Package {} not found in node_modules", package_name)
    }

    /// Query specific package with pnpm
    fn query_pnpm_specific(package_name: &str, dir: &Path) -> Result<String> {
        let output = Command::new("pnpm")
            .arg("list")
            .arg(package_name)
            .arg("--json")
            .arg("--depth=0") // Only direct/transitive, not full tree
            .current_dir(dir)
            .output()
            .context("Failed to execute pnpm list")?;

        if !output.status.success() {
            anyhow::bail!("pnpm list failed");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let results: Vec<PnpmListOutput> =
            serde_json::from_str(&stdout).context("Failed to parse pnpm output")?;

        // Extract version from first occurrence
        for result in results {
            if let Some(pkg) = result.dependencies.get(package_name) {
                return Ok(pkg.version.clone());
            }
            if let Some(pkg) = result.dev_dependencies.get(package_name) {
                return Ok(pkg.version.clone());
            }
        }

        anyhow::bail!("Package {} not found in pnpm output", package_name)
    }

    /// Query specific package with npm
    fn query_npm_specific(package_name: &str, dir: &Path) -> Result<String> {
        let output = Command::new("npm")
            .arg("list")
            .arg(package_name)
            .arg("--json")
            .arg("--depth=0")
            .current_dir(dir)
            .output()
            .context("Failed to execute npm list")?;

        if !output.status.success() {
            anyhow::bail!("npm list failed");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: NpmListOutput =
            serde_json::from_str(&stdout).context("Failed to parse npm output")?;

        if let Some(pkg) = result.dependencies.get(package_name) {
            return Ok(pkg.version.clone());
        }

        anyhow::bail!("Package {} not found in npm output", package_name)
    }

    /// Check if any packages were resolved
    pub fn has_packages(&self) -> bool {
        !self.packages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_pnpm_deps() {
        let mut deps = HashMap::new();

        let mut subdeps = HashMap::new();
        subdeps.insert(
            "ms".to_string(),
            PnpmPackageInfo {
                version: "2.1.3".to_string(),
                dependencies: HashMap::new(),
            },
        );

        deps.insert(
            "debug".to_string(),
            PnpmPackageInfo {
                version: "4.3.4".to_string(),
                dependencies: subdeps,
            },
        );

        let mut output = HashMap::new();
        RuntimeResolver::flatten_pnpm_deps(&deps, &mut output);

        assert_eq!(output.get("debug"), Some(&"4.3.4".to_string()));
        assert_eq!(output.get("ms"), Some(&"2.1.3".to_string()));
    }
}
