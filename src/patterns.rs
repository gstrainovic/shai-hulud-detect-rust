use regex::Regex;
use serde::{Deserialize, Serialize};

/// Risk levels for detected patterns
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
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
        // Webhook.site patterns (high risk exfiltration)
        patterns.push(Pattern {
            name: "webhook_site".to_string(),
            regex: Regex::new(r"webhook\.site").unwrap(),
            description: "webhook.site reference detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // Known malicious webhook endpoint
        patterns.push(Pattern {
            name: "malicious_webhook_endpoint".to_string(),
            regex: Regex::new(r"bb8ca5f6-4175-45d2-b042-fc9ebb8170b7").unwrap(),
            description: "Known malicious webhook endpoint".to_string(),
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
            description: "Known attacker wallet address detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // Phishing domain npmjs.help
        patterns.push(Pattern {
            name: "npmjs_help_domain".to_string(),
            regex: Regex::new(r"npmjs\.help").unwrap(),
            description: "Phishing domain npmjs.help detected".to_string(),
            risk_level: RiskLevel::High,
        });

        // TruffleHog binary references
        patterns.push(Pattern {
            name: "trufflehog_binary".to_string(),
            regex: Regex::new(r"trufflehog|TruffleHog").unwrap(),
            description: "TruffleHog binary reference detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Environment variable scanning patterns
        patterns.push(Pattern {
            name: "env_var_scanning".to_string(),
            regex: Regex::new(r"(AWS_ACCESS_KEY|GITHUB_TOKEN|NPM_TOKEN)").unwrap(),
            description: "Credential scanning patterns detected".to_string(),
            risk_level: RiskLevel::Medium,
        });

        // Process.env access patterns
        patterns.push(Pattern {
            name: "process_env_access".to_string(),
            regex: Regex::new(r"process\.env\[").unwrap(),
            description: "Suspicious environment variable access".to_string(),
            risk_level: RiskLevel::Low,
        });

        // Ethereum wallet addresses (general pattern)
        patterns.push(Pattern {
            name: "ethereum_address_patterns".to_string(),
            regex: Regex::new(r"0x[a-fA-F0-9]{40}").unwrap(),
            description: "Ethereum wallet address patterns detected".to_string(),
            risk_level: RiskLevel::Medium,
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

    /// Check content against all patterns
    pub fn check_content(&self, content: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for pattern in &self.patterns {
            if pattern.regex.is_match(content) {
                matches.push(PatternMatch {
                    pattern_name: pattern.name.clone(),
                    description: pattern.description.clone(),
                    risk_level: pattern.risk_level.clone(),
                });
            }
        }

        matches
    }
}
