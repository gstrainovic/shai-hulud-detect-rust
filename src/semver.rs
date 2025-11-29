//! Semantic versioning support for package version matching
//! Port of `semver_bash` from the original shell script
//! Origin: <https://github.com/cloudflare/semver_bash/blob/6cc9ce10/semver.sh>

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;

lazy_static! {
    static ref SEMVER_RE: Regex =
        Regex::new(r"[^0-9]*([0-9]+)\.([0-9]+)\.([0-9]+)([0-9A-Za-z\-]*)").unwrap();
}

/// Parsed semantic version components
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub special: String,
}

impl SemVer {
    // Function: semverParseInto (as parse method)
    // Purpose: Parse semantic version string into major, minor, patch, and special components
    // Args: version_string
    // Modifies: None
    // Returns: Parsed SemVer struct or None
    // Origin: https://github.com/cloudflare/semver_bash/blob/6cc9ce10/semver.sh
    pub fn parse(version: &str) -> Option<Self> {
        let version = version.trim();
        let caps = SEMVER_RE.captures(version)?;

        let major = caps.get(1)?.as_str().parse().ok()?;
        let minor = caps.get(2)?.as_str().parse().ok()?;
        let patch = caps.get(3)?.as_str().parse().ok()?;
        let special = caps.get(4).map_or("", |m| m.as_str()).to_string();

        Some(SemVer {
            major,
            minor,
            patch,
            special,
        })
    }
}

// Function: semver_match
// Purpose: Check if version matches semver pattern with caret (^), tilde (~), wildcard (x/X), or exact matching
// Args: test_subject (version to test), test_pattern (pattern like "^1.0.0", "~1.1.0", "4.x", "1.2.X")
// Modifies: None
// Returns: true for match, false for no match (supports || for multi-pattern matching)
// Examples: "1.1.2" matches "^1.0.0", "~1.1.0", "1.x", "*" but not "^2.0.0" or "~1.2.0"
pub fn semver_match(test_subject: &str, test_pattern: &str) -> bool {
    // Always matches
    if test_pattern == "*" {
        return true;
    }

    // Destructure subject
    let subject = match SemVer::parse(test_subject) {
        Some(v) => v,
        None => return false,
    };

    // Handle multi-variant patterns (splits '||' into individual patterns)
    for pattern in test_pattern.split("||") {
        let pattern = pattern.trim();

        // Always matches
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('^') {
            // Caret: Major must match, minor.patch >= pattern
            let pattern_ver = match SemVer::parse(&pattern[1..]) {
                Some(v) => v,
                None => continue,
            };

            if subject.major != pattern_ver.major {
                continue;
            }

            match subject.minor.cmp(&pattern_ver.minor) {
                Ordering::Greater => return true,
                Ordering::Less => (),
                Ordering::Equal => {
                    if subject.patch >= pattern_ver.patch {
                        return true;
                    }
                }
            }
        } else if pattern.starts_with('~') {
            // Tilde: Major+minor must match, patch >= pattern
            let pattern_ver = match SemVer::parse(&pattern[1..]) {
                Some(v) => v,
                None => continue,
            };

            if subject.major == pattern_ver.major
                && subject.minor == pattern_ver.minor
                && subject.patch >= pattern_ver.patch
            {
                return true;
            }
        } else if pattern.contains('x') || pattern.contains('X') {
            // Wildcard pattern (4.x, 1.2.x, 4.X, 1.2.X, x.x.x, etc.)
            // Parse pattern components, handling 'x'/'X' wildcards specially
            let pattern_parts: Vec<&str> = pattern.split('.').collect();
            let subject_parts = [
                subject.major.to_string(),
                subject.minor.to_string(),
                subject.patch.to_string(),
            ];

            // Check each component, skip comparison for 'x'/'X' wildcards
            let mut matches = true;
            for i in 0..3 {
                if i < pattern_parts.len() && i < subject_parts.len() {
                    let pattern_part = pattern_parts[i];
                    let subject_part = &subject_parts[i];

                    // Skip wildcard components (both lowercase x and uppercase X)
                    if pattern_part == "x" || pattern_part == "X" {
                        continue;
                    }

                    // Extract numeric part (remove any non-numeric suffix)
                    let pattern_num = pattern_part
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .collect::<String>();
                    let subject_num = subject_part
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .collect::<String>();

                    // Compare numeric parts
                    if subject_num != pattern_num {
                        matches = false;
                        break;
                    }
                }
            }

            if matches {
                return true;
            }
        } else {
            // Exact match
            let pattern_ver = match SemVer::parse(pattern) {
                Some(v) => v,
                None => continue,
            };

            if subject.major == pattern_ver.major
                && subject.minor == pattern_ver.minor
                && subject.patch == pattern_ver.patch
                && subject.special == pattern_ver.special
            {
                return true;
            }
        }
    }

    // Fallthrough = no match
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_parse() {
        let v = SemVer::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.special, "");

        let v = SemVer::parse("1.2.3-beta").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.special, "-beta");

        let v = SemVer::parse("v2.0.0").unwrap();
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn test_caret_matching() {
        // ^1.0.0 should match 1.0.0, 1.0.1, 1.1.0, 1.9.9 but not 2.0.0
        assert!(semver_match("1.0.0", "^1.0.0"));
        assert!(semver_match("1.0.1", "^1.0.0"));
        assert!(semver_match("1.1.0", "^1.0.0"));
        assert!(semver_match("1.9.9", "^1.0.0"));
        assert!(!semver_match("2.0.0", "^1.0.0"));
        assert!(!semver_match("0.9.9", "^1.0.0"));
    }

    #[test]
    fn test_tilde_matching() {
        // ~1.2.0 should match 1.2.0, 1.2.1, 1.2.9 but not 1.3.0
        assert!(semver_match("1.2.0", "~1.2.0"));
        assert!(semver_match("1.2.1", "~1.2.0"));
        assert!(semver_match("1.2.9", "~1.2.0"));
        assert!(!semver_match("1.3.0", "~1.2.0"));
        assert!(!semver_match("1.1.9", "~1.2.0"));
    }

    #[test]
    fn test_exact_matching() {
        assert!(semver_match("1.2.3", "1.2.3"));
        assert!(!semver_match("1.2.4", "1.2.3"));
        assert!(!semver_match("1.3.3", "1.2.3"));
    }

    #[test]
    fn test_wildcard() {
        assert!(semver_match("1.2.3", "*"));
        assert!(semver_match("99.99.99", "*"));
    }

    #[test]
    fn test_or_operator() {
        assert!(semver_match("1.0.0", "^1.0.0 || ^2.0.0"));
        assert!(semver_match("2.0.0", "^1.0.0 || ^2.0.0"));
        assert!(!semver_match("3.0.0", "^1.0.0 || ^2.0.0"));
    }

    #[test]
    fn test_wildcard_patterns() {
        // Test lowercase x wildcards
        assert!(semver_match("4.0.0", "4.x"));
        assert!(semver_match("4.1.2", "4.x"));
        assert!(semver_match("4.99.99", "4.x"));
        assert!(!semver_match("5.0.0", "4.x"));
        assert!(!semver_match("3.99.99", "4.x"));

        // Test uppercase X wildcards (case insensitive)
        assert!(semver_match("3.0.0", "3.X"));
        assert!(semver_match("3.5.7", "3.X"));
        assert!(!semver_match("4.0.0", "3.X"));

        // Test major.minor.x pattern
        assert!(semver_match("1.2.0", "1.2.x"));
        assert!(semver_match("1.2.5", "1.2.x"));
        assert!(semver_match("1.2.99", "1.2.x"));
        assert!(!semver_match("1.3.0", "1.2.x"));
        assert!(!semver_match("2.2.0", "1.2.x"));

        // Test x.x.x (matches everything)
        assert!(semver_match("1.2.3", "x.x.x"));
        assert!(semver_match("99.88.77", "x.x.x"));

        // Test mixed case
        assert!(semver_match("2.3.4", "2.X.x"));
    }
}
