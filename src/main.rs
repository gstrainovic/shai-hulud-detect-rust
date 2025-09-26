use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

mod hash_checker;
mod output;
mod patterns;
mod scanner;

use output::{ScanResults, TestResults};
use scanner::Scanner;

/// Shai-Hulud NPM Supply Chain Attack Detector (Rust implementation)
#[derive(Parser)]
#[command(name = "shai-hulud-scanner")]
#[command(about = "A Rust implementation of the Shai-Hulud NPM supply chain attack detector")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to scan for indicators of compromise
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Enable paranoid mode with additional security checks
    #[arg(long)]
    paranoid: bool,

    /// Output results in JSON format
    #[arg(long, short)]
    json: bool,

    /// Output file for JSON results (default: scan_results.json)
    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Run in test mode using test-cases validation
    #[arg(long)]
    test: bool,

    /// Quiet mode - only show summary
    #[arg(long, short)]
    quiet: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize scanner
    let scanner = Scanner::new(&cli.path, cli.paranoid).await?;

    // Run the scan
    let results = scanner.scan().await?;

    // Determine exit code based on findings
    let exit_code = match results.high_risk_count() {
        0 => match results.medium_risk_count() {
            0 => 0, // Success - no issues found
            _ => 1, // Medium risk found - warning
        },
        _ => 2, // High risk found - failure
    };

    // Output results
    if cli.json || cli.output.is_some() {
        let output_file = cli
            .output
            .unwrap_or_else(|| PathBuf::from("scan_results.json"));
        results.save_json(&output_file)?;

        if !cli.quiet {
            println!("Results saved to: {}", output_file.display());
        }
    }

    if !cli.quiet {
        results.print_summary();
    }

    // Exit with appropriate code for CI/CD
    std::process::exit(exit_code);
}
