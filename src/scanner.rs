use crate::hash_checker::HashChecker;
use crate::output::{FileResult, ScanResults};
use crate::patterns::PatternMatcher;
use crate::patterns::RiskLevel;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Main scanner struct that orchestrates the detection process
pub struct Scanner {
    scan_path: PathBuf,
    paranoid: bool,
    pattern_matcher: PatternMatcher,
    hash_checker: HashChecker,
    compromised_packages: HashMap<String, Vec<String>>, // package_name -> versions
}

impl Scanner {
    /// Create a new scanner instance
    pub async fn new(scan_path: &Path, paranoid: bool) -> Result<Self> {
        println!("📦 Loading compromised packages database...");
        let compromised_packages = Self::load_compromised_packages()?;
        let package_count = compromised_packages
            .iter()
            .map(|(_, versions)| versions.len())
            .sum::<usize>();
        println!(
            "📦 Loaded {} compromised packages from database",
            package_count
        );

        let pattern_matcher = PatternMatcher::new(paranoid);
        let hash_checker = HashChecker::new();

        Ok(Scanner {
            scan_path: scan_path.to_path_buf(),
            paranoid,
            pattern_matcher,
            hash_checker,
            compromised_packages,
        })
    }

    /// Load compromised packages from ../shai-hulud-detect/compromised-packages.txt
    fn load_compromised_packages() -> Result<HashMap<String, Vec<String>>> {
        let current_dir = std::env::current_dir()?;
        let packages_file = current_dir
            .parent()
            .context("Cannot find parent directory")?
            .join("shai-hulud-detect")
            .join("compromised-packages.txt");

        if !packages_file.exists() {
            anyhow::bail!(
                "Compromised packages file not found: {}",
                packages_file.display()
            );
        }

        let content = fs::read_to_string(&packages_file).context(format!(
            "Failed to read compromised packages from {}",
            packages_file.display()
        ))?;

        let mut packages = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse package:version format
            if let Some((package_name, version)) = line.split_once(':') {
                let package_name = package_name.trim().to_string();
                let version = version.trim().to_string();

                packages
                    .entry(package_name)
                    .or_insert_with(Vec::new)
                    .push(version);
            }
        }

        Ok(packages)
    }

    /// Run the complete scan process
    pub async fn scan(&self) -> Result<ScanResults> {
        println!("🔍 Starting Shai-Hulud detection scan...");
        println!("Scanning directory: {}", self.scan_path.display());

        if self.paranoid {
            println!("🔍+ Running in paranoid mode with additional security checks");
        }

        let mut results = ScanResults::new(&self.scan_path);

        // Step 1: Find all relevant files
        let files = self.find_files()?;
        println!("🔍 Found {} files to analyze", files.len());

        // Step 2: Check package.json files for compromised packages
        self.check_package_files(&files, &mut results).await?;

        // Step 3: Check file hashes against known malicious files
        self.check_file_hashes(&files, &mut results).await?;

        // Step 4: Check for malicious patterns in content
        self.check_content_patterns(&files, &mut results).await?;

        println!("✅ Scan completed");
        Ok(results)
    }

    /// Find all relevant files to scan
    fn find_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.scan_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Include package.json, lock files, and JavaScript files
                if filename == "package.json"
                    || filename == "package-lock.json"
                    || filename == "pnpm-lock.yaml"
                    || filename.ends_with(".js")
                    || filename.ends_with(".ts")
                    || filename.ends_with(".yml")
                    || filename.ends_with(".yaml")
                    || filename.ends_with(".json")
                    || filename.ends_with(".sh")
                    || filename.ends_with(".py")
                    || filename.ends_with(".md")
                {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    /// Check package.json files for compromised packages
    async fn check_package_files(
        &self,
        files: &[PathBuf],
        results: &mut ScanResults,
    ) -> Result<()> {
        let package_files: Vec<_> = files
            .iter()
            .filter(|f| f.file_name().and_then(|n| n.to_str()) == Some("package.json"))
            .collect();

        if package_files.is_empty() {
            return Ok(());
        }

        println!(
            "🔍 Checking {} package.json files for compromised packages...",
            package_files.len()
        );

        for file in package_files {
            if let Ok(content) = fs::read_to_string(file) {
                if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&content) {
                    self.check_package_dependencies(&package_json, file, results);
                }
            }
        }

        Ok(())
    }

    /// Check dependencies in a package.json for compromised packages
    fn check_package_dependencies(
        &self,
        package_json: &serde_json::Value,
        file: &Path,
        results: &mut ScanResults,
    ) {
        let mut detected_packages = Vec::new();

        // Check both dependencies and devDependencies
        for dep_type in ["dependencies", "devDependencies"] {
            if let Some(deps) = package_json.get(dep_type).and_then(|d| d.as_object()) {
                for (package_name, version_spec) in deps {
                    if let Some(version_spec_str) = version_spec.as_str() {
                        if self.is_compromised_package(package_name, version_spec_str) {
                            detected_packages
                                .push(format!("{}@{}", package_name, version_spec_str));
                        }
                    }
                }
            }
        }

        if !detected_packages.is_empty() {
            let risk_level = RiskLevel::High; // Compromised packages are always high risk
            let comment = format!(
                "Contains known compromised packages: {}",
                detected_packages.join(", ")
            );

            results.add_file_result(FileResult {
                file: file.to_string_lossy().to_string(),
                risk_level,
                comment,
                patterns_detected: vec!["compromised_packages".to_string()],
                details: Some(detected_packages),
            });
        }
    }

    /// Check if a package and version combination is compromised
    fn is_compromised_package(&self, package_name: &str, version_spec: &str) -> bool {
        if let Some(compromised_versions) = self.compromised_packages.get(package_name) {
            // For exact version matches
            if compromised_versions.contains(&version_spec.to_string()) {
                return true;
            }

            // For semver ranges, check if any compromised version could match
            // This is a simplified check - in practice, you'd want proper semver parsing
            if version_spec.starts_with('^') || version_spec.starts_with('~') {
                let base_version = version_spec.trim_start_matches('^').trim_start_matches('~');
                for compromised_version in compromised_versions {
                    if compromised_version.starts_with(base_version) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check file hashes against known malicious files
    async fn check_file_hashes(&self, files: &[PathBuf], results: &mut ScanResults) -> Result<()> {
        let js_files: Vec<_> = files
            .iter()
            .filter(|f| f.extension().and_then(|e| e.to_str()) == Some("js"))
            .collect();

        if js_files.is_empty() {
            return Ok(());
        }

        println!(
            "🔍 Checking {} JavaScript files for known malicious content...",
            js_files.len()
        );

        for file in js_files {
            if let Some(hash) = self.hash_checker.calculate_file_hash(file)? {
                if self.hash_checker.is_malicious_hash(&hash) {
                    results.add_file_result(FileResult {
                        file: file.to_string_lossy().to_string(),
                        risk_level: RiskLevel::High,
                        comment: format!("File matches known malicious hash: {}", hash),
                        patterns_detected: vec!["malicious_file_hash".to_string()],
                        details: Some(vec![hash]),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check file content for malicious patterns
    async fn check_content_patterns(
        &self,
        files: &[PathBuf],
        results: &mut ScanResults,
    ) -> Result<()> {
        println!(
            "🔍 Checking {} files for suspicious content patterns...",
            files.len()
        );

        for file in files {
            if let Ok(content) = fs::read_to_string(file) {
                let matches = self.pattern_matcher.check_content(&content);

                if !matches.is_empty() {
                    let max_risk = matches
                        .iter()
                        .map(|m| &m.risk_level)
                        .max()
                        .unwrap_or(&RiskLevel::Low);

                    let patterns: Vec<String> =
                        matches.iter().map(|m| m.pattern_name.clone()).collect();

                    let details: Vec<String> = matches
                        .iter()
                        .map(|m| format!("{}: {}", m.pattern_name, m.description))
                        .collect();

                    results.add_file_result(FileResult {
                        file: file.to_string_lossy().to_string(),
                        risk_level: max_risk.clone(),
                        comment: format!(
                            "Suspicious patterns detected: {}",
                            matches
                                .iter()
                                .map(|m| m.description.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        patterns_detected: patterns,
                        details: Some(details),
                    });
                }
            }
        }

        Ok(())
    }
}
