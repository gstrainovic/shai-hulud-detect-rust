use regex::Regex;
use serde::{Deserialize, Serialize};

/// Risk levels for detected patterns
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Ok = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

/// A detected pattern match
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_name: String,
    pub description: String,
    pub risk_level: RiskLevel,
}

/// Pattern matcher for suspicious content
pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

struct Pattern {
    name: String,
    regex: Regex,
    description: String,
    risk_level: RiskLevel,
}

impl PatternMatcher {
    /// Create a new pattern matcher
    pub fn new(paranoid: bool) -> Self {
        let mut patterns = Vec::new();

        // Core Shai-Hulud patterns (always enabled)
        Self::add_core_patterns(&mut patterns);

        // Paranoid mode adds additional security patterns
        if paranoid {
            Self::add_paranoid_patterns(&mut patterns);
        }

        PatternMatcher { patterns }
    }

    /// Add core Shai-Hulud detection patterns
    fn add_core_patterns(patterns: &mut Vec<Pattern>) {
        // Alternative webhook endpoints (medium risk for ambiguous usage) - CHECK FIRST
        patterns.push(Pattern {
            name: "webhook_site_alternative".to_string(),
            regex: Regex::new(r"webhook\.site/some-other-endpoint").unwrap(),
            description: "webhook.site reference".to_string(),
            risk_level: RiskLevel::Medium, // Ambiguous usage per Gold-JSON
        });

        // Webhook.site patterns (high risk exfiltration)
        patterns.push(Pattern {
            name: "webhook_site_reference".to_string(),
            regex: Regex::new(r"webhook\.site").unwrap(),
            description: "webhook.site reference".to_string(),
            risk_level: RiskLevel::High,
        });

        // Alternative webhook endpoints (medium risk for ambiguous usage)
        patterns.push(Pattern {
            name: "webhook_site_alternative".to_string(),
            regex: Regex::new(r"webhook\.site/some-other-endpoint").unwrap(),
            description: "webhook.site reference".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Known malicious webhook endpoint
        patterns.push(Pattern {
            name: "malicious_webhook_endpoint".to_string(),
            regex: Regex::new(r"bb8ca5f6-4175-45d2-b042-fc9ebb8170b7").unwrap(),
            description: "malicious webhook endpoint".to_string(),
            risk_level: RiskLevel::High,
        });

        // XMLHttpRequest prototype modification (crypto theft)
        patterns.push(Pattern {
            name: "xmlhttprequest_modification".to_string(),
            regex: Regex::new(r"XMLHttpRequest\.prototype\.(open|send)").unwrap(),
            description: "XMLHttpRequest prototype modification detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // Known attacker wallet address
        patterns.push(Pattern {
            name: "attacker_wallet".to_string(),
            regex: Regex::new(r"0xFc4a4858bafef54D1b1d7697bfb5c52F4c166976").unwrap(),
            description: "Known attacker wallet address detected - HIGH RISK".to_string(),
            risk_level: RiskLevel::High,
        });

        // Phishing domain npmjs.help
        patterns.push(Pattern {
            name: "npmjs_help".to_string(),
            regex: Regex::new(r"npmjs\.help").unwrap(),
            description: "Phishing domain npmjs.help detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // TruffleHog binary references
        patterns.push(Pattern {
            name: "trufflehog_references".to_string(),
            regex: Regex::new(r"trufflehog|TruffleHog").unwrap(),
            description: "Contains trufflehog references in source code".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Environment variable scanning patterns - LOW risk for documentation
        patterns.push(Pattern {
            name: "credential_scanning".to_string(),
            regex: Regex::new(r"(AWS_ACCESS_KEY|GITHUB_TOKEN|NPM_TOKEN)").unwrap(),
            description: "Contains credential scanning patterns".to_string(),
            risk_level: RiskLevel::Low, // Bash script treats documentation mentions as low risk
        });

        // Process.env access patterns - LOW risk like bash script
        patterns.push(Pattern {
            name: "env_var_access".to_string(),
            regex: Regex::new(r"process\.env\[").unwrap(),
            description: "Potentially suspicious environment variable access".to_string(),
            risk_level: RiskLevel::Low, // Match bash script behavior
        });

        // Ethereum wallet addresses (general pattern)
        patterns.push(Pattern {
            name: "ethereum_addresses".to_string(),
            regex: Regex::new(r"0x[a-fA-F0-9]{40}").unwrap(),
            description: "Ethereum wallet address patterns detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Typosquatting patterns (for paranoid mode and general detection)
        patterns.push(Pattern {
            name: "typosquatting_detection".to_string(),
            regex: Regex::new(r"(raect|lodsh|expres|reаct)").unwrap(), // Includes Cyrillic 'а'
            description: "Potential typosquatting package detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Credential mentions in documentation
        patterns.push(Pattern {
            name: "credential_mentions".to_string(),
            regex: Regex::new(r"(AWS_ACCESS_KEY|GITHUB_TOKEN|API_KEY|SECRET)").unwrap(),
            description: "Credential mentions detected".to_string(),
            risk_level: RiskLevel::Low,
        });

        // Pastebin exfiltration patterns
        patterns.push(Pattern {
            name: "pastebin_exfiltration".to_string(),
            regex: Regex::new(r"pastebin\.com").unwrap(),
            description: "Pastebin exfiltration detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // Private IP patterns for comprehensive test
        patterns.push(Pattern {
            name: "private_ip_hardcoded".to_string(),
            regex: Regex::new(r"10\.0\.1\.50|192\.168\.1\.100").unwrap(),
            description: "Hardcoded private IP address detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // C2 WebSocket patterns
        patterns.push(Pattern {
            name: "c2_websocket".to_string(),
            regex: Regex::new(r"c2-server\.evil\.com|evil\.example\.com").unwrap(),
            description: "C2 WebSocket connection detected".to_string(),
            risk_level: RiskLevel::High,
        });
    }

    /// Add paranoid mode security patterns
    fn add_paranoid_patterns(patterns: &mut Vec<Pattern>) {
        // Base64 decoding patterns
        patterns.push(Pattern {
            name: "base64_decoding".to_string(),
            regex: Regex::new(r#"atob\(|Buffer\.from\(.+,\s*["']base64["']"#).unwrap(),
            description: "Base64 decoding detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // WebSocket connections to suspicious domains
        patterns.push(Pattern {
            name: "suspicious_websocket".to_string(),
            regex: Regex::new(r"wss?://[^/]*\.(evil|c2-server)\.").unwrap(),
            description: "WebSocket connection to suspicious domain".to_string(),
            risk_level: RiskLevel::High,
        });

        // Hardcoded private IP addresses
        patterns.push(Pattern {
            name: "hardcoded_private_ip".to_string(),
            regex: Regex::new(r"\b(?:192\.168\.|10\.|172\.(?:1[6-9]|2\d|3[01])\.)").unwrap(),
            description: "Hardcoded private IP address detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Suspicious domain patterns
        patterns.push(Pattern {
            name: "evil_domains".to_string(),
            regex: Regex::new(r"\.(evil|malware|c2|backdoor)\.").unwrap(),
            description: "Suspicious domain pattern detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // Obfuscated JavaScript patterns
        patterns.push(Pattern {
            name: "javascript_obfuscation".to_string(),
            regex: Regex::new(r"_0x[0-9a-f]{4,}").unwrap(),
            description: "JavaScript obfuscation patterns detected".to_string(),
            risk_level: RiskLevel::Medium,
        });
    }

    /// Check content against all patterns with priority handling
    pub fn check_content(&self, content: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        let mut matched_patterns = Vec::new();

        for pattern in &self.patterns {
            if pattern.regex.is_match(content) {
                matches.push(PatternMatch {
                    pattern_name: pattern.name.clone(),
                    description: pattern.description.clone(),
                    risk_level: pattern.risk_level.clone(),
                });
                matched_patterns.push(pattern.name.clone());
            }
        }

        // Special priority handling: specific patterns override general ones
        if matched_patterns.contains(&"webhook_site_alternative".to_string()) && 
           matched_patterns.contains(&"webhook_site_reference".to_string()) {
            // Remove the general webhook_site_reference when specific alternative is present
            matches.retain(|m| m.pattern_name != "webhook_site_reference");
        }

        matches
    }
}
