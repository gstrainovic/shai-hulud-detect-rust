// Network Exfiltration Detector
// Rust port of: check_network_exfiltration()

use crate::detectors::{Finding, RiskLevel};
use regex::Regex;
use lazy_static::lazy_static;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

lazy_static! {
    static ref IP_PATTERN: Regex = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap();
}

// Suspicious domains and patterns beyond webhook.site
const SUSPICIOUS_DOMAINS: &[&str] = &[
    "pastebin.com", "hastebin.com", "ix.io", "0x0.st", "transfer.sh",
    "file.io", "anonfiles.com", "mega.nz", "dropbox.com/s/",
    "discord.com/api/webhooks", "telegram.org", "t.me",
    "ngrok.io", "localtunnel.me", "serveo.net",
    "requestbin.com", "webhook.site", "beeceptor.com",
    "pipedream.com", "zapier.com/hooks",
];

// Function: check_network_exfiltration
// Purpose: Detect network exfiltration patterns including suspicious domains and IPs
// Args: $1 = scan_dir (directory to scan)
// Modifies: NETWORK_EXFILTRATION_WARNINGS (global array)
// Returns: Populates array with hardcoded IPs and suspicious domains
pub fn check_network_exfiltration<P: AsRef<Path>>(scan_dir: P) -> Vec<Finding> {
    let mut findings = Vec::new();
    let extensions = &["js", "ts", "json", "mjs"];

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
        let path_str = entry.path().to_string_lossy();
        
        // Skip vendor/library files to reduce false positives
        if path_str.contains("/vendor/") || path_str.contains("\\vendor\\") 
           || path_str.contains("/node_modules/") || path_str.contains("\\node_modules\\") {
            continue;
        }

        if let Ok(content) = fs::read_to_string(entry.path()) {
            // Check for hardcoded IP addresses
            if let Some(captures) = IP_PATTERN.find_iter(&content).next() {
                let ip = captures.as_str();
                // Skip common safe IPs
                if ip != "127.0.0.1" && ip != "0.0.0.0" && ip != "255.255.255.255" {
                    let ips: Vec<_> = IP_PATTERN.find_iter(&content).take(3).map(|m| m.as_str()).collect();
                    let ips_str = ips.join(" ");
                    
                    if path_str.ends_with(".min.js") {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            format!("Hardcoded IP addresses found (minified file): {}", ips_str),
                            RiskLevel::Medium,
                            "network_exfiltration",
                        ));
                    } else {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            format!("Hardcoded IP addresses found: {}", ips_str),
                            RiskLevel::Medium,
                            "network_exfiltration",
                        ));
                    }
                }
            }

            // Check for suspicious domains (skip package-lock and yarn.lock)
            if !path_str.ends_with("package-lock.json") && !path_str.ends_with("yarn.lock") {
                for domain in SUSPICIOUS_DOMAINS {
                    if content.contains(domain) {
                        // Make sure it's not just a comment or documentation
                        let lines: Vec<&str> = content.lines()
                            .filter(|line| {
                                line.contains(domain) && 
                                !line.trim().starts_with('#') && 
                                !line.trim().starts_with("//")
                            })
                            .collect();

                        if let Some(first_line) = lines.first() {
                            let snippet = if first_line.len() > 80 {
                                format!("{}...", &first_line[..80])
                            } else {
                                first_line.to_string()
                            };

                            findings.push(Finding::new(
                                entry.path().to_path_buf(),
                                format!("Suspicious domain found: {}: {}", domain, snippet),
                                RiskLevel::Medium,
                                "network_exfiltration",
                            ));
                        }
                    }
                }
            }

            // Check for base64-encoded URLs
            if (content.contains("atob(") || content.contains("base64") && content.contains("decode")) 
               && !path_str.contains("/vendor/") && !path_str.contains("\\vendor\\") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Base64 decoding detected".to_string(),
                    RiskLevel::Low,
                    "network_exfiltration",
                ));
            }

            // Check for DNS-over-HTTPS patterns
            if content.contains("dns-query") || content.contains("application/dns-message") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "DNS-over-HTTPS pattern detected".to_string(),
                    RiskLevel::Medium,
                    "network_exfiltration",
                ));
            }

            // Check for WebSocket connections to unusual endpoints
            if content.contains("ws://") || content.contains("wss://") {
                let ws_endpoints: Vec<_> = content.lines()
                    .filter(|line| line.contains("ws://") || line.contains("wss://"))
                    .filter(|line| !line.contains("localhost") && !line.contains("127.0.0.1"))
                    .collect();

                for endpoint in ws_endpoints {
                    let snippet = if endpoint.len() > 80 {
                        format!("{}...", &endpoint[..80])
                    } else {
                        endpoint.to_string()
                    };

                    findings.push(Finding::new(
                        entry.path().to_path_buf(),
                        format!("WebSocket connection to external endpoint: {}", snippet),
                        RiskLevel::Medium,
                        "network_exfiltration",
                    ));
                }
            }

            // Check for suspicious HTTP headers
            if content.contains("X-Exfiltrate") || content.contains("X-Data-Export") || content.contains("X-Credential") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Suspicious HTTP headers detected".to_string(),
                    RiskLevel::High,
                    "network_exfiltration",
                ));
            }

            // Check for data encoding that might hide exfiltration (but be more selective)
            if content.contains("btoa(") && !path_str.contains("/vendor/") && !path_str.contains("\\vendor\\") && !path_str.ends_with(".min.js") {
                // Check if it's near network operations
                if content.contains("fetch") || content.contains("XMLHttpRequest") || content.contains("axios") {
                    // Additional check - make sure it's not just legitimate authentication
                    if !content.contains("Authorization:") && !content.contains("Basic ") && !content.contains("Bearer ") {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            "Suspicious base64 encoding near network operation".to_string(),
                            RiskLevel::Medium,
                            "network_exfiltration",
                        ));
                    }
                }
            }
        }
    }

    findings
}
