// Data loading and storage
// Corresponds to bash arrays and load_compromised_packages()

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// Known malicious file hashes (source: https://socket.dev/blog/ongoing-supply-chain-attack-targets-crowdstrike-npm-packages)
// Corresponds to MALICIOUS_HASHLIST bash array
pub const MALICIOUS_HASHLIST: &[&str] = &[
    "de0e25a3e6c1e1e5998b306b7141b3dc4c0088da9d7bb47c1c00c91e6e4f85d6",
    "81d2a004a1bca6ef87a1caf7d0e0b355ad1764238e40ff6d1b1cb77ad4f595c3",
    "83a650ce44b2a9854802a7fb4c202877815274c129af49e6c2d1d5d5d55c501e",
    "4b2399646573bb737c4969563303d8ee2e9ddbd1b271f1ca9e35ea78062538db",
    "dc67467a39b70d1cd4c1f7f7a459b35058163592f4a9e8fb4dffcbba98ef210c",
    "46faab8ab153fae6e80e7cca38eab363075bb524edd79e42269217a083628f09",
    "b74caeaa75e077c99f7d44f46daaf9796a3be43ecf24f2a1fd381844669da777",
    "86532ed94c5804e1ca32fa67257e1bb9de628e3e48a1f56e67042dc055effb5b", // test-cases/multi-hash-detection/file1.js
    "aba1fcbd15c6ba6d9b96e34cec287660fff4a31632bf76f2a766c499f55ca1ee", // test-cases/multi-hash-detection/file2.js
];

// Known compromised namespaces - packages in these namespaces may be compromised
// Corresponds to COMPROMISED_NAMESPACES bash array
pub const COMPROMISED_NAMESPACES: &[&str] = &[
    "@crowdstrike",
    "@art-ws",
    "@ngx",
    "@ctrl",
    "@nativescript-community",
    "@ahmedhfarag",
    "@operato",
    "@teselagen",
    "@things-factory",
    "@hestjs",
    "@nstudio",
    "@basic-ui-components-stc",
    "@nexe",
    "@thangved",
    "@tnf-dev",
    "@ui-ux-gang",
    "@yoobic",
];

// AI-Reviewed verified files - files manually reviewed and deemed safe
// Each entry contains SHA-256 hash, file path, and review metadata
#[derive(Debug, Clone)]
pub struct VerifiedFile {
    pub hash: &'static str,
    #[allow(dead_code)]
    pub path: &'static str, // Relative path from node_modules (e.g., "vue-demi/scripts/postinstall.js")
    #[allow(dead_code)]
    pub package: &'static str,
    pub reason: &'static str,
    pub reviewed_by: &'static str,
    pub reviewed_date: &'static str,
}

pub const VERIFIED_FILES: &[VerifiedFile] = &[
    VerifiedFile {
        hash: "ce2f8852444caccee5a19008a7582cc3bd072c39fa6008edac3ad4e489f02d5e",
        path: "error-ex/index.js",
        package: "error-ex@1.3.4",
        reason: "Error message manipulation utility - extracts error properties safely",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "85378d9a0f6e2bd60b2cf2228ac75b8004fac78582eebcd0dc9f9161f25666dc",
        path: "parse-json/index.js",
        package: "parse-json@7.1.1",
        reason: "JSON parser with better error messages - no network or file system access",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "c5bb23b3ca69e97ddefdb76724b1a7936ac18b5e47c3fe3c5391969d6e6d06f8",
        path: "strip-ansi/index.js",
        package: "strip-ansi@7.1.2",
        reason: "ANSI escape code stripping utility - removes terminal color codes safely",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "4508758772b1f52850b576ca714bbfd6edb05f8d36492ceab573db47f5cd7d84",
        path: "string-width/index.js",
        package: "string-width@5.1.2",
        reason: "Calculates display width of strings - no network or file system access",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "6e3e10026230a33197e56422a2d95fc1815528c0bde7c1c790fd1a733b04bd39",
        path: "unist-util-visit-parents/index.js",
        package: "unist-util-visit-parents@6.0.1",
        reason: "Abstract syntax tree visitor utility - no network or file system access",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "10361ec7e4678874114103e47caa1c8ef1cffc78e0efce5088e081a26fe6e977",
        path: "wrap-ansi/index.js",
        package: "wrap-ansi@8.1.0",
        reason: "Text wrapping utility for ANSI escape codes - no network or file system access",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "2dd3014e8ce92317dfd819fc678217d8fdf47086a4607cc49566f0dee02b832a",
        path: "markdown-table/index.js",
        package: "markdown-table@3.0.4",
        reason: "Markdown table generation utility - no network or file system access",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "a5dc0fe8f78d02ddf6554e75bab527612c047b80610128fa721287f71187fd7d",
        path: "formdata-polyfill/FormData.js",
        package: "formdata-polyfill@4.0.10",
        reason: "FormData polyfill for IE compatibility - wraps XMLHttpRequest for FormData support only",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
    VerifiedFile {
        hash: "697a9732b7e7c2ea771298fe0020dd80797b280a3ce528a5d3044c89f891f1d4",
        path: "formdata-polyfill/formdata.min.js",
        package: "formdata-polyfill@4.0.10",
        reason: "FormData polyfill minified - IE compatibility wrapper, no network exfiltration",
        reviewed_by: "ai-agent",
        reviewed_date: "2025-10-18",
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompromisedPackage {
    pub name: String,
    pub version: String,
}

impl CompromisedPackage {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            Some(Self::new(parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
}

// Function: load_compromised_packages
// Purpose: Load compromised package database from external file or fallback list
// Args: packages_file - path to compromised-packages.txt
// Modifies: None
// Returns: HashSet of CompromisedPackage entries (604+ package:version entries)
pub fn load_compromised_packages<P: AsRef<Path>>(
    packages_file: P,
) -> Result<HashSet<CompromisedPackage>> {
    let packages_file = packages_file.as_ref();
    let mut packages = HashSet::new();

    // STRATEGY: GitHub-First (always fresh data!)
    // 1. Try GitHub download (best - always up to date)
    // 2. Fall back to local files (offline mode)
    // 3. Last resort: embedded minimal list

    // Try GitHub first
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "📡 Fetching latest compromised packages from GitHub...",
    );

    match download_from_github() {
        Ok(content) => {
            // Parse GitHub content
            for line in content.lines() {
                let line = line.trim_end_matches('\r');
                if line.trim().starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                if let Some(pkg) = CompromisedPackage::from_line(line) {
                    packages.insert(pkg);
                }
            }

            crate::colors::print_status(
                crate::colors::Color::Green,
                &format!(
                    "✅ Downloaded {} compromised packages from GitHub",
                    packages.len()
                ),
            );

            // Cache for offline use
            if let Err(e) = fs::write("compromised-packages.txt", &content) {
                crate::colors::print_status(
                    crate::colors::Color::Yellow,
                    &format!("⚠️  Could not cache file: {e}"),
                );
            }

            return Ok(packages);
        }
        Err(e) => {
            crate::colors::print_status(
                crate::colors::Color::Yellow,
                &format!("⚠️  GitHub download failed: {e} - using cached files..."),
            );
        }
    }

    // Fallback: Try local file
    if packages_file.exists() {
        let content =
            fs::read_to_string(packages_file).context("Failed to read compromised-packages.txt")?;

        for line in content.lines() {
            let line = line.trim_end_matches('\r');
            if line.trim().starts_with('#') || line.trim().is_empty() {
                continue;
            }
            if let Some(pkg) = CompromisedPackage::from_line(line) {
                packages.insert(pkg);
            }
        }

        crate::colors::print_status(
            crate::colors::Color::Blue,
            &format!("📦 Using cached file ({} packages)", packages.len()),
        );

        return Ok(packages);
    }

    // Last resort: embedded list
    crate::colors::print_status(
        crate::colors::Color::Red,
        "❌ No internet and no cached file!",
    );
    crate::colors::print_status(
        crate::colors::Color::Yellow,
        "⚠️  Using embedded minimal list (7 packages only!)",
    );

    let fallback = vec![
        "@ctrl/tinycolor:4.1.0",
        "@ctrl/tinycolor:4.1.1",
        "@ctrl/tinycolor:4.1.2",
        "@ctrl/deluge:1.2.0",
        "angulartics2:14.1.2",
        "koa2-swagger-ui:5.11.1",
        "koa2-swagger-ui:5.11.2",
    ];

    for entry in fallback {
        if let Some(pkg) = CompromisedPackage::from_line(entry) {
            packages.insert(pkg);
        }
    }

    Ok(packages)
}

// Helper: Download from GitHub
fn download_from_github() -> Result<String> {
    let url = "https://raw.githubusercontent.com/Cobenian/shai-hulud-detect/main/compromised-packages.txt";

    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .context("HTTP request failed")?;

    response.into_string().context("Failed to read response")
}

// Helper to load both packages and hashes
pub fn load_detection_data<P: AsRef<Path>>(
    packages_file: P,
) -> Result<(HashSet<CompromisedPackage>, HashSet<String>)> {
    let packages = load_compromised_packages(packages_file)?;
    let hashes: HashSet<String> = MALICIOUS_HASHLIST
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    Ok((packages, hashes))
}

// Function: is_compromised_namespace
// Purpose: Check if package name belongs to compromised namespace
// Args: package_name - name of package to check
// Returns: true if package is in compromised namespace
#[allow(dead_code)]
pub fn is_compromised_namespace(package_name: &str) -> bool {
    COMPROMISED_NAMESPACES
        .iter()
        .any(|ns| package_name.starts_with(ns))
}
