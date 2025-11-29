// Cryptocurrency Theft Patterns Detector - BASH EXACT VERSION
// Matches bash check_crypto_theft_patterns() from shai-hulud-detector.sh lines 1176-1260
//
// IMPORTANT: Bash does NOT deduplicate between checks!
// Each check writes to the same file, so a file can have multiple findings.
// ONLY the last check (Ethereum wallet patterns) skips already-flagged files.

use crate::detectors::{verification, Finding, RiskLevel};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use walkdir::WalkDir;

static ETH_WALLET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"0x[a-fA-F0-9]{40}").unwrap());
static KNOWN_WALLETS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"0xFc4a4858bafef54D1b1d7697bfb5c52F4c166976|1H13VnQJKtT4HjD5ZFKaaiZEetMbG7nDHx|TB9emsCq6fQw6wRk4HBxxNnU6Hwt1DnV67",
    )
    .unwrap()
});
static MALICIOUS_FUNCTIONS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"checkethereumw|runmask|newdlocal|_0x19ca67").unwrap());

/// Detect cryptocurrency theft patterns - BASH EXACT version
/// Matches bash `check_crypto_theft_patterns()` exactly:
/// - NO deduplication between checks (file can have multiple findings)
/// - ONLY the last check (Ethereum wallet) skips already-flagged files
#[allow(clippy::too_many_lines)]
pub fn check_crypto_theft_patterns<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    crate::colors::print_status(
        crate::colors::Color::Blue,
        "🔍 Checking for cryptocurrency theft patterns...",
    );

    let mut findings = Vec::new();
    let mut flagged_files: HashSet<String> = HashSet::new(); // Only used for last check
    let extensions = &["js", "ts", "json"];

    // Collect all code files first
    let code_files: Vec<_> = WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| extensions.contains(&ext))
        })
        .collect();

    // BASH ORDER: Check patterns in same order as bash scanner
    // NO DEDUPLICATION between checks 1-5!

    // 1. Check for specific malicious functions from chalk/debug attack (highest priority)
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if MALICIOUS_FUNCTIONS.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Known crypto theft function names detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_malicious_functions",
                ));
                // Track for last check only
                flagged_files.insert(entry.path().to_string_lossy().to_lowercase());
            }
        }
    }

    // 2. Check for known attacker wallets (high priority)
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if KNOWN_WALLETS.is_match(&content) {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Known attacker wallet address detected - HIGH RISK".to_string(),
                    RiskLevel::High,
                    "crypto_attacker_wallet",
                ));
                flagged_files.insert(entry.path().to_string_lossy().to_lowercase());
            }
        }
    }

    // 3. Check for npmjs.help phishing domain
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if content.contains("npmjs.help") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Phishing domain npmjs.help detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_phishing",
                ));
                flagged_files.insert(entry.path().to_string_lossy().to_lowercase());
            }
        }
    }

    // 4. Check for XMLHttpRequest hijacking (medium priority)
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if content.contains("XMLHttpRequest.prototype.send") {
                let path_str = entry.path().to_string_lossy().to_string();
                let is_framework = path_str.contains("/react-native/Libraries/Network/")
                    || path_str.contains("\\react-native\\Libraries\\Network\\")
                    || path_str.contains("/next/dist/compiled/")
                    || path_str.contains("\\next\\dist\\compiled\\");

                let has_crypto = ETH_WALLET.is_match(&content)
                    || content.contains("checkethereumw")
                    || content.contains("runmask")
                    || content.contains("webhook.site")
                    || content.contains("npmjs.help");

                if is_framework {
                    if has_crypto {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            "XMLHttpRequest prototype modification with crypto patterns detected - HIGH RISK".to_string(),
                            RiskLevel::High,
                            "crypto_xhr_hijack",
                        ));
                    } else {
                        // BASH EXACT: Framework XMLHttpRequest = LOW RISK
                        findings.push(Finding::new(
                            std::path::PathBuf::from("Crypto pattern"),
                            format!(
                                "{}:XMLHttpRequest prototype modification detected in framework code - LOW RISK",
                                crate::utils::normalize_path(entry.path())
                            ),
                            RiskLevel::Low,
                            "crypto_xhr_framework",
                        ));
                    }
                } else if has_crypto {
                    findings.push(Finding::new(
                        entry.path().to_path_buf(),
                        "XMLHttpRequest prototype modification with crypto patterns detected - HIGH RISK".to_string(),
                        RiskLevel::High,
                        "crypto_xhr_hijack",
                    ));
                } else {
                    let mut finding = Finding::new(
                        entry.path().to_path_buf(),
                        "XMLHttpRequest prototype modification detected - MEDIUM RISK".to_string(),
                        RiskLevel::Medium,
                        "crypto_xhr_simple",
                    );
                    let hash_verification = verification::verify_file_by_hash(entry.path());
                    if let verification::VerificationStatus::Verified { .. } = hash_verification {
                        finding.verification = Some(hash_verification);
                    }
                    findings.push(finding);
                }
                flagged_files.insert(entry.path().to_string_lossy().to_lowercase());
            }
        }
    }

    // 5. Check for javascript obfuscation
    for entry in &code_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if content.contains("javascript-obfuscator") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "JavaScript obfuscation detected".to_string(),
                    RiskLevel::Medium,
                    "crypto_obfuscation",
                ));
                flagged_files.insert(entry.path().to_string_lossy().to_lowercase());
            }
        }
    }

    // 6. Check for generic Ethereum wallet address patterns (MEDIUM priority)
    // BASH EXACT: This is the ONLY check that skips already-flagged files!
    // "if grep -qF "$file:" "$TEMP_DIR/crypto_patterns.txt" 2>/dev/null; then continue"
    for entry in &code_files {
        let path_key = entry.path().to_string_lossy().to_lowercase();
        // ONLY this check skips already-flagged files
        if flagged_files.contains(&path_key) {
            continue;
        }
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if ETH_WALLET.is_match(&content) {
                // Check for crypto-related context keywords
                let content_lower = content.to_lowercase();
                if content_lower.contains("ethereum")
                    || content_lower.contains("wallet")
                    || content_lower.contains("address")
                    || content_lower.contains("crypto")
                {
                    findings.push(Finding::new(
                        entry.path().to_path_buf(),
                        "Ethereum wallet address patterns detected".to_string(),
                        RiskLevel::Medium,
                        "crypto_wallet_pattern",
                    ));
                }
            }
        }
    }

    findings
}
