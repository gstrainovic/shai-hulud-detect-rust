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
    dependencies: HashMap<String, PnpmPackage>,
}

#[derive(Debug, Deserialize)]
struct PnpmPackage {
    version: String,
    #[serde(default)]
    dependencies: HashMap<String, PnpmPackage>,
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
}

/// Runtime resolver - queries package manager for actual installed versions
pub struct RuntimeResolver {
    pub packages: HashMap<String, String>, // package_name -> version
}

impl RuntimeResolver {
    /// Try to resolve packages using runtime package manager query
    pub fn from_runtime<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();

        // Try pnpm first
        if let Ok(packages) = Self::query_pnpm(dir) {
            return Ok(Self { packages });
        }

        // Try npm
        if let Ok(packages) = Self::query_npm(dir) {
            return Ok(Self { packages });
        }

        // No runtime resolution available
        Ok(Self {
            packages: HashMap::new(),
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

        Ok(all_packages)
    }

    /// Recursively flatten pnpm dependencies
    fn flatten_pnpm_deps(
        deps: &HashMap<String, PnpmPackage>,
        output: &mut HashMap<String, String>,
    ) {
        for (name, pkg) in deps {
            // Store first version found (most likely to be used)
            output.entry(name.clone()).or_insert(pkg.version.clone());

            // Recurse into nested dependencies
            Self::flatten_pnpm_deps(&pkg.dependencies, output);
        }
    }

    /// Recursively flatten npm dependencies
    fn flatten_npm_deps(deps: &HashMap<String, NpmPackage>, output: &mut HashMap<String, String>) {
        for (name, pkg) in deps {
            output.entry(name.clone()).or_insert(pkg.version.clone());
            Self::flatten_npm_deps(&pkg.dependencies, output);
        }
    }

    /// Get version for a package
    pub fn get_version(&self, package_name: &str) -> Option<&str> {
        self.packages.get(package_name).map(|s| s.as_str())
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
            PnpmPackage {
                version: "2.1.3".to_string(),
                dependencies: HashMap::new(),
            },
        );

        deps.insert(
            "debug".to_string(),
            PnpmPackage {
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
