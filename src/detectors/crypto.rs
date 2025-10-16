// Cryptocurrency Theft Patterns Detector - OPTIMIZED VERSION
// Combined best features from V3 and Final for precise Bash matching

use crate::detectors::{Finding, RiskLevel};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

lazy_static! {
    static ref ETH_WALLET: Regex = Regex::new(r"0x[a-fA-F0-9]{40}").unwrap();
    static ref KNOWN_WALLETS: Regex = Regex::new(
        r"0xFc4a4858bafef54D1b1d7697bfb5c52F4c166976|1H13VnQJKtT4HjD5ZFKaaiZEetMbG7nDHx|TB9emsCq6fQw6wRk4HBxxNnU6Hwt1DnV67"
    ).unwrap();
    static ref MALICIOUS_FUNCTIONS: Regex =
        Regex::new(r"checkethereumw|runmask|newdlocal|_0x19ca67").unwrap();
    static ref CRYPTO_REGEX: Regex = Regex::new(r"ethereum.*0x\[a-fA-F0-9\]|bitcoin.*\[13\]\[a-km-zA-HJ-NP-Z1-9\]").unwrap();
}

/// Detect cryptocurrency theft patterns - V3 enhanced for exact Bash compatibility
pub fn check_crypto_theft_patterns<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking for cryptocurrency theft patterns...",
    );

    let mut findings = Vec::new();
    let extensions = &["js", "ts", "json"];

    for entry in WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        })
    {
        let path_str = entry.path().to_string_lossy().to_string();

        if let Ok(content) = fs::read_to_string(entry.path()) {
            // Check for XMLHttpRequest hijacking - EXACT BASH Logic for 100% compatibility
            if content.contains("XMLHttpRequest.prototype.send") {
                // Bash exact logic: framework paths get LOW RISK (effectively ignored in MEDIUM count)
                // Check for framework code in node_modules
                let is_framework = path_str
                    .contains("/node_modules/react-native/Libraries/Network/")
                    || path_str.contains("\\node_modules\\react-native\\Libraries\\Network\\")
                    || path_str.contains("/node_modules/next/dist/compiled/")
                    || path_str.contains("\\node_modules\\next\\dist\\compiled\\");

                if is_framework {
                    // Bash: framework code gets LOW RISK (not counted in MEDIUM)
                    let has_crypto = KNOWN_WALLETS.is_match(&content)
                        || content.contains("checkethereumw")
                        || content.contains("runmask")
                        || content.contains("webhook.site")
                        || content.contains("npmjs.help");

                    if has_crypto {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            "XMLHttpRequest prototype modification with crypto patterns detected - HIGH RISK".to_string(),
                            RiskLevel::High,
                            "crypto_xhr_hijack",
                        ));
                    } else {
                        // BASH EXACT: Framework XMLHttpRequest = LOW RISK with "Crypto pattern" file_path
                        findings.push(Finding::new(
                            std::path::PathBuf::from("Crypto pattern"),
                            format!(
                                "{}:XMLHttpRequest prototype modification detected in framework code - LOW RISK",
                                crate::utils::normalize_path(&entry.path().to_path_buf())
                            ),
                            RiskLevel::Low,
                            "crypto_xhr_framework",
                        ));
                    }
                } else {
                    // Non-framework: check for crypto patterns
                    let has_crypto = KNOWN_WALLETS.is_match(&content)
                        || content.contains("checkethereumw")
                        || content.contains("runmask")
                        || content.contains("webhook.site")
                        || content.contains("npmjs.help");

                    if has_crypto {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            "XMLHttpRequest prototype modification with crypto patterns detected - HIGH RISK".to_string(),
                            RiskLevel::High,
                            "crypto_xhr_hijack",
                        ));
                    } else {
                        // BASH FIX: XMLHttpRequest without crypto = MEDIUM RISK (simple-xhr.js case)
                        let finding = Finding::new(
                            entry.path().to_path_buf(),
                            "XMLHttpRequest prototype modification detected - MEDIUM RISK"
                                .to_string(),
                            RiskLevel::Medium,
                            "crypto_xhr_simple",
                        );

                        // Note: No hardcoded pattern verification
                        // Only lockfile/runtime verification is used

                        findings.push(finding);
                    }
                }
            }

            // Check for known attacker wallets (always HIGH RISK)
            if KNOWN_WALLETS.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Known attacker wallet address detected - HIGH RISK".to_string(),
                    RiskLevel::High,
                    "crypto_attacker_wallet",
                ));
            }

            // Check for wallet address patterns (V3 enhanced)
            if ETH_WALLET.is_match(&content) {
                if content.contains("ethereum")
                    || content.contains("wallet")
                    || content.contains("address")
                    || content.contains("crypto")
                {
                    findings.push(Finding::new(
                        entry.path().to_path_buf(),
                        "Ethereum wallet address patterns detected".to_string(),
                        RiskLevel::Medium,
                        "crypto_wallet_pattern",
                    ));
                }
            }

            // Check for specific malicious functions from chalk/debug attack
            if MALICIOUS_FUNCTIONS.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Known crypto theft function names detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_malicious_functions",
                ));
            }

            // Check for npmjs.help phishing domain
            if content.contains("npmjs.help") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Phishing domain npmjs.help detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_phishing",
                ));
            }

            // Check for javascript obfuscation patterns
            if content.contains("javascript-obfuscator") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "JavaScript obfuscation detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_obfuscation",
                ));
            }

            // Check for cryptocurrency address regex patterns
            if CRYPTO_REGEX.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Cryptocurrency regex patterns detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_regex_patterns",
                ));
            }
        }
    }

    findings
}
