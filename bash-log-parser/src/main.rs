use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use colored::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{preceded, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

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

// Strip ANSI escape sequences while preserving original text
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
        } else {
            result.push(ch);
        }
    }
    result
}

// Nom parsers
fn risk_marker(input: &str) -> IResult<&str, &str> {
    // Normalize input: remove any leading non-ASCII characters (emojis), then trim leading whitespace
    let s = input
        .trim_start_matches(|c: char| !c.is_ascii())
        .trim_start();
    alt((
        tag("HIGH RISK:"),
        tag("MEDIUM RISK:"),
        tag("LOW RISK:"),
        // PARANOID mode formats
        tag("HIGH RISK (PARANOID):"),
        tag("MEDIUM RISK (PARANOID):"),
        tag("LOW RISK (PARANOID):"),
        // Also match informational LOW risk sections (with any text after)
        map(preceded(tag("LOW RISK FINDINGS"), take_until(":")), |_| {
            "LOW RISK:"
        }),
    ))(s)
}

// Parse PARANOID warning: "- Warning: message"
fn parse_paranoid_warning(input: &str) -> IResult<&str, &str> {
    preceded(
        tuple((tag("- Warning: "), space0)),
        take_while1(|c: char| c != '\n' && c != '\r'),
    )(input)
}

// Parse PARANOID path: "Found in: path"
fn parse_paranoid_path(input: &str) -> IResult<&str, &str> {
    preceded(
        tuple((tag("Found in: "), space0)),
        take_while1(|c: char| c != '\n' && c != '\r'),
    )(input)
}

fn parse_paranoid_header(input: &str) -> IResult<&str, &str> {
    let input = input.trim_start_matches(|c: char| !c.is_ascii());
    let (input, _) = space0(input)?;
    let (input, risk) = alt((tag("HIGH"), tag("MEDIUM"), tag("LOW")))(input)?;
    let (input, _) = tag(" RISK (PARANOID):")(input)?;
    // Consume optional description text after the colon
    let (input, _) = opt(not_line_ending)(input)?;
    Ok((input, risk))
}

fn parse_paranoid_continuation(input: &str) -> IResult<&str, String> {
    let (rest, line) = not_line_ending(input)?;
    let trimmed = line.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('-')
        || trimmed.starts_with("NOTE:")
        || trimmed.contains("RISK")
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (rest, _) = opt(line_ending)(rest)?;
    Ok((rest, trimmed.to_string()))
}

fn normalize_paranoid_message(message: &str) -> String {
    let mut msg = message.trim().to_string();

    if msg.ends_with("...") {
        msg.truncate(msg.len().saturating_sub(3));
        msg = msg.trim_end().to_string();
    }

    if msg.starts_with("Suspicious base64 encoding near network operation") {
        return "Suspicious base64 encoding near network operation".to_string();
    }

    if msg.starts_with("Base64 decoding at line ") {
        if let Some(idx) = msg.find("line ") {
            let rest = &msg[idx + 5..];
            if let Some(colon_pos) = rest.find(':') {
                if rest[..colon_pos].chars().all(|c| c.is_ascii_digit()) {
                    let prefix = &msg[..idx];
                    let suffix = &rest[colon_pos..];
                    msg = format!("{}line{}", prefix, suffix);
                }
            }
        }
    }

    msg
}

fn parse_paranoid_entry(input: &str) -> IResult<&str, (String, String)> {
    let (input, _) = space0(input)?;
    let (input, message) = parse_paranoid_warning(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, _) = space0(input)?;
    let (input, path) = parse_paranoid_path(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let mut path_buf = path.trim().to_string();
    let (input, continuations) = many0(parse_paranoid_continuation)(input)?;
    for extra in continuations {
        let extra_trim = extra.trim();
        // If the path already ends with this continuation fragment, skip to avoid duplication
        if path_buf.ends_with(extra_trim) {
            continue;
        }
        path_buf.push_str(extra_trim);
    }

    Ok((input, (normalize_paranoid_message(message), path_buf)))
}

fn parse_paranoid_block(input: &str) -> IResult<&str, (String, Vec<(String, String)>)> {
    let (input, risk) = parse_paranoid_header(input)?;
    let (input, _) = opt(line_ending)(input)?;
    let (input, entries) = many1(parse_paranoid_entry)(input)?;
    Ok((input, (risk.to_string(), entries)))
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
        eprintln!("[DEBUG] Processing line {}: '{}'", i, line);

        // Check for risk marker (normal or paranoid)
        if (line.contains("RISK") && line.contains(":")) || line.contains("RISK FINDINGS") {
            match risk_marker(line) {
                Ok((_, marker)) => {
                    current_risk = Some(extract_risk(marker));
                    eprintln!("[DEBUG] Set current_risk to {:?}", current_risk);
                }
                Err(e) => eprintln!(
                    "[DEBUG] risk_marker failed for line: '{}' error: {:?}",
                    line, e
                ),
            }
        }

        if line.contains("RISK (PARANOID):") {
            eprintln!("[DEBUG] Found PARANOID block at line {}", i);
            current_risk = None;
            let remaining = lines[i..].join("\n");
            if let Ok((rest, (risk_label, entries))) = parse_paranoid_block(&remaining) {
                let consumed_len = remaining.len().saturating_sub(rest.len());
                let consumed_lines = remaining[..consumed_len].lines().count().max(1);
                eprintln!("[DEBUG] Parsed {} paranoid entries", entries.len());
                for (message, path) in entries {
                    findings.push(Finding::new(&path, &message, &risk_label));
                }
                i += consumed_lines.saturating_sub(1);
                continue;
            } else {
                eprintln!("[DEBUG] Failed to parse paranoid block");
            }
        }

        if let Some(risk) = current_risk {
            eprintln!("[DEBUG] In risk block: {}", risk);
            // Try: "- Label: value" + "Found in: path"
            if let Ok((_, value)) = parse_label_value(line) {
                eprintln!("[DEBUG] Matched label_value: {}", value);
                if i + 1 < lines.len() {
                    if let Ok((_, path)) = parse_found_in(lines[i + 1].trim()) {
                        eprintln!("[DEBUG] Matched found_in: {}", path);
                        let mut path_string = path.to_string();
                        // Handle soft-wrapped path lines (terminal line wraps) by concatenating following lines
                        let mut look_ahead = i + 2;
                        while look_ahead < lines.len() {
                            let cont = lines[look_ahead].trim();
                            if cont.is_empty()
                                || cont.starts_with('-')
                                || cont.contains("RISK")
                                || cont.starts_with("NOTE:")
                                || cont.starts_with("Found in:")
                                || cont.starts_with("┌")
                                || cont.starts_with("│")
                                || cont.starts_with("└")
                                || cont.starts_with("Context:")
                            {
                                break;
                            }
                            // Heuristic: continuation fragments usually don't contain spaces before path tail
                            // Append directly (they are broken mid-path)
                            path_string.push_str(cont);
                            look_ahead += 1;
                        }
                        findings.push(Finding::new(&path_string, value, risk));
                        eprintln!(
                            "[DEBUG] Added finding: {} | {} | {}",
                            path_string, value, risk
                        );
                        i = look_ahead;
                        continue;
                    }
                }
            }

            // Try: "- /path:message"
            if let Ok((_, (path, message))) = parse_path_colon_message(line) {
                eprintln!("[DEBUG] Matched path_colon_message: {} : {}", path, message);
                findings.push(Finding::new(path, message, risk));
                i += 1;
                continue;
            }

            // Try: "- simple list item" (for LOW RISK FINDINGS)
            if line.trim().starts_with("- ") && risk == "LOW" {
                eprintln!("[DEBUG] Matched low risk list item");
                let item = line.trim().trim_start_matches("- ").trim();
                // For LOW RISK items, extract category prefix as file_path
                if let Some(colon_pos) = item.find(": ") {
                    let category = &item[..colon_pos];
                    let message = &item[colon_pos + 2..];
                    findings.push(Finding::new(category, message, risk));
                } else {
                    // Fallback: use full item as message with generic file_path
                    findings.push(Finding::new("low risk finding", item, risk));
                }
                i += 1;
                continue;
            }

            // Try: "- /path" (workflows)
            if let Ok((_, path)) = parse_simple_path(line) {
                eprintln!("[DEBUG] Matched simple_path: {}", path);
                if !path.contains(':') && path.starts_with('/') {
                    findings.push(Finding::new(
                        path,
                        "Known malicious workflow filename",
                        risk,
                    ));
                    eprintln!("[DEBUG] Added workflow finding");
                }
            }
        }

        i += 1;
    }

    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_paranoid_block_parses_entries() {
        let sample = "MEDIUM RISK (PARANOID): Potential typosquatting/homoglyph attacks detected:\n   - Warning: Potential typosquatting of 'lodash': lodsh (missing character)\n     Found in: /c/foo/package.json\npackage.json\n   - Warning: Potential Unicode/homoglyph characters in package: reаct\n     Found in: /c/foo/package.json\npackage.json\n";
        let (_, (risk, entries)) = parse_paranoid_block(sample).expect("block should parse");
        assert_eq!(risk, "MEDIUM");
        assert_eq!(entries.len(), 2);
        assert!(entries[0].1.ends_with("/package.json") || entries[0].1.ends_with("package.json"));
    }
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
        "trufflehog_activity",
        "git_branches",
        "postinstall_hooks",
        "shai_hulud_repos",
        "namespace_warnings", // Restored - now properly filtered at source
        "integrity_issues",
        "typosquatting_warnings",
        "network_exfiltration_warnings",
        "lockfile_safe_versions",
    ];

    findings.extend(
        categories
            .iter()
            .filter_map(|category| data.get(category).and_then(|v| v.as_array()))
            .flat_map(|items| {
                items.iter().filter_map(|item| {
                    match (
                        item.get("file_path").and_then(|v| v.as_str()),
                        item.get("message").and_then(|v| v.as_str()),
                        item.get("risk_level").and_then(|v| v.as_str()),
                    ) {
                        (Some(path), Some(msg), Some(risk)) => Some(Finding::new(path, msg, risk)),
                        _ => None,
                    }
                })
            }),
    );

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

    // BASH COMPATIBILITY: Use Vec-based matching to preserve duplicates (like Bash arrays)
    let bash_fingerprints: Vec<_> = bash_findings.iter().map(|f| f.fingerprint()).collect();
    let rust_fingerprints: Vec<_> = rust_findings.iter().map(|f| f.fingerprint()).collect();

    // Count exact 1:1 matches including duplicates
    let mut matches = 0;
    let mut bash_matched = vec![false; bash_fingerprints.len()];
    let mut rust_matched = vec![false; rust_fingerprints.len()];

    // Match each bash finding with first available rust finding
    for (bash_idx, bash_fp) in bash_fingerprints.iter().enumerate() {
        if bash_matched[bash_idx] {
            continue;
        }
        for (rust_idx, rust_fp) in rust_fingerprints.iter().enumerate() {
            if rust_matched[rust_idx] {
                continue;
            }
            if bash_fp == rust_fp {
                matches += 1;
                bash_matched[bash_idx] = true;
                rust_matched[rust_idx] = true;
                break;
            }
        }
    }

    let missing = bash_matched.iter().filter(|&&matched| !matched).count();
    let extra = rust_matched.iter().filter(|&&matched| !matched).count();

    let unmatched_bash: Vec<String> = bash_fingerprints
        .iter()
        .enumerate()
        .filter(|(idx, _)| !bash_matched[*idx])
        .map(|(_, fp)| fp.clone())
        .collect();
    let unmatched_rust: Vec<String> = rust_fingerprints
        .iter()
        .enumerate()
        .filter(|(idx, _)| !rust_matched[*idx])
        .map(|(_, fp)| fp.clone())
        .collect();

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

    if missing == 0
        && extra == 0
        && matches == bash_findings.len()
        && matches == rust_findings.len()
    {
        println!("{}", "🎉 PERFECT MATCH!".bright_green().bold());
        std::process::exit(0);
    } else {
        println!("{}", "⚠️  Differences detected".bright_yellow());

        // Special case: same count but different fingerprints (partial matches)
        if missing == 0
            && extra == 0
            && (matches != bash_findings.len() || matches != rust_findings.len())
        {
            println!();
            println!(
                "{}",
                "📝 Partial fingerprint matches detected:"
                    .bright_yellow()
                    .bold()
            );
            println!("   Same finding count but different formatting/content");
            println!(
                "   {} matches out of {} total findings",
                matches,
                bash_findings.len()
            );

            // Show first few fingerprints for comparison
            println!();
            println!("{}", "First few Bash fingerprints:".bright_red());
            bash_findings.iter().take(3).for_each(|bf| {
                println!("  - {}", bf.fingerprint());
            });

            println!();
            println!("{}", "First few Rust fingerprints:".bright_cyan());
            rust_findings.iter().take(3).for_each(|rf| {
                println!("  + {}", rf.fingerprint());
            });
        }

        if missing > 0 {
            println!();
            println!("{}", "Missing in Rust:".bright_red().bold());
            // Show unmatched bash findings
            unmatched_bash.iter().take(5).for_each(|fp| {
                println!("  - {}", fp);
            });
            if missing > 5 {
                println!("  ... and {} more", missing - 5);
            }
        }

        if extra > 0 {
            println!();
            println!("{}", "Extra in Rust:".bright_cyan().bold());
            // Show unmatched rust findings
            unmatched_rust.iter().take(5).for_each(|fp| {
                println!("  + {}", fp);
            });
            if extra > 5 {
                println!("  ... and {} more", extra - 5);
            }
        }

        let webhook_mismatch = unmatched_bash
            .iter()
            .chain(unmatched_rust.iter())
            .any(|fp| fp.contains("webhook.site"));
        if webhook_mismatch {
            println!();
            println!(
                "{}",
                "NOTE: Bekannte Abweichung rund um webhook.site – siehe https://github.com/Cobenian/shai-hulud-detect/pull/50 (WIP)."
                    .bright_yellow()
            );
        }

        std::process::exit(1);
    }
}
