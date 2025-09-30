use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod e2e_tests;
mod hash_checker;
mod output;
mod pattern_registry;
mod pattern_table;
mod patterns;
mod scanner;
mod temp_file_manager;

use e2e_tests::E2ETestRunner;
use pattern_table::print_pattern_table;
use scanner::Scanner;

/// Shai-Hulud NPM Supply Chain Attack Detector (Rust implementation)
///
/// # Purpose
/// Comprehensive malware detection for npm supply chain attacks
/// Detects compromised packages, malicious files, and suspicious patterns
///
/// # Default Behavior
/// - Saves results to scan_results.json (use --no-json to disable)
/// - Shows progress and summary only (details available in JSON)
/// - Tracks scan timing and performance metrics
/// - Exits with appropriate codes for CI/CD integration
///
/// # Database
/// Maintains 604+ confirmed compromised package versions from September 2025 attacks
/// Auto-updates pattern detection based on latest threat intelligence
#[derive(Parser)]
#[command(name = "shai-hulud-scanner")]
#[command(about = "A Rust implementation of the Shai-Hulud NPM supply chain attack detector")]
#[command(version = "0.2.0")]
struct Cli {
    /// Path to scan for indicators of compromise
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

    /// Enable paranoid mode with additional security checks
    #[arg(long)]
    paranoid: bool,

    /// Disable JSON output (JSON is default)
    #[arg(long)]
    no_json: bool,

    /// Output file for JSON results (default: scan_results.json)
    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output file for console log (default: none, console only)
    #[arg(long, value_name = "FILE")]
    log_file: Option<PathBuf>,

    /// Run in test mode using test-cases validation
    #[arg(long)]
    test: bool,

    /// Show pattern mappings table
    #[arg(long)]
    show_patterns: bool,

    /// Run end-to-end tests against test_verification_detailed.json
    #[arg(long)]
    run_e2e_tests: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Show pattern mappings table if requested
    if cli.show_patterns {
        print_pattern_table()?;
        return Ok(());
    }

    // Run E2E tests if requested
    if cli.run_e2e_tests {
        println!("🧪 Running End-to-End tests against test_verification_detailed.json");
        let runner =
            E2ETestRunner::new("test_verification_detailed.json", "../shai-hulud-detect").await?;

        let results = runner.run_all_tests().await?;
        runner.print_test_summary(&results);

        let failed_count = results.iter().filter(|r| !r.passed).count();
        std::process::exit(if failed_count == 0 { 0 } else { 1 });
    }

    // Path is required for scanning
    let path = cli
        .path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Path is required for scanning"))?;

    // Initialize scanner
    let scanner = Scanner::new(path, cli.paranoid, true).await?; // Always show progress

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

    // Output results - JSON is now default
    if !cli.no_json {
        let output_file = cli.output.unwrap_or_else(|| {
            // Default: logs/rust/[folder_name]/scan_results.json
            let folder_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            PathBuf::from(format!("logs/rust/{}/scan_results.json", folder_name))
        });

        // Create directory if it doesn't exist
        if let Some(parent) = output_file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!(
                    "Warning: Failed to create output directory {}: {}",
                    parent.display(),
                    e
                );
            }
        }

        results.save_json(&output_file)?;
        println!("📄 Results saved to: {}", output_file.display());
    }

    // Always show only summary (details are in JSON)
    let log_file = cli.log_file.unwrap_or_else(|| {
        // Default: logs/rust/[folder_name]/console.log
        let folder_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        PathBuf::from(format!("logs/rust/{}/console.log", folder_name))
    });

    results.print_summary_to_file(Some(&log_file));

    // Exit with appropriate code for CI/CD
    std::process::exit(exit_code);
}
