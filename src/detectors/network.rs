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
        
        // BASH LINE 1102, 1116, 1162, 1219: Skip vendor/library files AND node_modules
        if path_str.contains("/vendor/") || path_str.contains("\\vendor\\") 
           || path_str.contains("/node_modules/") || path_str.contains("\\node_modules\\") {
            continue;
        }

        if let Ok(content) = fs::read_to_string(entry.path()) {
            // BASH LINE 1102-1112: Check for hardcoded IP addresses (skip vendor/node_modules)
            if let Some(captures) = IP_PATTERN.find_iter(&content).next() {
                let ip = captures.as_str();
                // BASH LINE 1108: Skip common safe IPs
                if ip != "127.0.0.1" && ip != "0.0.0.0" && ip != "255.255.255.255" {
                    let ips: Vec<_> = IP_PATTERN.find_iter(&content).take(3).map(|m| m.as_str()).collect();
                    let ips_str = ips.join(" ");
                    
                    // BASH LINE 1109-1112: Check if minified
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

            // BASH LINE 1116-1157: Check for suspicious domains (skip package-lock, yarn.lock, vendor, node_modules)
            if !path_str.ends_with("package-lock.json") && !path_str.ends_with("yarn.lock") {
                for domain in SUSPICIOUS_DOMAINS {
                    if content.contains(domain) {
                        // BASH LINE 1120-1122: Make sure it's not just a comment
                        let lines: Vec<&str> = content.lines()
                            .filter(|line| {
                                line.contains(domain) && 
                                !line.trim().starts_with('#') && 
                                !line.trim().starts_with("//")
                            })
                            .take(1)  // BASH takes only first match
                            .collect();

                        if let Some(first_line) = lines.first() {
                            // BASH LINE 1131-1154: Format snippet based on line length
                            let snippet = if path_str.ends_with(".min.js") || first_line.len() > 150 {
                                // Extract around domain (BASH LINE 1134)
                                if let Some(pos) = first_line.find(domain) {
                                    let start = pos.saturating_sub(20);
                                    let end = (pos + domain.len() + 20).min(first_line.len());
                                    format!("...{}...", &first_line[start..end])
                                } else {
                                    format!("...{}...", &first_line[..80.min(first_line.len())])
                                }
                            } else {
                                // BASH LINE 1147: Cut to 80 chars
                                if first_line.len() > 80 {
                                    format!("{}...", &first_line[..80])
                                } else {
                                    first_line.to_string()
                                }
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

            // BASH LINE 1162-1186: Check for base64-encoded URLs (skip vendor/node_modules)
            if (content.contains("atob(") || (content.contains("base64") && content.contains("decode"))) {
                // BASH LINE 1166: Get line number for snippet
                let has_atob = content.contains("atob(");
                let snippet = if path_str.ends_with(".min.js") || content.lines().next().map(|l| l.len()).unwrap_or(0) > 500 {
                    // BASH LINE 1171-1179: Extract around atob
                    if has_atob {
                        if let Some(line) = content.lines().find(|l| l.contains("atob(")) {
                            if let Some(pos) = line.find("atob") {
                                let start = pos.saturating_sub(30);
                                let end = (pos + 35).min(line.len());
                                format!("...{}...", &line[start..end])
                            } else {
                                "Base64 decoding detected".to_string()
                            }
                        } else {
                            "Base64 decoding detected".to_string()
                        }
                    } else {
                        "Base64 decoding detected".to_string()
                    }
                } else {
                    // BASH LINE 1182-1183
                    if let Some(line) = content.lines().find(|l| l.contains("atob") || l.contains("base64")) {
                        if line.len() > 80 {
                            format!("{}...", &line[..80])
                        } else {
                            line.to_string()
                        }
                    } else {
                        "Base64 decoding detected".to_string()
                    }
                };
                
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    format!("Base64 decoding at line: {}", snippet),
                    RiskLevel::Medium,
                    "network_exfiltration",
                ));
            }

            // BASH LINE 1189-1191: Check for DNS-over-HTTPS patterns
            if content.contains("dns-query") || content.contains("application/dns-message") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "DNS-over-HTTPS pattern detected".to_string(),
                    RiskLevel::Medium,
                    "network_exfiltration",
                ));
            }

            // BASH LINE 1194-1209: Check for WebSocket connections to unusual endpoints
            if content.contains("ws://") || content.contains("wss://") {
                // BASH extracts all ws:// endpoints first, then filters
                let ws_regex = Regex::new(r#"wss?://[^\s"'']+"#).unwrap();
                for cap in ws_regex.find_iter(&content) {
                    let endpoint = cap.as_str();
                    // BASH LINE 1202: Skip localhost/127.0.0.1
                    if !endpoint.contains("localhost") && !endpoint.contains("127.0.0.1") {
                        findings.push(Finding::new(
                            entry.path().to_path_buf(),
                            format!("WebSocket connection to external endpoint: {}", endpoint),
                            RiskLevel::Medium,
                            "network_exfiltration",
                        ));
                    }
                }
            }

            // BASH LINE 1212-1214: Check for suspicious HTTP headers
            if content.contains("X-Exfiltrate") || content.contains("X-Data-Export") || content.contains("X-Credential") {
                findings.push(Finding::new(
                    entry.path().to_path_buf(),
                    "Suspicious HTTP headers detected".to_string(),
                    RiskLevel::Medium,
                    "network_exfiltration",
                ));
            }

            // BASH LINE 1217-1232: Check for btoa near network operations (skip vendor/node_modules/min.js)
            if content.contains("btoa(") && !path_str.ends_with(".min.js") {
                // BASH LINE 1220: Check if near network operations
                let has_network = content.contains("fetch") || content.contains("XMLHttpRequest") || content.contains("axios");
                if has_network {
                    // BASH LINE 1222-1223: Skip if Authorization/Basic/Bearer
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
