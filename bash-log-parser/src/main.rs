use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Finding {
    file_path: String,
    message: String,
    risk_level: String,
    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParsedBashLog {
    workflow_files: Vec<Finding>,
    malicious_hashes: Vec<Finding>,
    compromised_found: Vec<Finding>,
    suspicious_found: Vec<Finding>,
    lockfile_safe_versions: Vec<Finding>,
    namespace_warnings: Vec<Finding>,
    postinstall_hooks: Vec<Finding>,
    suspicious_content: Vec<Finding>,
    crypto_patterns: Vec<Finding>,
    git_branches: Vec<Finding>,
    trufflehog: Vec<Finding>,
    shai_hulud_repos: Vec<Finding>,
    integrity: Vec<Finding>,
    typosquatting: Vec<Finding>,
    network_exfiltration: Vec<Finding>,
}

impl ParsedBashLog {
    fn new() -> Self {
        Self {
            workflow_files: Vec::new(),
            malicious_hashes: Vec::new(),
            compromised_found: Vec::new(),
            suspicious_found: Vec::new(),
            lockfile_safe_versions: Vec::new(),
            namespace_warnings: Vec::new(),
            postinstall_hooks: Vec::new(),
            suspicious_content: Vec::new(),
            crypto_patterns: Vec::new(),
            git_branches: Vec::new(),
            trufflehog: Vec::new(),
            shai_hulud_repos: Vec::new(),
            integrity: Vec::new(),
            typosquatting: Vec::new(),
            network_exfiltration: Vec::new(),
        }
    }
}

fn parse_bash_log(log_content: &str) -> Result<ParsedBashLog> {
    let mut parsed = ParsedBashLog::new();

    // Regex patterns to extract findings
    let high_risk_workflow = Regex::new(r"🚨 HIGH RISK: Malicious workflow files detected:")?;
    let high_risk_compromised =
        Regex::new(r"🚨 HIGH RISK: Compromised package versions detected:")?;
    let medium_risk_content = Regex::new(r"⚠️  MEDIUM RISK: Suspicious content patterns:")?;
    let high_risk_crypto = Regex::new(r"🚨 HIGH RISK: Cryptocurrency theft patterns detected:")?;
    let medium_risk_crypto =
        Regex::new(r"⚠️  MEDIUM RISK: Potential cryptocurrency manipulation patterns:")?;
    let high_risk_trufflehog =
        Regex::new(r"🚨 HIGH RISK: Trufflehog/secret scanning activity detected:")?;
    let medium_risk_trufflehog =
        Regex::new(r"⚠️  MEDIUM RISK: Potentially suspicious secret scanning patterns:")?;

    // Pattern for file paths and messages
    let file_pattern = Regex::new(r"(?:Found in:|   - )(.+?)$")?;

    let lines: Vec<&str> = log_content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Parse workflow files
        if high_risk_workflow.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("🚨")
                && !lines[i].starts_with("⚠️")
            {
                if let Some(file_match) = file_pattern.captures(lines[i]) {
                    let file_path = file_match.get(1).unwrap().as_str().trim();
                    parsed.workflow_files.push(Finding {
                        file_path: normalize_path(file_path),
                        message: "Known malicious workflow filename".to_string(),
                        risk_level: "High".to_string(),
                        category: "workflow".to_string(),
                    });
                }
                i += 1;
            }
            continue;
        }

        // Parse compromised packages
        if high_risk_compromised.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("🚨")
                && !lines[i].starts_with("⚠️")
            {
                let current_line = lines[i].trim();

                // Look for "Package: name@version"
                if current_line.starts_with("- Package: ") {
                    let package_info = current_line.strip_prefix("- Package: ").unwrap_or("");
                    i += 1;

                    // Next line should be "Found in: path"
                    if i < lines.len() && lines[i].trim().starts_with("Found in: ") {
                        let file_path = lines[i].trim().strip_prefix("Found in: ").unwrap_or("");
                        parsed.compromised_found.push(Finding {
                            file_path: normalize_path(file_path),
                            message: package_info.to_string(),
                            risk_level: "High".to_string(),
                            category: "compromised_package".to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        // Parse suspicious content
        if medium_risk_content.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("🚨")
                && !lines[i].starts_with("⚠️")
            {
                let current_line = lines[i].trim();

                if current_line.starts_with("- Pattern: ") {
                    let pattern = current_line.strip_prefix("- Pattern: ").unwrap_or("");
                    i += 1;

                    if i < lines.len() && lines[i].trim().starts_with("Found in: ") {
                        let file_path = lines[i].trim().strip_prefix("Found in: ").unwrap_or("");
                        parsed.suspicious_content.push(Finding {
                            file_path: normalize_path(file_path),
                            message: pattern.to_string(),
                            risk_level: "Medium".to_string(),
                            category: "suspicious_content".to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        // Parse crypto patterns (HIGH RISK)
        if high_risk_crypto.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("NOTE:")
                && !lines[i].starts_with("⚠️")
            {
                let current_line = lines[i].trim();

                if current_line.starts_with("- ") && current_line.contains(":") {
                    let parts: Vec<&str> = current_line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let file_path = parts[0].trim_start_matches("- ");
                        let message = parts[1].trim();

                        let category = if message.contains("XMLHttpRequest") {
                            "crypto_xhr_hijack"
                        } else if message.contains("attacker wallet") {
                            "crypto_attacker_wallet"
                        } else {
                            "crypto_pattern"
                        };

                        parsed.crypto_patterns.push(Finding {
                            file_path: normalize_path(file_path),
                            message: message.to_string(),
                            risk_level: "High".to_string(),
                            category: category.to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        // Parse crypto patterns (MEDIUM RISK)
        if medium_risk_crypto.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("NOTE:")
                && !lines[i].starts_with("🚨")
            {
                let current_line = lines[i].trim();

                if current_line.starts_with("- ") && current_line.contains(":") {
                    let parts: Vec<&str> = current_line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        let file_path = parts[0].trim_start_matches("- ");
                        let message = parts[1].trim();

                        let category = if message.contains("Ethereum wallet") {
                            "crypto_wallet_pattern"
                        } else if message.contains("npmjs.help") {
                            "crypto_phishing"
                        } else {
                            "crypto_pattern"
                        };

                        parsed.crypto_patterns.push(Finding {
                            file_path: normalize_path(file_path),
                            message: message.to_string(),
                            risk_level: "Medium".to_string(),
                            category: category.to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        // Parse trufflehog (HIGH RISK)
        if high_risk_trufflehog.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("NOTE:")
                && !lines[i].starts_with("⚠️")
            {
                let current_line = lines[i].trim();

                if current_line.starts_with("- Activity: ") {
                    let activity = current_line.strip_prefix("- Activity: ").unwrap_or("");
                    i += 1;

                    if i < lines.len() && lines[i].trim().starts_with("Found in: ") {
                        let file_path = lines[i].trim().strip_prefix("Found in: ").unwrap_or("");

                        let category = if activity.contains("Trufflehog binary") {
                            "trufflehog_binary"
                        } else if activity.contains("Credential patterns") {
                            "credential_exfiltration"
                        } else {
                            "trufflehog_execution"
                        };

                        parsed.trufflehog.push(Finding {
                            file_path: normalize_path(file_path),
                            message: activity.to_string(),
                            risk_level: "High".to_string(),
                            category: category.to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        // Parse trufflehog (MEDIUM RISK)
        if medium_risk_trufflehog.is_match(line) {
            i += 1;
            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].starts_with("NOTE:")
                && !lines[i].starts_with("=")
            {
                let current_line = lines[i].trim();

                if current_line.starts_with("- Pattern: ") {
                    let pattern = current_line.strip_prefix("- Pattern: ").unwrap_or("");
                    i += 1;

                    if i < lines.len() && lines[i].trim().starts_with("Found in: ") {
                        let file_path = lines[i].trim().strip_prefix("Found in: ").unwrap_or("");

                        let category = if pattern.contains("credential scanning") {
                            "credential_patterns"
                        } else if pattern.contains("environment variable") {
                            "env_suspicious"
                        } else if pattern.contains("trufflehog references") {
                            "trufflehog_reference"
                        } else {
                            "trufflehog_pattern"
                        };

                        parsed.trufflehog.push(Finding {
                            file_path: normalize_path(file_path),
                            message: pattern.to_string(),
                            risk_level: "Medium".to_string(),
                            category: category.to_string(),
                        });
                    }
                }
                i += 1;
            }
            continue;
        }

        i += 1;
    }

    Ok(parsed)
}

fn normalize_path(path: &str) -> String {
    // Remove Windows UNC prefix and convert to standard path
    let cleaned = path.trim().replace("\\\\?\\", "").replace("\\\\", "\\");

    // Convert Windows paths to forward slashes for comparison
    PathBuf::from(cleaned).to_string_lossy().to_string()
}

fn compare_findings(bash_log: &ParsedBashLog, rust_json_path: &str) -> Result<()> {
    let rust_json_content =
        fs::read_to_string(rust_json_path).context("Failed to read Rust JSON file")?;

    let rust_data: serde_json::Value =
        serde_json::from_str(&rust_json_content).context("Failed to parse Rust JSON")?;

    println!("=== COMPARISON REPORT ===\n");

    // Compare each category
    compare_category(
        "Workflow Files",
        &bash_log.workflow_files,
        &rust_data["workflow_files"],
    );
    compare_category(
        "Malicious Hashes",
        &bash_log.malicious_hashes,
        &rust_data["malicious_hashes"],
    );
    compare_category(
        "Compromised Packages",
        &bash_log.compromised_found,
        &rust_data["compromised_found"],
    );
    compare_category(
        "Suspicious Content",
        &bash_log.suspicious_content,
        &rust_data["suspicious_content"],
    );
    compare_category(
        "Crypto Patterns",
        &bash_log.crypto_patterns,
        &rust_data["crypto_patterns"],
    );
    compare_category("Trufflehog", &bash_log.trufflehog, &rust_data["trufflehog"]);

    Ok(())
}

fn compare_category(name: &str, bash_findings: &[Finding], rust_findings: &serde_json::Value) {
    let empty_vec = vec![];
    let rust_array = rust_findings.as_array().unwrap_or(&empty_vec);

    println!("📊 {}", name);
    println!("   Bash: {} findings", bash_findings.len());
    println!("   Rust: {} findings", rust_array.len());

    if bash_findings.len() == rust_array.len() {
        println!("   ✅ COUNT MATCH\n");
    } else {
        println!("   ❌ COUNT MISMATCH\n");
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <bash_log_file> <rust_json_file>", args[0]);
        std::process::exit(1);
    }

    let bash_log_path = &args[1];
    let rust_json_path = &args[2];

    println!("📖 Parsing Bash log: {}", bash_log_path);
    let bash_log_content =
        fs::read_to_string(bash_log_path).context("Failed to read Bash log file")?;

    let parsed_bash = parse_bash_log(&bash_log_content)?;

    println!("📖 Reading Rust JSON: {}\n", rust_json_path);

    compare_findings(&parsed_bash, rust_json_path)?;

    Ok(())
}
