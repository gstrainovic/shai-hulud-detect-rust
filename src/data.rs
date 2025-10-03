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

    if packages_file.exists() {
        let content =
            fs::read_to_string(packages_file).context("Failed to read compromised-packages.txt")?;

        for line in content.lines() {
            // Trim potential Windows carriage returns
            let line = line.trim_end_matches('\r');

            // Skip comments and empty lines
            if line.trim().starts_with('#') || line.trim().is_empty() {
                continue;
            }

            // Add valid package:version lines to set
            if let Some(pkg) = CompromisedPackage::from_line(line) {
                packages.insert(pkg);
            }
        }

        crate::colors::print_status(
            crate::colors::Color::Blue,
            &format!(
                "📦 Loaded {} compromised packages from {}",
                packages.len(),
                packages_file.display()
            ),
        );
    } else {
        // Fallback to embedded list if file not found
        crate::colors::print_status(
            crate::colors::Color::Yellow,
            &format!(
                "⚠️  Warning: {} not found, using embedded package list",
                packages_file.display()
            ),
        );

        // Core compromised packages - fallback list
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
    }

    Ok(packages)
}

// Helper to load both packages and hashes
pub fn load_detection_data<P: AsRef<Path>>(
    packages_file: P,
) -> Result<(HashSet<CompromisedPackage>, HashSet<String>)> {
    let packages = load_compromised_packages(packages_file)?;
    let hashes: HashSet<String> = MALICIOUS_HASHLIST.iter().map(|s| s.to_string()).collect();
    Ok((packages, hashes))
}

// Function: is_compromised_namespace
// Purpose: Check if package name belongs to compromised namespace
// Args: package_name - name of package to check
// Returns: true if package is in compromised namespace
pub fn is_compromised_namespace(package_name: &str) -> bool {
    COMPROMISED_NAMESPACES
        .iter()
        .any(|ns| package_name.starts_with(ns))
}
