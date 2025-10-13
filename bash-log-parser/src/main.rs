use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use colored::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{not_line_ending, space0},
    sequence::{preceded, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(ClapParser)]
#[command(name = "bash-log-parser")]
#[command(about = "Parse Bash scanner logs using nom parser combinators")]
struct Cli {
    /// Bash scanner log file (.log)
    bash_log: PathBuf,
    /// Rust scanner JSON file (.json)
    rust_json: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Finding {
    file_path: String,
    message: String,
    risk_level: String,
}

impl Finding {
    fn normalize_path(path: &str) -> String {
        path.replace("\\\\?\\", "")
            .replace("\\", "/")
            .to_lowercase()
            .trim_start_matches("/c/")
            .trim_start_matches("c:/")
            .trim_start_matches("c:")
            .to_string()
    }

    fn new(file_path: &str, message: &str, risk_level: &str) -> Self {
        Self {
            file_path: Self::normalize_path(file_path),
            message: message.trim().to_string(),
            risk_level: risk_level.to_uppercase(),
        }
    }

    fn fingerprint(&self) -> String {
        format!(
            "{}|{}|{}",
            self.file_path,
            self.message.to_lowercase(),
            self.risk_level
        )
    }
}

// Strip ANSI escape sequences and emojis
fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1B' {
            // ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else if ch.is_ascii() || ch.is_ascii_whitespace() {
            // Only keep ASCII characters
            result.push(ch);
        }
        // Skip emojis and other non-ASCII characters
    }
    result
}

// Nom parsers
fn risk_marker(input: &str) -> IResult<&str, &str> {
    // Skip any non-ASCII characters (emojis) before the marker
    let trimmed = input.trim_start_matches(|c: char| !c.is_ascii());
    alt((tag("HIGH RISK:"), tag("MEDIUM RISK:"), tag("LOW RISK:")))(trimmed)
}

fn extract_risk(marker: &str) -> &str {
    if marker.contains("HIGH") {
        "HIGH"
    } else if marker.contains("MEDIUM") {
        "MEDIUM"
    } else {
        "LOW"
    }
}

// Parse "- Pattern: XXX" or "- Package: XXX" or "- Activity: XXX"
// Note: Bash log has leading spaces (   -) so we need to handle them
fn parse_label_value(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?; // Handle leading spaces
    let (input, _) = tag("- ")(input)?;
    let (input, _) = alt((
        tag("Pattern:"),
        tag("Package:"),
        tag("Activity:"),
        tag("Issue:"),
        tag("Warning:"), // Add Warning for paranoid mode
    ))(input)?;
    let (input, _) = space0(input)?;
    let (input, value) = not_line_ending(input)?;
    Ok((input, value.trim()))
}

// Parse "     Found in: /path"
fn parse_found_in(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("Found in:")(input)?;
    let (input, _) = space0(input)?;
    let (input, path) = not_line_ending(input)?;
    Ok((input, path.trim()))
}

// Parse "- /path:message"
fn parse_path_colon_message(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = space0(input)?; // Handle leading spaces
    let (input, _) = tag("- ")(input)?;
    let (input, path) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, message) = not_line_ending(input)?;
    Ok((input, (path.trim(), message.trim())))
}

// Parse "- /path/to/file" (simple path for workflows)
fn parse_simple_path(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?; // Handle leading spaces
    preceded(
        tuple((tag("- "), space0)),
        take_while1(|c: char| c != '\n' && c != '\r'),
    )(input)
}

fn parse_bash_log(content: &str) -> Result<Vec<Finding>> {
    let clean = strip_ansi(content);
    let lines: Vec<&str> = clean.lines().collect();

    let mut findings = Vec::new();
    let mut current_risk = None;
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check for risk marker
        if line.contains("RISK:") {
            if let Ok((_, marker)) = risk_marker(line) {
                current_risk = Some(extract_risk(marker));
            }
        }

        if let Some(risk) = current_risk {
            // Try: "- Label: value" + "Found in: path"
            if let Ok((_, value)) = parse_label_value(line) {
                if i + 1 < lines.len() {
                    if let Ok((_, path)) = parse_found_in(lines[i + 1].trim()) {
                        findings.push(Finding::new(path, value, risk));
                        i += 2;
                        continue;
                    }
                }
            }

            // Try: "- /path:message"
            if let Ok((_, (path, message))) = parse_path_colon_message(line) {
                findings.push(Finding::new(path, message, risk));
                i += 1;
                continue;
            }

            // Try: "- /path" (workflows)
            if let Ok((_, path)) = parse_simple_path(line) {
                if !path.contains(':') && path.starts_with('/') {
                    findings.push(Finding::new(
                        path,
                        "Known malicious workflow filename",
                        risk,
                    ));
                }
            }
        }

        i += 1;
    }

    Ok(findings)
}

fn load_rust_json(path: &Path) -> Result<Vec<Finding>> {
    let content = fs::read_to_string(path)?;
    let data: serde_json::Value = serde_json::from_str(&content)?;

    let mut findings = Vec::new();
    let categories = [
        "workflow_files",
        "malicious_hashes",
        "compromised_found",
        "suspicious_found",
        "suspicious_content",
        "crypto_patterns",
        "trufflehog",
        "git_branches",
        "postinstall_hooks",
        "shai_hulud_repos",
        "namespace_warnings",
        "integrity_issues",
        "typosquatting_warnings",
        "network_exfiltration_warnings",
        "lockfile_safe_versions",
    ];

    for category in &categories {
        if let Some(items) = data.get(category).and_then(|v| v.as_array()) {
            for item in items {
                if let (Some(path), Some(msg), Some(risk)) = (
                    item.get("file_path").and_then(|v| v.as_str()),
                    item.get("message").and_then(|v| v.as_str()),
                    item.get("risk_level").and_then(|v| v.as_str()),
                ) {
                    findings.push(Finding::new(path, msg, risk));
                }
            }
        }
    }

    Ok(findings)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{}", "═".repeat(70).bright_blue());
    println!(
        "{}",
        "  Bash Log Parser (nom combinators)".bright_cyan().bold()
    );
    println!("{}", "═".repeat(70).bright_blue());
    println!();

    let bash_content = fs::read_to_string(&cli.bash_log).context("Failed to read Bash log file")?;
    let bash_findings = parse_bash_log(&bash_content).context("Failed to parse Bash log")?;

    let rust_findings = load_rust_json(&cli.rust_json).context("Failed to load Rust JSON")?;

    println!("{}", "📊 Findings:".bright_yellow().bold());
    println!(
        "  Bash: {}",
        bash_findings.len().to_string().bright_white().bold()
    );
    println!(
        "  Rust: {}",
        rust_findings.len().to_string().bright_white().bold()
    );
    println!();

    let bash_set: HashSet<_> = bash_findings.iter().map(|f| f.fingerprint()).collect();
    let rust_set: HashSet<_> = rust_findings.iter().map(|f| f.fingerprint()).collect();

    let matches = bash_set.intersection(&rust_set).count();
    let missing = bash_set.difference(&rust_set).count();
    let extra = rust_set.difference(&bash_set).count();

    println!("{}", "🔍 Comparison:".bright_yellow().bold());
    println!(
        "  {} {}",
        "✓ Matches:".bright_green(),
        matches.to_string().bright_white().bold()
    );
    println!(
        "  {} {}",
        "✗ Missing in Rust:".bright_red(),
        missing.to_string().bright_white().bold()
    );
    println!(
        "  {} {}",
        "⊕ Extra in Rust:".bright_cyan(),
        extra.to_string().bright_white().bold()
    );
    println!();

    if missing == 0 && extra == 0 {
        println!("{}", "🎉 PERFECT MATCH!".bright_green().bold());
        std::process::exit(0);
    } else {
        println!("{}", "⚠️  Differences detected".bright_yellow());

        if missing > 0 {
            println!();
            println!("{}", "Missing in Rust:".bright_red().bold());
            for fp in bash_set.difference(&rust_set).take(5) {
                println!("  - {}", fp);
            }
            if missing > 5 {
                println!("  ... and {} more", missing - 5);
            }
        }

        if extra > 0 {
            println!();
            println!("{}", "Extra in Rust:".bright_cyan().bold());
            for fp in rust_set.difference(&bash_set).take(5) {
                println!("  + {}", fp);
            }
            if extra > 5 {
                println!("  ... and {} more", extra - 5);
            }
        }

        std::process::exit(1);
    }
}
